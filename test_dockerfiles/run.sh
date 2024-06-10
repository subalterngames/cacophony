#! /bin/bash

TAG=cacophony
cd ../
docker build -f test_dockerfiles/$1/Dockerfile -t $TAG --build-arg DISTRO=$1 --build-arg VERSION=$2 .
docker run --rm --device /dev/snd $TAG