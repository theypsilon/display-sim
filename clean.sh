#!/usr/bin/env bash

set -xeuo pipefail

cd "$(dirname $0)"

cargo clean
rm -rf target || true
rm -rf www/src/wasm || true
rm -rf www/dist || true
rm -rf www/node_modules || true
