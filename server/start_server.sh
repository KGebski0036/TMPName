#!/bin/bash
docker build -t testmaster . 
mkdir -p ./docker_vol ; docker run -v ./docker_vol:/var/server/repo -p 8080:8080 testmaster