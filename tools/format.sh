#!/bin/bash
# Formats and lints the Rust codebase
set -e

cargo fmt --all
cargo clippy --all -- -D warnings
