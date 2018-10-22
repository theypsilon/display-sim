#!/usr/bin/env bash

set -euo pipefail

docker build -t crt-3d-sim:0.0.0 .
docker run --rm --name crt-3d-sim-server -p 80:80 crt-3d-sim:0.0.0