# mopro-ffi

This is the main/only crate that is used to build mopro apps.

## `SwiftBindings`

The uniffi bindings are precompiled and committed here for a specifically named crate. This avoids the complexity of building/invoking the uniffi cli by dependent packages. Note that dependent crates _must_ have the library name `mopro_bindings`, or rebuild the binding themselves.

## Modules

The root module exports functions for generating proofs. It also exports a macro that can be used to setup uniffi from our provided udl file. User modification to the UDL file is not supported at this time.

### `circom`

Includes all proving and serialization logic for circom proofs. Does _not_ include logic for witness generation.

### `halo2`

Includes proving logic for halo2. Implementation is currently stubbed.