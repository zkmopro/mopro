#!/bin/sh

set -e

# This script only needs to be run when we change the UDL file
# The resulting output should be committed to the repo

cargo run --bin uniffi-bindgen generate src/mopro.udl --language swift --out-dir SwiftBindings

cargo run --bin uniffi-bindgen generate src/mopro.udl --language kotlin --out-dir KotlinBindings