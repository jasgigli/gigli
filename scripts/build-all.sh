#!/bin/bash
# Builds all components of GigliOptix
set -e

./tools/install-deps.sh
cargo build --release
