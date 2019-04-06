#!/usr/bin/env bash

set -euo pipefail

cd "$(dirname $0)"

optimize_wasm=false
npm_build=false
dev_watch=false
npm_deps=false
build_type="--debug"
if [[ "$@" =~ "--release" ]]; then
    echo -n "[RELEASE BUILD] "
    optimize_wasm=true
    npm_deps=true
    npm_build=true
    build_type=""

    cargo clean
    rm -rf www/node_modules || true
elif [[ "$@" =~ "--release-wasm" ]] ; then
    echo -n "[RELEASE WASM BUILD] "
    optimize_wasm=true
    build_type=""
elif [[ "$@" =~ "--dev-server" ]] ; then
    echo -n "[DEV SERVER BUILD] "
    dev_watch=true
    npm_deps=true

    cargo fmt

    new_hash="$(find src/ -type f -name *.rs -print0 | xargs -0 sha1sum | sha1sum)"
    old_hash="$(cat .build-hash || true)"

    if [[ "$new_hash" == "$old_hash" ]] ; then
        echo "... nothing to do"
        echo
        exit 0
    fi
else
    echo -n "[DEBUG BUILD] "
    npm_deps=true
fi

rm -rf www/src/wasm || true

echo "wasm-pack buld ${build_type}:"
wasm-pack build ${build_type} --out-dir www/src/wasm

if ${optimize_wasm} ; then
    pushd www/src/wasm
    wasm-opt -O3 -o screen_sim_bg.wasm screen_sim_bg.wasm
    popd
fi

if ${npm_deps} && [ ! -d www/node_modules ]; then
    pushd www
    npm install --dev
    popd
fi

if ${npm_build} ; then
    pushd www
    npm run build
    popd
fi

if ${dev_watch} ; then
    echo "$new_hash" > .build-hash
fi

echo