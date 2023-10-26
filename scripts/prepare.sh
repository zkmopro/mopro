#!/bin/bash

# Deal with errors
set -euo pipefail

PROJECT_DIR=$(pwd)
CIRCOM_DIR="${PROJECT_DIR}/mopro-core/examples/circom"

# Build Circom circuits in mopro-core and run trusted setup
echo "mopro-core: Compiling example circuits..."
cd $CIRCOM_DIR

# NOTE: multiplier2 is already pre-compiled
# echo "Compiling multiplier example circuit..."
# ./scripts/compile.sh multiplier2 multiplier2.circom

# Setup and compile keccak256
echo "Compiling keccak256 example circuit..."
(cd keccak256 && npm install)
./scripts/compile.sh keccak256 keccak256_256_test.circom

# TODO: Finish trusted setup script
#echo "mopro-core: Running trusted setup for keccak256..."
#./scripts/trusted_setup.sh

# Add support for target architectures
echo "mopro-ffi: Adding support for target architectures..."
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