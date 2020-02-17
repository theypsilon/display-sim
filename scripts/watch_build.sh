#!/usr/bin/env bash

set -euo pipefail

cd "$(dirname $0)/.."

./scripts/build.sh || true

echo "Setting watcher..."
echo

cargo watch -s "echo; $(pwd)/scripts/build.sh --dev-server" -w 'rust/' -w 'Cargo.toml'
