#!/usr/bin/env bash

set -xeuo pipefail

cd "$(dirname $0)"

rm -rf pkg || true
rm -rf www/node_modules || true