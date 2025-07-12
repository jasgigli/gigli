#!/bin/bash
# Installs Rust toolchain, LLVM, wasm target, and tree-sitter for GigliOptix development
set -e

# Install Rust
if ! command -v rustup &> /dev/null; then
    echo "Installing rustup..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
fi

# Install LLVM (assumes Ubuntu/WSL)
sudo apt-get update
sudo apt-get install -y llvm clang

# Add wasm target for Rust
rustup target add wasm32-unknown-unknown

# Install tree-sitter CLI
if ! command -v tree-sitter &> /dev/null; then
    npm install -g tree-sitter-cli
fi

echo "All dependencies installed."
