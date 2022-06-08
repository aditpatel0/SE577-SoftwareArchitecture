
Lets talk architecture!

This application has two components to it

1. The front end web application this utilizes the VUE framework coupled with Quasar. It also utilizes Typescript.

2. The backend server witch utilized Rust (please see project-release-2.md to review why Rust was utilized and  a link to Rust documentation)


Requirements that must be present to run this application:

1. Node/Npm installed

2. Quesar CLI tool installed

3. Docker installed and running


Once above requirements are completed please do the following actions.

1. Execute the script called build.sh. When this executes it will start to build a container in docker for the backend of the web-service. It will also start to install the front end components.

Please run  $ ./build.sh


2. The next script run.sh will be the one to create the development server --> and open a web browser.

Please run $ ./run.sh

