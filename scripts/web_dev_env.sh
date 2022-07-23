#!/usr/bin/env bash

set -euo pipefail

cd "$(dirname $0)/.."

TMUX_WINDOW_INDEX="$(tmux display-message -p '#I')"
trap "tmux kill-pane -a -t ${TMUX_WINDOW_INDEX} 2> /dev/null || true" SIGINT SIGTERM EXIT

tmux split -t "${TMUX_WINDOW_INDEX}" -h 'cd www; npm run start'

rm .build-hash || true
cargo watch -s "echo; $(pwd)/scripts/build.sh --dev-server" -w 'rust/' -w 'Cargo.toml'
