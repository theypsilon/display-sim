#!/usr/bin/env bash

set -euo pipefail

docker build -t crt-3d-sim:0.0.0 .

if [[ $@ =~ .*--extract-dist.* ]] ; then
    rm -rf dist || true && mkdir -p dist
    docker run --rm -v $(pwd)/dist:/tmp crt-3d-sim:0.0.0 sh -c "cp -r /var/www/html/* /tmp && chown -R $UID:$UID /tmp"
fi

if [[ $@ =~ .*--serve.* ]] ; then
    echo Server running on port 80...
    docker run --rm --name crt-3d-sim-server -p 80:80 -p 443:443 crt-3d-sim:0.0.0
fi