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

cp pkg/crt_3d_sim* www/resources/js/

if ${release} ; then
    wasm-opt -O3 -o www/resources/js/crt_3d_sim_bg.wasm www/resources/js/crt_3d_sim_bg.wasm
    pushd www
    npm install --dev
    npm run build
    cp *.css dist/
    cp favicon.ico dist/
    cp -r assets dist/
    popd
else
    if [ ! -d www/node_modules ]; then
        cd www
        npm install --dev
    fi
fi
