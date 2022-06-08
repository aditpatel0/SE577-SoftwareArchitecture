Comments and thoughts on Release4MD:

Security is such an important part of my day to day here at Comcast. I had some familarity with RSA tokens and CADA. We utilize this when we login into our VM servers. So I stayed awhile in the Outh space. This idea that we can secure the ability to read data is my biggest takeway in this stop. It eliminates the need to share passwords.


It really was fascnitating understanding how tokens are used to access data, and being a DBA my curosity led me to how this data is stored. As the trends move toward a more data concsious society, Outh can help secure personal information.

I do see a bit of a disadvange when it comes to Outh because if you are connected to one central hub, it leads to a high risk of getting hacked causing a snowball effect. Each stage should have its own authentication.




Requirements that must be present to run this application:

1. Node/Npm installed

2. Quesar CLI tool installed

3. Docker installed and running


Once above requirements are completed please do the following actions.

1. Execute the script called build.sh. When this executes it will start to build a container in docker for the backend of the web-service. It will also start to install the front end components.

Please run  $ ./build.sh


2. The next script run.sh will be the one to create the development server --> and open a web browser. Now you will be required to use personal access token before succesful run.

Please run $ ./run.sh <PLEASE_INSERT_TOKEN_HERE>

