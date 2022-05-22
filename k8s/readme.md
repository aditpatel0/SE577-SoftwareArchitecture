# Kubernetes Setup

For this demo I will be using kind as my local kubernetes environment. If you want to use this as well, see the install directions at: https://kind.sigs.k8s.io/. You should also have all of the other kubernetes tooling such as `kubectl` installed. For newbees and for demos i also like Lens - https://k8slens.dev/

By default kind doesnt support ingress, and requires a few additional things on MacOS and Windows given the docker network is not exposed to the host.

The first thing that has to be done is to create a kind cluster - this will be the most basic cluster - one control plane and one worker note in the data plane.

`kind create cluster --config ./kind-cluster-config.yml`

You can create a default cluster without the `config` parameter but you will not be able to easily get requests into your clusters. If you are familiar with `port-forward` you can work around it. Anyhow the main thing the config file does is to setup some port mappings that will become helpful in the next step.

```
 extraPortMappings:
      - containerPort: 80
        hostPort: 80
        protocol: TCP
      - containerPort: 443
        hostPort: 443
        protocol: TCP
```

For now we are basically specifying that we will be exposing ports 80 and 443 to the host, and these will map to ports 80 and 443 in the containers.

The next step will install nginx as an ingress controller, and by default it will listen on ports 80 and 443 within the container. So you could adjust the `hostPort` in the config if you want to use alternative ports.

#### Nginx Ingress Controller

To install Nginx configured for Kind you can execute the following command

```
kubectl apply -f https://raw.githubusercontent.com/kubernetes/ingress-nginx/main/deploy/static/provider/kind/deploy.yaml
```

This might take a few min to get running. You can wait for it via this command:

```
kubectl wait --namespace ingress-nginx \
  --for=condition=ready pod \
  --selector=app.kubernetes.io/component=controller \
  --timeout=90s
```

You are now setup. The directions for this section came from here https://kind.sigs.k8s.io/docs/user/ingress/

### Using Local Docker?

If you are using docker locally, you need to load the image into kind so that you can deploy it. Kind, like normal kubernetes wants to pull containers from a registry. But since we have it local we will work that way.

```
kind load docker-image architecting-software/se577-demo-app
```

This is the container that guts built from the `make docker-server` command

### Time to deploy

Run

```
kubectl apply -f se577-client-deploy.yml
```

should output:

```
deployment.apps/se577-ui-deployment created
ingress.networking.k8s.io/se577-ui-ingress created
service/se577-web-ui-svc created
```

This does several things.

1. It creates a deployment, that has a replica set to scale up and down the number of pods to server requests - this is for scale and resilience
2. it creates a service, that internally allows for load balancing across the deployment cluster
3. It configures the ingress, so that you can reach the application from outside of the cluster.

To test that everything worked, go to

```
http://localhost/ui/
```

In your browser - the demo app should render
