#!/usr/bin/env bash

set -euo pipefail

cd "$(dirname $0)/.."

if [[ "$@" != "--rust-only" ]] ; then
    if [ -f ~/.nvm/nvm.sh ] ; then
        source ~/.nvm/nvm.sh
        nvm use stable
    fi

    pushd www
    npm test
    popd
fi
cargo clippy --all
cargo test --all

