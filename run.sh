#!/usr/bin/env bash

set -xeuo pipefail

cd "$(dirname $0)"

wasm-pack build

pushd pkg
npm link
popd

cd www
npm install
npm link wasm-game-of-life
npm run start
