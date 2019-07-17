#!/usr/bin/env bash

set -euo pipefail

cd "$(dirname $0)/.."

./scripts/build.sh || true

echo "Setting watcher..."
echo

watchexec --exts rs,toml "$(pwd)/scripts/build.sh --dev-server"
#watchman-make -p 'rust/**/*.rs' 'Cargo.toml' 'rust/**/*.toml' --make="echo; $(pwd)/scripts/build.sh --dev-server" -t build
