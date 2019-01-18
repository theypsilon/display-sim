#!/usr/bin/env bash

set -euo pipefail

cd "$(dirname $0)"

./build.sh
cd www
npm install
npm run start
