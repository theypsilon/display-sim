#!/usr/bin/env bash

set -euo pipefail

cd "$(dirname $0)"

echo

release=false
build_type="--debug"
if [[ "$@" == "--release" ]]; then
    release=true
    build_type=""
    echo "  *** RELEASE BUILD ***"
else
    echo "  *** DEBUG BUILD ***"
fi
wasm-pack build ${build_type}

cp pkg/crt_3d_sim* www/

if ${release} ; then
    wasm-opt -O3 -o www/crt_3d_sim_bg.wasm www/crt_3d_sim_bg.wasm
    cd www
    npm install --dev
    npm run build
    cp worker.js dist/
    cp style.css dist/
    cp favicon.ico dist/
    cp -r assets dist/
else
    if [ ! -d www/node_modules ]; then
        cd www
        npm install --dev
    fi
fi
