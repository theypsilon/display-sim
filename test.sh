#!/usr/bin/env bash

set -euo pipefail

if [ -f ~/.nvm/nvm.sh ] ; then
    source ~/.nvm/nvm.sh
    nvm use stable
fi

pushd www
npm test
popd
cargo clippy
cargo test -p screen-sim-core -p screen-sim-web-error -p screen-sim-web-render -p screen-sim-webgl-stubs -p screen-sim-webgl-to-sdl2 -p screen-sim-benchmarks

