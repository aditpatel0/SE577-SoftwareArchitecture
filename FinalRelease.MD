

Requirements that must be present to run this application:

1. Node/Npm installed

2. Quesar CLI tool installed

3. Docker installed and running


Once above requirements are completed please do the following actions.

1. Execute the script called build.sh. When this executes it will start to build a container in docker for the backend of the web-service. It will also start to install the front end components.

Please run  $ ./build.sh


2. The next script run.sh will be the one to create the development server --> and open a web browser. Now you will be required to use personal access token before succesful run.

Please run $ ./run.sh <PLEASE_INSERT_TOKEN_HERE>

Architecture Model:
Please view png called arcitecture model.

Decisions that were made:
Authentication using personal access token due to the simplicity of this application a more involved OAuth was not required.
RUST is being utilized (for deeper dive into my thought process read Projectrelease3) This helps control the APIS in this the application.

Attributes of this application include:
This application is contained with its seperate front-end/backend set up.
This application communicates with github gists api.
It will allow users to view an individual's gists.
It also allows uers to list branches of the respository.

