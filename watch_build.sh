#!/usr/bin/env bash

set -euo pipefail

cd "$(dirname $0)"

./build.sh
watchman-make -p 'src/**/*.rs' 'Cargo.toml' --make=$(pwd)/build.sh -t build