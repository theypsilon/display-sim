#!/usr/bin/env bash

set -euo pipefail

cd "$(dirname $0)"

tmux split -v 'source ~/.nvm/nvm.sh; nvm use stable; cd www; npm run start'
./watch_build.sh
