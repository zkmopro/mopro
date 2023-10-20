#!/bin/bash

PROJECT_DIR=$(pwd)

# build circom circuits in mopro-core
cd ${PROJECT_DIR}/mopro-core/examples/circom/keccak256
npm install
./compile.sh

# build ffi in mopro-ffi
cd ${PROJECT_DIR}/mopro-ffi
rustup target add x86_64-apple-ios aarch64-apple-ios aarch64-apple-ios-sim
cargo install --bin uniffi-bindgen --path .
