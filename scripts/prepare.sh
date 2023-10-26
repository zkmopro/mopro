#!/bin/bash

# Deal with errors
set -euo pipefail

PROJECT_DIR=$(pwd)

# build circom circuits in mopro-core
cd ${PROJECT_DIR}/mopro-core/examples/circom/keccak256
npm install
./compile.sh

# build ffi in mopro-ffi
cd ${PROJECT_DIR}/mopro-ffi
rustup target add x86_64-apple-ios aarch64-apple-ios aarch64-apple-ios-sim

# Install uniffi-bindgen binary in mopro-ffi
echo "mopro-ffi: Installing uniffi-bindgen..."
if ! command -v uniffi-bindgen &> /dev/null
then
    echo "mopro-ffi: Installing uniffi-bindgen..."
    cargo install --bin uniffi-bindgen --path .
else
    echo "mopro-ffi: uniffi-bindgen already installed, skipping."
fi