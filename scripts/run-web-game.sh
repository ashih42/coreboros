#!/usr/bin/env bash

# Note: Run this script at project root as:
# ./scripts/run-web-game.sh

# Exit this script immediately if any command fails.
set -e

# Print all commands before executing them.
set -x

# Make a release build for web assembly.
cargo build --release --bin game --target wasm32-unknown-unknown

# Make a symbolic link (shortcut) to the compiled wasm file.
# This shortcut is used in `index.html`.
ln -sf target/wasm32-unknown-unknown/release/game.wasm ./game.wasm

# Run the web server.
basic-http-server .
