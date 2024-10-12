#!/bin/bash
docker build -t testownik-ng-server . 
mkdir ./docker_vol ; docker run -v ./docker_vol:/var/server/repo -p 8080:8080 testownik-ng-server