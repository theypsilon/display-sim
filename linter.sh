#!/usr/bin/env bash

set -euo pipefail

cargo clippy -- -A clippy::float_cmp -A clippy::identity_op -W clippy::useless_attribute
