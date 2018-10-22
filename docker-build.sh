#!/usr/bin/env bash

set -euo pipefail

docker build -t crt-3d-sim:0.0.0 .

if [[ $@ =~ .*--git-docs.* ]] ; then
    if git diff-index --quiet HEAD -- ; then
        rm -rf docs || true && mkdir -p docs
        docker run --rm -v $(pwd)/docs:/tmp crt-3d-sim:0.0.0 sh -c "cp -r /var/www/html/* /tmp && chown -R $UID:$UID /tmp"
        git add docs
    else
        echo "Can't add docs because some other changes are not commited." >2
        exit -1
    fi
fi

if [[ $@ =~ .*--serve.* ]] ; then
    echo Server running on port 80...
    docker run --rm --name crt-3d-sim-server -p 80:80 crt-3d-sim:0.0.0
fi