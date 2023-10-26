#!/bin/bash

# Deal with errors
set -euo pipefail

PROJECT_DIR=$(pwd)
CIRCOM_DIR="${PROJECT_DIR}/mopro-core/examples/circom"

compile_circuit() {
    local circuit_dir=$1
    local circuit_file=$2
    local target_file="$circuit_dir/target/$(basename $circuit_file .circom).r1cs"

    echo "Compiling $circuit_file example circuit..."
    if [ ! -f "$target_file" ]; then
        ./scripts/compile.sh $circuit_dir $circuit_file
    else
        echo "File $target_file already exists, skipping compilation."
    fi
}

# Build Circom circuits in mopro-core and run trusted setup
echo "mopro-core: Compiling example circuits..."
cd $CIRCOM_DIR

# Compile multiplier2
compile_circuit multiplier2 multiplier2.circom

# Setup and compile keccak256
(cd keccak256 && npm install)
compile_circuit keccak256 keccak256_256_test.circom

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