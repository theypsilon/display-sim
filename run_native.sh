#!/usr/bin/env sh

set -euo pipefail

cd "$(dirname $0)"

cargo test --all
cargo run
