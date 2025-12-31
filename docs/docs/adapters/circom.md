# Circom Adapter

Mopro supports the integration of Circom circuits. For this, you need to have pre-built `zkey` and `wasm` files for your circuits. You can find more information on how to generate these files in the [Circom documentation](https://docs.circom.io).

---

## Preparing Circuits

To use your own Circom circuits with Mopro, you need to prepare the necessary files by following these steps:

### 1. Compile the Circuit

Use Circom to compile your `.circom` file into `.wasm` and `.r1cs` files:

```sh
circom <your_circuit>.circom --wasm --r1cs -o <output_dir>
```

-   `<your_circuit>.circom`: your circuit source file
-   `<output_dir>`: directory for output files (e.g., `test-vectors/circom/`)

### 2. Trusted Setup (Powers of Tau)

You need a ptau file (e.g., `powersOfTau28_hez_final_21.ptau`). Download it or generate it as described in the [Circom docs](https://docs.circom.io/getting-started/proving-circuits/#powers-of-tau-ceremony).

Run the trusted setup:

```sh
snarkjs groth16 setup <output_dir>/<your_circuit>.r1cs <ptau_file> <output_dir>/<your_circuit>_0000.zkey
snarkjs zkey contribute <output_dir>/<your_circuit>_0000.zkey <output_dir>/<your_circuit>_final.zkey --name="Contributor Name"
```

### 3. (Optional) Generate arkzkey

If you use the Arkworks backend, you may need to convert the zkey to arkzkey or use it directly if supported by your workflow.

### 4. Place the Files

Put the resulting `.wasm` and `.zkey` (or arkzkey) files in a directory such as `test-vectors/circom/`.

### 5. Configure Your Rust Project

In your `build.rs`:

```rust
rust_witness::transpile::transpile_wasm("test-vectors/circom".to_string());
```

In your `lib.rs`:

```rust
rust_witness::witness!(your_circuit);
```

### 6. Proof Generation Example

See the [circom-prover README](https://github.com/zkmopro/mopro/blob/main/circom-prover/README.md) for how to generate and verify proofs using these files.

---

## Samples

Please follow the mopro CLI [getting started](/docs/getting-started) and select the **Circom** adapter to see how to implement a Circom prover using mopro.

## Setup the rust project

You can follow the instructions in the [Rust Setup](/setup/rust-setup.md) guide to create a new Rust project that builds this library with Circom proofs.

In your `Cargo.toml` file, ensure the `circom-prover` package is imported:

```toml
[dependencies]
circom-prover = "0.1"
# ...
```

## Witness Generation Functions

In order for the Mopro to be able to generate proofs for your chosen circom circuits, you need to provide a witness generation function for each of the circuits you plan to use to generate proofs for. This function handles the witness generation for your circuit. You can read more about witnesses for circom circuits [here](https://docs.circom.io/background/background/#witness).

Mopro provides three different witness generators. By default, the Mopro CLI uses `rust-witness`, as it offers better stability and faster build times.

If you're interested in exploring the differences between witness generators, check out [this blog post](/blog/circom-comparison).

### [`rust-witness`](https://github.com/chancehudson/rust-witness)

The function signature should be:

```rust
type RustWitnessWtnsFn = fn(HashMap<String, Vec<BigInt>>) -> Vec<BigInt>;
```

-   Implementing the Witness Function

For simplicity, you can use the `witness!` macro provided by the `rust-witness` crate. This macro generates a witness function for you given the circuit name. You can read more about the `witness!` macro [here](https://github.com/chancehudson/rust-witness).

-   Adding the `rust-witness` Crate Dependency

To use it, you must first add the `rust-witness` crate to your `Cargo.toml` regular and build dependencies:

```toml
[dependencies]
# ...
rust-witness = "0.1"

[build-dependencies]
# ...
rust-witness = "0.1"
```

-   Configuring the path to the `.wasm` circuit files in the `build.rs`

Then you need to add to the `build.rs` the call to `rust_witness::transpile::transpile_wasm` macro and pass it the path to the folder containing the `.wasm` files for the circom circuits. The path can be absolute or a relative to the location of the `build.rs` file. Note that the `.wasm` files can be recursively in subfolders of the specified folder, as in the example below.

For example, for the following project structure:

```text
your-rust-project
├── build.rs
...
test-vectors
├── circom
│   ├── multiplier
│   │   ├── multiplier2.wasm
│   │   └── multiplier3.wasm
│   └── keccak256_256_test.wasm
...
```

You will need to add the following to the `build.rs` file:

```rust
fn main() {
    // ...
    rust_witness::transpile::transpile_wasm("../test-vectors/circom".to_string());
    // ...
}
```

-   Automatically Generating Witness Functions

Then you can automatically generate the witness functions for all the circuits in the specified folder.

To do so, in the `lib.rs` file, you can add the following:

```rust
rust_witness::witness!(multiplier2);
rust_witness::witness!(multiplier3);
rust_witness::witness!(keccak256256test);
```

This will generate the witness function for the specified circuit following [the naming convention here](https://github.com/chancehudson/rust-witness?tab=readme-ov-file#rust-witness).

### [`witnesscalc_adapter`](https://github.com/zkmopro/witnesscalc_adapter)

:::warning
Ensure the following [requirements](https://github.com/zkmopro/witnesscalc_adapter?tab=readme-ov-file#requirements) before using `witnesscalc_adapter`.
:::

The witnesscalc-adapter is based on the original [witnesscalc](https://github.com/0xPolygonID/witnesscalc) implementation in C++. To use it, compile your Circom circuit with the `--c` flag to generate C++ code. Then, retrieve the `.dat` file from the output C++ folder for use in the adapter. For example, compile the circuit using:

```sh
circom multiplier2.circom --c
```

-   The function signature should be:

```rust
type WitnesscalcWtnsFn = fn(&str) -> anyhow::Result<Vec<u8>>;
```

-   Adding the `witnesscalc-adapter` Crate Dependency

To use it, you must first add the `witnesscalc-adapter` crate to your `Cargo.toml` regular and build dependencies:

```toml
[dependencies]
# ...
circom-prover = { version="0.1", features = ["witnesscalc"] } # activate witnesscalc feature
witnesscalc-adapter = "0.1"

[build-dependencies]
# ...
witnesscalc-adapter = "0.1"
```

-   Configuring the path to the `.dat` circuit files in the `build.rs`

Then you need to add to the `build.rs` the call to `witnesscalc_adapter::build_and_link` macro and pass it the path to the folder containing the `.dat` files for the circom circuits. The path can be absolute or a relative to the location of the `build.rs` file. Note that the `.dat` files can be recursively in subfolders of the specified folder.

You will need to add the following to the `build.rs` file:

```rust
fn main() {
    // ...
    witnesscalc_adapter::build_and_link("../test-vectors/circom");
    // ...
}
```

-   Automatically Generating Witness Functions

Then you can automatically generate the witness functions for all the circuits in the specified folder.

To do so, in the `lib.rs` file, you can add the following:

```rust
witnesscalc_adapter::witness!(multiplier2);
witnesscalc_adapter::witness!(multiplier3);
witnesscalc_adapter::witness!(keccak_256_256_test);
```

:::warning
Currently, circuit names can include underscores (`_`) but not dashes (`-`). <br/>
Make sure your circuit name does not contain dashes, and avoid using `main` as the circuit name to prevent conflicts.
:::

### [`circom-witnesscalc`](https://github.com/iden3/circom-witnesscalc)

:::danger
It doesn't support for some circuits: see [Unimplemented features](https://github.com/iden3/circom-witnesscalc?tab=readme-ov-file#unimplemented-features)
:::

-   Compile a circuit graph before using the `circom-witnesscalc`. Please checkout: [Compile a circuit and build the witness graph](https://github.com/iden3/circom-witnesscalc?tab=readme-ov-file#compile-a-circuit-and-build-the-witness-graph).

-   The function signature should be:

```rust
type CircomWitnessCalcWtnsFn = fn(&str) -> anyhow::Result<Vec<u8>>;
```

-   Activate `circom-witnesscalc` Feature

To use it, you only need to activate the `circom-witnesscalc` feature in `mopro-ffi` in your `Cargo.toml`.

```toml
[dependencies]
# ...
circom-prover = { version= "0.1", features = ["circom-witnesscalc"] } # activate circom-witnesscalc feature
```

-   Use the Mopro helper to enable the graph functionality in your project.

```rust
const GRAPH_PATH: &str = "./test-vectors/circom/multiplier2.bin";
mopro_ffi::graph!(multiplier2, &GRAPH_PATH)
// The witness name (multiplier2) does not need to match the file name, and there are no restrictions on its format.
// Then you will have a witness function called multiplier2_witness
```

## Setting the Circom Circuits

To set Circom circuits you want to use on other platforms, you use the `set_circom_circuits!` macro generated by the Mopro CLI template. This macro should be called in the `lib.rs` file of your project, after the `mopro_ffi::app!()` macro call. You should pass it a list of tuples (pairs), where the first element is the name of the `zkey` file and the second element is the witness generation function.

For example:

```rust
crate::set_circom_circuits! {
    // using rust-witness
    ("multiplier2_final.zkey", circom_prover::witness::WitnessFn::RustWitness(multiplier2_witness)),
    // using witnesscalc
    ("multiplier3_final.zkey", circom_prover::witness::WitnessFn::WitnessCalc(multiplier3_witness)),
    // using circom-witnesscalc
    ("keccak256_256_test_final.zkey", circom_prover::witness::WitnessFn::CircomWitnessCalc(keccak256256test_witness)),
}
```

Under the hood, the `set_circom_circuits!` macro generates a small lookup table and a helper function:

```rust
pub(crate) fn circom_get(name: &str) -> Option<circom_prover::witness::WitnessFn> { /* generated by the macro */ }
```

This helper is used by the Circom adapter (`generate_circom_proof`) to find the correct witness generation function for a given `zkey` file name.

### Manual Configuration

For advanced users, you can manually define the `circom_get` function in the `lib.rs` file instead of using the macro, as long as you keep the same signature. The Circom adapter will call this function to resolve the witness function for a given `zkey`:

```rust
use circom_prover::witness::WitnessFn;

pub(crate) fn circom_get(name: &str) -> Option<WitnessFn> {
    match name {
        "your_circuit.zkey" => Some(WitnessFn::RustWitness(your_circuit_wtns_gen_fn)),
        _ => None,
    }
}
```

This might be useful if you want to have more control over how circuits are mapped to witness functions (for example, adding custom feature flags or other runtime logic).

## Proof Generation Functions

Mopro now supports 2 Circom provers. You can find more information in [the blog post](/blog/circom-comparison).

### [`ark-works`](https://github.com/arkworks-rs)

-   By default, Arkworks is enabled as it offers greater stability.

### [`rust-rapidsnark`](https://github.com/zkmopro/rust-rapidsnark)

-   `rust-rapidsnark` is based on the original C++ implementation of [rapidsnark](https://github.com/iden3/rapidsnark), with the binary wrapped and integrated in Rust.
-   Activate `rapidsnark` Feature for both `[dependencies]` and `[build-dependencies]`

```toml
[dependencies]
# ...
circom-prover = { version= "0.1", features = ["rapidsnark"] } # activate rapidsnark feature

[build-dependencies]
# ...
circom-prover = { version= "0.1", features = ["rapidsnark"] } # activate rapidsnark featurer
```

## Generate Proofs

By integrating the witness generator with the prover, you can run `generate_circom_proof` using

```rust
let zkey_path = "./test-vectors/circom/multiplier2_final.zkey".to_string();
let circuit_inputs = "{\"a\": 2, \"b\": 3}".to_string();
let result = generate_circom_proof(
    zkey_path.clone(),
    circuit_inputs,
    ProofLib::Arkworks
    // To use rapidsnark
    // ProofLib::Rapidsnark
);
```

## Using the Library

After you have specified the circuits you want to use, you can follow the usual steps to build the library and use it in your project.

### iOS API

The Circom adapter exposes the following functions to be used in the iOS project:

### `generateCircomProof`

```swift
// Generate a proof for a given circuit zkey, as well as the circuit inputs
// Make sure that the name of the zkey file matches the one you set in the `set_circom_circuits!` macro
public func generateCircomProof(zkeyPath: String, circuitInputs: String, proofLib: ProofLib)throws  -> CircomProofResult
```

### `verifyCircomProof`

```swift
// Verify a proof for a given circuit zkey
// This works for arbitrary circuits, as long as the zkey file is valid
public func verifyCircomProof(zkeyPath: String, proofResult: CircomProofResult, proofLib: ProofLib)throws  -> Bool
```

### `G1`

```swift
public struct G1 {
    public var x: String
    public var y: String
    public var z: String
}
```

### `G2`

```swift
public struct G2 {
    public var x: [String]
    public var y: [String]
    public var z: [String]
}
```

### `CircomProof`

```swift
public struct CircomProof {
    public var a: G1
    public var b: G2
    public var c: G1
    public var `protocol`: String
    public var curve: String
}
```

### `CircomProofResult`

```swift
public struct CircomProofResult {
    public var proof: CircomProof
    public var inputs: [String]
}
```

### `ProofLib`

```swift
public enum ProofLib {
    case arkworks
    case rapidsnark
}
```

### Android API

The Circom adapter exposes the equivalent functions and types to be used in the Android project.

### `generateCircomProof`

```kotlin
// Generate a proof for a given circuit zkey, as well as the circuit inputs
// Make sure that the name of the zkey file matches the one you set in the `set_circom_circuits!` macro
fun `generateCircomProof`(
    `zkeyPath`: kotlin.String,
    `circuitInputs`: kotlin.String,
    `proofLib`: ProofLib,
): CircomProofResult
```

### `verifyCircomProof`

```kotlin
fun `verifyCircomProof`(
    `zkeyPath`: kotlin.String,
    `proofResult`: CircomProofResult,
    `proofLib`: ProofLib,
): kotlin.Boolean
```

### `G1`

```kotlin
data class G1(
    var `x`: kotlin.String,
    var `y`: kotlin.String,
    var `z`: kotlin.String,
)
```

### `G2`

```kotlin
data class G2(
    var `x`: List<kotlin.String>,
    var `y`: List<kotlin.String>,
    var `z`: List<kotlin.String>,
)
```

### `CircomProof`

```kotlin
data class CircomProof(
    var `a`: G1,
    var `b`: G2,
    var `c`: G1,
    var `protocol`: kotlin.String,
    var `curve`: kotlin.String,
)
```

### `CircomProofResult`

```kotlin
data class CircomProofResult(
    var `proof`: CircomProof,
    var `inputs`: List<kotlin.String>,
)
```

### `ProofLib`

```kotlin
enum class ProofLib {
    ARKWORKS,
    RAPIDSNARK,
}
```
