# Ashlang Adapter

Mopro supports the integration of [Ashlang](https://github.com/chancehudson/ashlang) circuits. Ashlang is a scripting language for expressing mathematical relations between scalars and vectors in a finite field. It supports proving on the following systems:

-   [tritonvm/triton-vm](https://github.com/tritonvm/triton-vm): using `tasm` target in this crate
-   [microsoft/spartan](https://github.com/microsoft/spartan): using `ar1cs` target in [chancehudson/ashlang-spartan](https://github.com/chancehudson/ashlang-spartan)

## Setup the rust project

You can follow the instructions in the [Rust Setup](/setup/rust-setup.md) guide to create a new Rust project that builds this library with Ashlang proofs.

In your `Cargo.toml` file, ensure the `ashlang` feature is activated for `mopro-ffi`:

```toml
[features]
default = ["mopro-ffi/ashlang"]
```

:::warning
**Note:** The current Ashlang adapter only supports the Spartan prover.
:::

<!-- TODO: how to compile circuits -->

## Using the Library

After you have specified the circuits you want to use, you can follow the usual steps to build the library and use it in your project.

### iOS API

The Ashlang adapter exposes the following functions to be used in the iOS project:

```swift
// Generate a proof for a given circuit ar1cs, as well as the circuit inputs
generateAshlangSpartanProof(ar1csPath: ar1csPath, inputs: inputs) -> GenerateProofResult

// Verify a proof for a given circuit ar1cs
verifyAshlangSpartanProof(ar1csPath: ar1csPath, proof: generateProofResult.proof) -> Bool
```

### Android API

The Ashlang adapter exposes the equivalent functions and types to be used in the Android project.
