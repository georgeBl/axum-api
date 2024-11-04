To start use the following commands:

docker build -t axum-app .
docker run -net=host -d --name axum-cont axum-app

It should start the app on localhost port:8080



