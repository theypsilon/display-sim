#!/usr/bin/env bash

set -euo pipefail

source ~/.nvm/nvm.sh
nvm use stable
pushd www
npm test
popd
cargo clippy
cargo test
