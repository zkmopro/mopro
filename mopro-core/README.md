# mopro-core

Core mobile Rust library, exposes UniFFI bindings to be used by `rust-ios`, etc.

## Overview

TBD.

## Examples

Run `cargo run --example circom`.

## UniFFI bindings

1. Install the `uniffi-bindgen` util locally:

`cargo install --bin uniffi-bindgen --path .`

2. Generate Swift Bindings:

`uniffi-bindgen generate src/mopro_uniffi.udl --language swift --out-dir target/SwiftBindings`