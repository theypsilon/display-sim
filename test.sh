#!/usr/bin/env bash

set -euo pipefail

if [ -f ~/.nvm/nvm.sh ] ; then
    source ~/.nvm/nvm.sh
    nvm use stable
fi

pushd www
npm test
popd
cargo clippy --all
cargo test --all

