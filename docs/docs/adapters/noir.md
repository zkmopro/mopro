# Noir Adapter

Mopro supports the integration of [Noir](https://noir-lang.org/) circuits, enabling zero-knowledge proofs in native mobile applications with ease. This adapter builds upon foundational work from the [zkPassport](https://zkpassport.id/) team and streamlines cross-platform integration with performance, reliability, and developer experience in mind.

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

**Current version**: Supports Noir `1.0.0-beta.8-3` with bb `1.0.0-nightly.20250723` with updated dependencies and enhanced functionality.

**Backend**: The adapter uses the Barretenberg backend, which is automatically downloaded from the released GitHub artifacts at [zkmopro/aztec-packages](https://github.com/zkmopro/aztec-packages/releases).
:::

## Hash Function Selection

The Noir adapter supports two hash functions for different use cases:

- **Poseidon hash**: Default choice, optimized for performance and off-chain verification
- **Keccak256 hash**: Required for Solidity verifier compatibility and on-chain verification

The hash function is automatically selected based on the `on_chain` parameter in the main proving functions, or you can use the hash-specific functions directly.

### New Features

The updated Noir adapter introduces several key enhancements:

- **Pre-generated Verification Keys**: Support for using existing verification keys (`vk` parameter) instead of deriving them from circuits each time, significantly improving performance.
- **Low Memory Mode**: Optional memory optimization for resource-constrained mobile environments (`low_memory_mode` parameter).
- **Oracle Hash Abstraction**: Flexible hash function selection enabling seamless switching between Poseidon and Keccak256 based on deployment requirements.
- **Enhanced Error Handling**: Improved error reporting with `Result<T, MoproError>` return types for better debugging and error management.

## Proving and Verifying Functions

### Preparing SRS

Please follow this guide to generate the SRS for your Noir circuit: [Downloading SRS (Structured Reference String)](https://github.com/zkmopro/noir-rs?tab=readme-ov-file#downloading-srs-structured-reference-string)

### Main Proving Function

The main proving function automatically selects the appropriate hash function based on the `on_chain` parameter:

```rust
pub fn generate_noir_proof(
    circuit_path: String,
    srs_path: Option<String>,
    inputs: Vec<String>,
    on_chain: bool,
    vk: Vec<u8>,
    low_memory_mode: bool,
) -> Result<Vec<u8>, MoproError>;
```

-   `circuit_path`: Path to the compiled Noir `.json` circuit.
-   `srs_path`: Optional path to the structured reference string.
-   `inputs`: A list of strings representing public/private inputs.
-   `on_chain`: If `true`, uses Keccak256 hash for Solidity compatibility; if `false`, uses Poseidon hash.
-   `vk`: Pre-generated verification key bytes.
-   `low_memory_mode`: Enables memory optimization for resource-constrained environments.
-   Returns a serialized proof (`Vec<u8>`).

### Main Verifying Function

The main verification function automatically uses the appropriate hash function based on the `on_chain` parameter:

```rust
pub fn verify_noir_proof(
    circuit_path: String,
    proof: Vec<u8>,
    on_chain: bool,
    vk: Vec<u8>,
    low_memory_mode: bool,
) -> Result<bool, MoproError>;
```

-   `circuit_path`: Path to the compiled Noir `.json` circuit.
-   `proof`: The serialized proof to verify.
-   `on_chain`: Must match the value used during proof generation.
-   `vk`: Pre-generated verification key bytes.
-   `low_memory_mode`: Enables memory optimization for resource-constrained environments.
-   Returns `true` if the proof is valid.

### Hash-Specific Functions

#### Poseidon Hash Functions (Off-chain)

For explicit Poseidon hash usage (better performance, off-chain verification):

```rust
pub fn generate_noir_proof_with_poseidon(
    circuit_path: String,
    srs_path: Option<String>,
    inputs: Vec<String>,
    vk: Vec<u8>,
    low_memory_mode: bool,
) -> Result<Vec<u8>, MoproError>;

pub fn verify_noir_proof_with_poseidon(
    circuit_path: String,
    proof: Vec<u8>,
    vk: Vec<u8>,
    low_memory_mode: bool,
) -> Result<bool, MoproError>;

pub fn get_noir_verification_poseidon_key(
    circuit_path: String,
    srs_path: Option<String>,
    low_memory_mode: bool,
) -> Result<Vec<u8>, MoproError>;
```

#### Keccak256 Hash Functions (On-chain)

For explicit Keccak256 hash usage (Solidity compatible, on-chain verification):

```rust
pub fn generate_noir_proof_with_keccak(
    circuit_path: String,
    srs_path: Option<String>,
    inputs: Vec<String>,
    disable_zk: bool,
    vk: Vec<u8>,
    low_memory_mode: bool,
) -> Result<Vec<u8>, MoproError>;

pub fn verify_noir_proof_with_keccak(
    circuit_path: String,
    proof: Vec<u8>,
    disable_zk: bool,
    vk: Vec<u8>,
    low_memory_mode: bool,
) -> Result<bool, MoproError>;

pub fn get_noir_verification_keccak_key(
    circuit_path: String,
    srs_path: Option<String>,
    low_memory_mode: bool,
) -> Result<Vec<u8>, MoproError>;
```

Note: `disable_zk` parameter allows generating proofs without zero-knowledge properties for testing purposes.

## Using the Library

### iOS API

#### Main Functions

```swift
public func generateNoirProof(
    circuitPath: String,
    srsPath: String?,
    inputs: [String],
    onChain: Bool,
    vk: Data,
    lowMemoryMode: Bool
) throws -> Data

public func verifyNoirProof(
    circuitPath: String,
    proof: Data,
    onChain: Bool,
    vk: Data,
    lowMemoryMode: Bool
) throws -> Bool
```

#### Hash-Specific Functions

```swift
// Poseidon (off-chain)
public func generateNoirProofWithPoseidon(
    circuitPath: String,
    srsPath: String?,
    inputs: [String],
    vk: Data,
    lowMemoryMode: Bool
) throws -> Data

public func verifyNoirProofWithPoseidon(
    circuitPath: String,
    proof: Data,
    vk: Data,
    lowMemoryMode: Bool
) throws -> Bool

public func getNoirVerificationPoseidonKey(
    circuitPath: String,
    srsPath: String?,
    lowMemoryMode: Bool
) throws -> Data

// Keccak256 (on-chain)
public func generateNoirProofWithKeccak(
    circuitPath: String,
    srsPath: String?,
    inputs: [String],
    disableZk: Bool,
    vk: Data,
    lowMemoryMode: Bool
) throws -> Data

public func verifyNoirProofWithKeccak(
    circuitPath: String,
    proof: Data,
    disableZk: Bool,
    vk: Data,
    lowMemoryMode: Bool
) throws -> Bool

public func getNoirVerificationKeccakKey(
    circuitPath: String,
    srsPath: String?,
    lowMemoryMode: Bool
) throws -> Data
```

### Android API

#### Main Functions

```kotlin
fun generateNoirProof(
    circuitPath: kotlin.String,
    srsPath: kotlin.String?,
    inputs: List<kotlin.String>,
    onChain: kotlin.Boolean,
    vk: kotlin.ByteArray,
    lowMemoryMode: kotlin.Boolean,
): kotlin.ByteArray

fun verifyNoirProof(
    circuitPath: kotlin.String,
    proof: kotlin.ByteArray,
    onChain: kotlin.Boolean,
    vk: kotlin.ByteArray,
    lowMemoryMode: kotlin.Boolean,
): kotlin.Boolean
```

#### Hash-Specific Functions

```kotlin
// Poseidon (off-chain)
fun generateNoirProofWithPoseidon(
    circuitPath: kotlin.String,
    srsPath: kotlin.String?,
    inputs: List<kotlin.String>,
    vk: kotlin.ByteArray,
    lowMemoryMode: kotlin.Boolean,
): kotlin.ByteArray

fun verifyNoirProofWithPoseidon(
    circuitPath: kotlin.String,
    proof: kotlin.ByteArray,
    vk: kotlin.ByteArray,
    lowMemoryMode: kotlin.Boolean,
): kotlin.Boolean

fun getNoirVerificationPoseidonKey(
    circuitPath: kotlin.String,
    srsPath: kotlin.String?,
    lowMemoryMode: kotlin.Boolean,
): kotlin.ByteArray

// Keccak256 (on-chain)
fun generateNoirProofWithKeccak(
    circuitPath: kotlin.String,
    srsPath: kotlin.String?,
    inputs: List<kotlin.String>,
    disableZk: kotlin.Boolean,
    vk: kotlin.ByteArray,
    lowMemoryMode: kotlin.Boolean,
): kotlin.ByteArray

fun verifyNoirProofWithKeccak(
    circuitPath: kotlin.String,
    proof: kotlin.ByteArray,
    disableZk: kotlin.Boolean,
    vk: kotlin.ByteArray,
    lowMemoryMode: kotlin.Boolean,
): kotlin.Boolean

fun getNoirVerificationKeccakKey(
    circuitPath: kotlin.String,
    srsPath: kotlin.String?,
    lowMemoryMode: kotlin.Boolean,
): kotlin.ByteArray
```

The Noir adapter exposes the equivalent functions and types to be used in both iOS and Android projects.

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
