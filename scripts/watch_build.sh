#!/usr/bin/env bash

set -euo pipefail

cd "$(dirname $0)/.."

./scripts/build.sh || true

echo "Setting watcher..."
echo

watchman-make -p 'src/**/*.rs' 'Cargo.toml' 'crates/**/*.rs' 'crates/**/*.toml' --make="echo; $(pwd)/scripts/build.sh --dev-server" -t build
