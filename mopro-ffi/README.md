# mopro-ffi

Thin wrapper around `mopro-core`, exposes UniFFI bindings to be used by `rust-ios`, etc.

## Overview

TBD.

## UniFFI bindings

1. Install the `uniffi-bindgen` util locally:

`cargo install --bin uniffi-bindgen --path .`

2. Generate Swift Bindings:

`uniffi-bindgen generate src/mopro_uniffi.udl --language swift --out-dir target/SwiftBindings`