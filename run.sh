#!/usr/bin/env bash

set -euo pipefail

cd "$(dirname $0)"

./scripts/build.sh
cd www
npm install
npm run start
