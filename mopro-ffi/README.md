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

- X account: <a href="https://twitter.com/zkmopro"><img src="https://img.shields.io/twitter/follow/zkmopro?style=flat-square&logo=x&label=zkmopro"></a>
- Telegram group: <a href="https://t.me/zkmopro"><img src="https://img.shields.io/badge/telegram-@zkmopro-blue.svg?style=flat-square&logo=telegram"></a>

## Acknowledgements

This work was initially sponsored by a joint grant from [PSE](https://pse.dev/) and [0xPARC](https://0xparc.org/). It is currently incubated by PSE.