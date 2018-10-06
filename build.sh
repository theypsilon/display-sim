#!/usr/bin/env bash

set -xeuo pipefail

cd "$(dirname $0)"

wasm-pack build --debug

if [ ! -d www/node_modules ]; then
    pushd pkg
    npm link
    popd

    cd www
    npm install
    npm link wasm-game-of-life
fi

