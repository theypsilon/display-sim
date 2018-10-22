#!/usr/bin/env bash

set -euo pipefail

docker build -t crt-3d-sim:0.0.0 .

if [[ $@ =~ .*--docs.* ]] ; then
    rm -rf docs || true && mkdir -p docs
    docker run --rm -v $(pwd)/docs:/tmp crt-3d-sim:0.0.0 sh -c "cp -r /var/www/html/* /tmp && chown -R $UID:$UID /tmp"
    git add docs
fi

if [[ $@ =~ .*--serve.* ]] ; then
    docker run --rm --name crt-3d-sim-server -p 80:80 crt-3d-sim:0.0.0
fi