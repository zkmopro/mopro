# Noir Adapter

Mopro supports the integration of [Noir](https://noir-lang.org/) circuits, enabling zero-knowledge proofs in native mobile applications with ease. This adapter builds upon foundational work from the zkPassport team and streamlines cross-platform integration with performance, reliability, and developer experience in mind.

## Samples

You can explore real examples of how the Noir adapter works in these projects:

-   [mopro-zkemail-nr](https://github.com/zkmopro/mopro-zkemail-nr)
-   [stealthnote-mobile](https://github.com/vivianjeng/stealthnote-mobile)
-   [test-e2e](https://github.com/zkmopro/mopro/tree/main/test-e2e)

## Setting Up the Rust Project

To get started, follow the [Rust Setup Guide](/setup/rust-setup.md) and activate the `noir` feature in your `Cargo.toml`:

```toml
[features]
default = ["mopro-ffi/noir"]

[dependencies]
mopro-ffi = { version = "0.2" }
# ...
```

:::info
The Noir adapter depends on [`zkmopro/noir-rs`](https://github.com/zkmopro/noir-rs). Please checkout the usage and the version here.
:::

## Proving and Verifying Functions

### Preparing SRS

Please follow this guide to generate the SRS for your Noir circuit: [Downloading SRS (Structured Reference String)](https://github.com/zkmopro/noir-rs?tab=readme-ov-file#downloading-srs-structured-reference-string)

### Proving Function

The proving function is responsible for loading the circuit, loading the SRS, converting the inputs into a witness map, and generating the proof using the Barretenberg backend.

```rust
pub fn generate_noir_proof(
    circuit_path: String,
    srs_path: Option<String>,
    inputs: Vec<String>,
) -> Vec<u8>;
```

-   `circuit_path`: Path to the compiled Noir .acir circuit.
-   `srs_path`: Optional path to the structured reference string.
-   `inputs`: A list of strings representing public/private inputs.
-   Returns a serialized proof (`Vec<u8>`).

### Verifying Function

The verification function loads the circuit and derives the verification key, then verifies the proof.

```rust
pub fn verify_noir_proof(circuit_path: String, proof: Vec<u8>) -> bool;
```

-   `circuit_path`: Path to the compiled Noir .acir circuit.
-   `proof`: The serialized proof to verify.
-   Returns `true` if the proof is valid.

## Using the Library

### iOS API

```swift
generateNoirProof(circuitPath: String, srsPath: String?, inputs: [String])throws  -> Data
```

```swift
verifyNoirProof(circuitPath: String, proof: Data)throws  -> Bool
```

### Android API

```kotlin
fun generateNoirProof(
    circuitPath: kotlin.String,
    srsPath: kotlin.String?,
    inputs: List<kotlin.String>,
): kotlin.ByteArray
```

```kotlin
fun verifyNoirProof(
    circuitPath: kotlin.String,
    proof: kotlin.ByteArray,
): kotlin.Boolean
```

The Noir adapter exposes the equivalent functions and types to be used in the Android project.

## Platform Support

| Platform                | Target Triple              | Status |
| ----------------------- | -------------------------- | ------ |
| iOS Device              | `aarch64-apple-ios`        | ✅     |
| iOS aarch64 Simulator   | `aarch64-apple-ios-sim`    | ✅     |
| iOS x86_64 Simulator    | `x86_64-apple-ios`         | ✅     |
| Android aarch64 Device  | `aarch64-linux-android`    | ✅     |
| Android x86_64 Emulator | `x86_64-linux-android`     | ✅     |
| macOS (M1/M2)           | `aarch64-apple-darwin`     | ✅     |
| Linux Desktop           | `x86_64-unknown-linux-gnu` | ✅     |
