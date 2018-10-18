#!/usr/bin/env bash

set -euo pipefail

cd "$(dirname $0)"

tmux split -v 'source ~/.nvm/nvm.sh && nvm use stable && ./run.sh'
./watch_build.sh