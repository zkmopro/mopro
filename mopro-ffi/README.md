# mopro-ffi

Mopro is a toolkit for ZK app development on mobile. Mopro makes client-side proving on mobile simple.

## Getting started

- Make sure you've installed the [prerequisites](https://zkmopro.org/docs/prerequisites).
- Getting started with this [tutorial](https://zkmopro.org/docs/getting-started/rust-setup).

## Run tests

- circom
  ```sh
  cargo test --features circom
  ```
- halo2
  ```sh
  cargo test --features halo2
  ```

## Bindings

- `SwiftBindings`
- `KotlinBindings`

The uniffi bindings are precompiled and committed here for a specifically named crate. This avoids the complexity of building/invoking the uniffi cli by dependent packages. Note that dependent crates _must_ have the library name `mopro_bindings`, or rebuild the binding themselves.

## Modules

The root module exports functions for generating proofs. It also exports a macro that can be used to setup uniffi from our provided udl file. User modification to the UDL file is not supported at this time.

### `circom`

Includes all proving and serialization logic for circom proofs. Does _not_ include logic for witness generation.

### `halo2`

Includes all proving logic for halo2. 

## Community
- Telegram: https://t.me/zkmopro
- X: https://x.com/zkmopro