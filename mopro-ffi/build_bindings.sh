#!/bin/sh

set -e

cargo run --bin uniffi-bindgen generate src/mopro.udl --language swift --out-dir SwiftBindings
