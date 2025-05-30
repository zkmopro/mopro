# Noir Adapter

Mopro supports the integration of [Noir](https://noir-lang.org/) circuits, enabling zero-knowledge proofs in native mobile applications with ease. This adapter builds upon foundational work from the zkPassport team and streamlines cross-platform integration with performance, reliability, and developer experience in mind.

## Samples

You can explore real examples of how the Noir adapter works in these projects:

- [mopro-zkemail-nr](https://github.com/zkmopro/mopro-zkemail-nr)
- [stealthnote-mobile](https://github.com/vivianjeng/stealthnote-mobile)
- [test-e2e](https://github.com/zkmopro/mopro/tree/main/test-e2e)

## Setting Up the Rust Project

To get started, follow the [Rust Setup Guide](/setup/rust-setup.md) and activate the `noir` feature in your `Cargo.toml`:

```toml
[features]
default = ["mopro-ffi/noir"]
```

You should also depend on `noir-rs`, our Rust binding crate built around the Noir + Barretenberg backend:

```toml
[dependencies]
noir-rs = { git = "https://github.com/zkmopro/noir-rs" }
```

This crate automatically fetches precompiled `libbarretenberg.a` binaries, avoiding the need for local compilation.

## Proving and Verifying Functions

### Proving Function

The proving function is responsible for loading the circuit, preparing the SRS, converting the inputs into a witness map, and generating the proof using the Barretenberg backend.

```rust
pub fn generate_noir_proof(
    circuit_path: String,
    srs_path: Option<String>,
    inputs: Vec<String>,
) -> Result<Vec<u8>, String>;
```

- `circuit_path`: Path to the compiled Noir .acir circuit.
- `srs_path`: Optional path to the structured reference string.
- `inputs`: A list of strings representing public/private inputs.
- Returns a serialized proof (Vec<u8>).

### Verifying Function

The verification function loads the circuit and derives the verification key, then verifies the proof.

```rust
pub fn verify_noir_proof(circuit_path: String, proof: Vec<u8>) -> bool;
```

- `circuit_path`: Path to the compiled Noir .acir circuit.
- `proof`: The serialized proof to verify.
- Returns `true` if the proof is valid.

## Using the Library

### iOS API

```swift
// Generate a proof for a given circuit file and srs file, as well as the circuit inputs.
generateNoirProof(circuitPath: circuitPath, srsPath: srsPath, inputs: inputs)

// Verify a proof for a given circuit file, as well as the proof.
verifyNoirProof(circuitPath: circuitPath, proof: proofData)
```

### Android

The Noir adapter exposes the equivalent functions and types to be used in the Android project.

## Platform Support

| Platform         | Target Triple              | Status |
| ---------------- | -------------------------- | ------ |
| iOS Device       | `aarch64-apple-ios`        | ✅     |
| iOS Simulator    | `x86_64-apple-ios`         | ✅     |
| Android Device   | `aarch64-linux-android`    | ✅     |
| Android Emulator | `x86_64-linux-android`     | ✅     |
| macOS (M1/M2)    | `aarch64-apple-darwin`     | ✅     |
| Linux Desktop    | `x86_64-unknown-linux-gnu` | ✅     |
