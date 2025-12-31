# Noir Adapter

Mopro supports the integration of [Noir](https://noir-lang.org/) circuits, enabling zero-knowledge proofs in native mobile applications with ease. This adapter builds upon foundational work from the [zkPassport](https://zkpassport.id/) team and streamlines cross-platform integration with performance, reliability, and developer experience in mind.

## Samples

You can explore real examples of how the Noir adapter works in these projects:

-   [mopro-zkemail-nr](https://github.com/zkmopro/mopro-zkemail-nr)
-   [stealthnote-mobile](https://github.com/vivianjeng/stealthnote-mobile)
-   [mopro-wallet-connect-noir](https://github.com/moven0831/mopro-wallet-connect-noir)

Or follow the mopro CLI [getting started](/docs/getting-started) and select the **Noir** adapter to see how to implement a Noir prover using mopro.

## Setting Up the Rust Project

To get started, follow the [Rust Setup Guide](/setup/rust-setup.md) and ensure the `noir_rs` package is imported:

```toml
[dependencies]
noir_rs = { package = "noir", git = "https://github.com/zkmopro/noir-rs", features = [
    "barretenberg",
    "android-compat",
], branch = "v1.0.0-beta.8-3" }
# ...
```

:::info
The Noir adapter depends on [`zkmopro/noir-rs`](https://github.com/zkmopro/noir-rs). Please checkout the usage and the version here.

**Current version**: Supports Noir `1.0.0-beta.8-3` with bb `1.0.0-nightly.20250723` with updated dependencies and enhanced functionality.

**Backend**: The adapter uses the Barretenberg backend, which is automatically downloaded from the released GitHub artifacts at [zkmopro/aztec-packages](https://github.com/zkmopro/aztec-packages/releases).
:::

## Hash Function Selection

The Noir adapter supports two functions as oracle hash options for different use cases:

-   **Poseidon hash**: Default choice, optimized for performance and off-chain verification
-   **Keccak256 hash**: Gas-efficient, required for Solidity verifier compatibility and on-chain verification

The hash function is automatically selected based on the `on_chain` parameter.

### Key Features

-   **Automatic Hash Selection**: Automatically chooses between Poseidon (performance) and Keccak256 (EVM compatibility) based on your use case
-   **Memory Optimization**: Low memory mode available for mobile devices
-   **Cross-Platform**: Works across iOS, Android, and other supported platforms

## Proving and Verifying Functions

### Preparing SRS

Please follow this guide to generate the SRS for your Noir circuit: [Downloading SRS (Structured Reference String)](https://github.com/zkmopro/noir-rs?tab=readme-ov-file#downloading-srs-structured-reference-string)

## Rust API

The Noir adapter provides three main functions that automatically handle hash function selection based on the `on_chain` parameter:

```rust
pub fn generate_noir_proof(
    circuit_path: String,
    srs_path: Option<String>,
    inputs: Vec<String>,
    on_chain: bool,
    vk: Vec<u8>,
    low_memory_mode: bool,
) -> Result<Vec<u8>, MoproError>;

pub fn verify_noir_proof(
    circuit_path: String,
    proof: Vec<u8>,
    on_chain: bool,
    vk: Vec<u8>,
    low_memory_mode: bool,
) -> Result<bool, MoproError>;

pub fn get_noir_verification_key(
    circuit_path: String,
    srs_path: Option<String>,
    on_chain: bool,
    low_memory_mode: bool,
) -> Result<Vec<u8>, MoproError>;
```

### Parameters

-   `circuit_path`: Path to the compiled Noir `.json` circuit
-   `srs_path`: Optional path to the structured reference string
-   `inputs`: List of strings representing public/private inputs (proof generation only)
-   `proof`: The serialized proof to verify (verification only)
-   `on_chain`: If `true`, uses Keccak256 hash for Solidity compatibility; if `false`, uses Poseidon hash for better performance
-   `vk`: Pre-generated verification key bytes
-   `low_memory_mode`: Enables memory optimization for resource-constrained environments

### Usage Notes

-   **Hash Selection**: Set `on_chain = true` for Ethereum/EVM compatibility, or `on_chain = false` for better performance
-   **Verification Keys**: Pre-generate verification keys using `get_noir_verification_key` and reuse them for better performance
-   **Memory Optimization**: Enable `low_memory_mode = true` for resource-constrained mobile environments

## Platform Support

The Noir adapter supports the following target platforms with Barretenberg backend binaries:

| Platform                | Target Triple              | Status |
| ----------------------- | -------------------------- | ------ |
| macOS (Intel)           | `x86_64-apple-darwin`      | ✅     |
| macOS (M Series)        | `aarch64-apple-darwin`     | ✅     |
| iOS Device              | `aarch64-apple-ios`        | ✅     |
| iOS aarch64 Simulator   | `aarch64-apple-ios-sim`    | ✅     |
| iOS x86_64 Simulator    | `x86_64-apple-ios`         | ✅     |
| Android aarch64 Device  | `aarch64-linux-android`    | ✅     |
| Android x86_64 Emulator | `x86_64-linux-android`     | ✅     |
| Linux Desktop           | `x86_64-unknown-linux-gnu` | ✅     |

All platforms use pre-compiled Barretenberg binaries automatically downloaded from [zkmopro/aztec-packages releases](https://github.com/zkmopro/aztec-packages/releases) during the build process.

## Using the Library

After you have specified the circuits you want to use, you can follow the usual steps to build the library and use it in your project.

### iOS API

The Noir adapter exposes the following functions to be used in the iOS project:

### `generateNoirProof`

```swift
public func generateNoirProof(circuitPath: String, srsPath: String?, inputs: [String], onChain: Bool, vk: Data, lowMemoryMode: Bool)throws  -> Data 
```

### `verifyNoirProof`

```swift
public func verifyNoirProof(circuitPath: String, proof: Data, onChain: Bool, vk: Data, lowMemoryMode: Bool)throws  -> Bool  
```

### `getNoirVerificationKey`

```swift
public func getNoirVerificationKey(circuitPath: String, srsPath: String?, onChain: Bool, lowMemoryMode: Bool)throws  -> Data  {

```

### Android API

The Noir adapter exposes the equivalent functions and types to be used in the Android project.

### `generateNoirProof`

```kotlin
fun `generateNoirProof`(
    `circuitPath`: kotlin.String,
    `srsPath`: kotlin.String?,
    `inputs`: List<kotlin.String>,
    `onChain`: kotlin.Boolean,
    `vk`: kotlin.ByteArray,
    `lowMemoryMode`: kotlin.Boolean,
): kotlin.ByteArray
```

### `verifyNoirProof`

```kotlin
fun `verifyNoirProof`(
    `circuitPath`: kotlin.String,
    `proof`: kotlin.ByteArray,
    `onChain`: kotlin.Boolean,
    `vk`: kotlin.ByteArray,
    `lowMemoryMode`: kotlin.Boolean,
): kotlin.Boolean
```

### `getNoirVerificationKey`

```kotlin
fun `getNoirVerificationKey`(
    `circuitPath`: kotlin.String,
    `srsPath`: kotlin.String?,
    `onChain`: kotlin.Boolean,
    `lowMemoryMode`: kotlin.Boolean,
): kotlin.ByteArray
```