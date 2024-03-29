#!/usr/bin/env bash

set -euo pipefail

cd "$(dirname $0)/.."

export REGISTRY_IMAGE="${REGISTRY_IMAGE:-display-sim}"
export TAG="${TAG:-latest}"
export IMAGE_NAME="${REGISTRY_IMAGE}:${TAG}"

DOCKER_SCAN_SUGGEST=false docker build --build-arg BUILD_WASM_PARAMS="--release-wasm" -t ${IMAGE_NAME} .

if [[ $@ =~ .*--extract-dist.* ]] ; then
    rm -rf dist || true && mkdir -p dist
    docker run --rm -v $(pwd)/dist:/tmp ${IMAGE_NAME} sh -c "cp -r /var/www/html/* /tmp && chown -R $UID:$UID /tmp"
fi

if [[ $@ =~ .*--serve.* ]] ; then
    echo Server running on port 80...
    docker run --rm --name display-sim-server -p 80:80 -p 443:443 ${IMAGE_NAME}
fi
