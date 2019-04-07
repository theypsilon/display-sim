#!/usr/bin/env bash

set -euo pipefail

export REGISTRY_IMAGE="${REGISTRY_IMAGE:-crt-3d-sim}"
export TAG="${TAG:-latest}"
export IMAGE_NAME="${REGISTRY_IMAGE}:${TAG}"

docker build -t ${IMAGE_NAME} .

if [[ $@ =~ .*--extract-dist.* ]] ; then
    rm -rf dist || true && mkdir -p dist
    docker run --rm -v $(pwd)/dist:/tmp ${IMAGE_NAME} sh -c "cp -r /var/www/html/* /tmp && chown -R $UID:$UID /tmp"
fi

if [[ $@ =~ .*--serve.* ]] ; then
    echo Server running on port 80...
    docker run --rm --name crt-3d-sim-server -p 80:80 -p 443:443 ${IMAGE_NAME}
fi