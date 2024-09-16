#!/bin/bash

set -e

# this block ensures we can invoke this script from anywhere and have it automatically change to this folder first
pushd "$(dirname -- "${BASH_SOURCE[0]}")" >/dev/null 2>&1
function teardown() {
    popd >/dev/null 2>&1 || true
}
trap teardown exit

# we need docker to build the images
if ! command -v docker >/dev/null 2>&1; then
    echo "error: docker not found"
    exit 1
fi

# TODO: hack workaround for intermittent builds on darwin aarch64
mkdir -p ./tmp
GOOS=linux GOARCH=amd64 CGO_ENABLED=0 go build -o ./tmp/api -trimpath ./cmd/api
GOOS=linux GOARCH=amd64 CGO_ENABLED=0 go build -o ./tmp/segment-producer -trimpath ./cmd/segment_producer
GOOS=linux GOARCH=amd64 CGO_ENABLED=0 go build -o ./tmp/stream-producer -trimpath ./cmd/stream_producer

docker build --platform=linux/amd64 -t kube-registry:5000/camry-api:latest -f ./docker/api/Dockerfile .
docker build --platform=linux/amd64 -t kube-registry:5000/camry-segment-producer:latest -f ./docker/segment-producer/Dockerfile .
docker build --platform=linux/amd64 -t kube-registry:5000/camry-stream-producer:latest -f ./docker/stream-producer/Dockerfile .
docker build --platform=linux/amd64 -t kube-registry:5000/camry-object-detector:latest -f ./docker/object-detector/Dockerfile --build-arg BASE_IMAGE=pytorch/pytorch:2.4.0-cuda11.8-cudnn9-runtime .
# docker build --platform=linux/amd64 -t kube-registry:5000/camry-object-detector:sm30 -f ./docker/object-detector/Dockerfile --build-arg BASE_IMAGE=dizcza/pytorch-sm30:v1.10.2 .
docker build --platform=linux/amd64 -t kube-registry:5000/camry-frontend:latest -f ./docker/frontend/Dockerfile .

docker image push kube-registry:5000/camry-api:latest
docker image push kube-registry:5000/camry-segment-producer:latest
docker image push kube-registry:5000/camry-stream-producer:latest
docker image push kube-registry:5000/camry-object-detector:latest
# docker image push kube-registry:5000/camry-object-detector:sm30
docker image push kube-registry:5000/camry-frontend:latest
