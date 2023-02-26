#!/bin/bash

if [ $# != 1 ]; then
    echo "Wrong number of arg"
    echo "./docker-operation.sh run or remove"
    exit 1    
fi

ARG=$1

if [ $ARG = "run" ]; then
    docker build -t socket_rs .
    docker run -it --name socket_rs socket_rs:latest
elif [ $ARG = "remove" ]; then
    docker rm socket_rs
    docker rmi socket_rs:latest
fi
