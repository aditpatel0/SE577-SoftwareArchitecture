#!/bin/bash

docker run -p 3000:3000 --rm --name Architecture-server1 Architecture-server-server &

cd frontend

quasar dev &

cd ../
