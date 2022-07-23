#!/usr/bin/env bash

set -euo pipefail

cd "$(dirname $0)/.."

if [[ "$@" != "--rust-only" ]] ; then
    pushd www
    npm test
    npm run lint
    popd
fi

cargo clippy --all
cargo test --all
cargo bench --all

