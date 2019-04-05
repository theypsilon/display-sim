#!/usr/bin/env bash

set -euo pipefail

cd "$(dirname $0)"

echo

cargo fmt

new_hash="$(find src/ -type f -name *.rs -print0 | xargs -0 sha1sum | sha1sum)"
old_hash="$(cat .build-hash || true)"

if [[ "$new_hash" == "$old_hash" ]] ; then exit -1; fi

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

cp pkg/screen_sim* www/

if ${release} ; then
    wasm-opt -O3 -o www/screen_sim_bg.wasm www/screen_sim_bg.wasm
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

echo "$new_hash" > .build-hash
