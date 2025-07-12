#!/bin/bash
# Runs GigliOptix in development mode with hot reload
set -e

cargo install cargo-watch || true
cargo watch -x 'run examples/hello.gx'
