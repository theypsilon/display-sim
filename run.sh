#!/usr/bin/env bash

set -xeuo pipefail

cd "$(dirname $0)"

cd www
npm run start
