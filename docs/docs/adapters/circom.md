# Circom Adapter

Mopro supports the integration of Circom circuits. For this, you need to have pre-built `zkey` and `wasm` files for your
circuits. You can find more information on how to generate these files in the [Circom documentation](https://docs.circom.io).

## Sample Project

Explore how the Circom adapter is implemented by checking out this sample project [mopro-app](https://github.com/vimwitch/mopro-app) or the [test-e2e](https://github.com/zkmopro/mopro/tree/main/test-e2e) where we maintain (and test) each adapter.

## Setup the rust project

You can follow the instructions in the [Rust Setup](/getting-started/rust-setup.md) guide to create a new Rust project that builds this library with Circom proofs.

In your `Cargo.toml` file, ensure the `circom` feature is activated for `mopro-ffi`:

```toml
[features]
default = ["mopro-ffi/circom"]
```

## Witness Generation Functions

In order for the Mopro to be able to generate proofs for your chosen circom circuits, you need to provide a witness
generation function for each of the circuits you plan to use to generate proofs for. This function handles the witness
generation for your circuit. You can read more about witnesses for circom circuits [here](https://docs.circom.io/background/background/#witness).

The function signature should be:

```rust
pub type WtnsFn = fn(HashMap<String, Vec<BigInt>>) -> Vec<BigInt>;
```

## Implementing the Witness Function

For simplicity, you can use the `witness!` macro provided by the `rust-witness` crate. This macro generates a witness
function for you given the circuit name. You can read more about the `witness!` macro [here](https://github.com/vimwitch/rust-witness).

#### Adding the `rust-witness` Crate Dependency

To use it, you must first add the `rust-witness` crate to your `Cargo.toml` regular and build dependencies:

```toml
[dependencies]
# ...
rust-witness = { git = "https://github.com/vimwitch/rust-witness.git" }

[build-dependencies]
# ...
rust-witness = { git = "https://github.com/vimwitch/rust-witness.git" }
```

#### Configuring the path to the `.wasm` circuit files in the `build.rs`

Then you need to add to the `build.rs` the call to `rust_witness::transpile::transpile_wasm` macro and pass it the path 
to the folder containing the `.wasm` files for the circom circuits. The path can be absolute or a relative to the location 
of the `build.rs` file. Note that the `.wasm` files can be recursively in subfolders of the specified folder, as in the example below.

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

#### Automatically Generating Witness Functions

Then you can automatically generate the witness functions for all the circuits in the specified folder.

To do so, in the `lib.rs` file, you can add the following:

```rust
rust_witness::witness!(multiplier2);
rust_witness::witness!(multiplier3);
rust_witness::witness!(keccak256256test);    
```

This will generate the witness function for the specified circuit following [the naming convention here](https://github.com/vimwitch/rust-witness?tab=readme-ov-file#rust-witness). 

## Setting the Circom Circuits

To set Circom circuits you want to use on other platforms, you need to use the `set_circom_circuits!` macro provided by the 
`mopro-ffi` crate. This macro should be called in the `lib.rs` file of your project, after the `mopro_ffi::app()` macro call.
 You should pass it a list of tuples (pairs), where the first element is the name of the `zkey` file and the second element is the witness generation function.

For example:

```rust
mopro_ffi::set_circom_circuits! {
    ("multiplier2_final.zkey", multiplier2_witness),
    ("multiplier3_final.zkey", multiplier3_witness),
    ("keccak256_256_test_final.zkey", keccak256256test_witness),
}
```

Under the hood, the `set_circom_circuits!` macro will generate a `get_circom_wtns_fn` function that will be used to get
the witness generation function for a given circuit `zkey` file.

### Manual Configuration

For advanced users, you can manually define the `get_circom_wtns_fn` function in the `lib.rs` file:

```rust
fn get_circom_wtns_fn(circuit: &str) -> Result<mopro_ffi::WtnsFn, mopro_ffi::MoproError> {
    match circuit {
        "your_circuit.zkey" => Ok(your_circuit_wtns_gen_fn),
        _ => Err(mopro_ffi::MoproError::CircomError(format!("Unknown ZKEY: {}", circuit).to_string()))
    }
}
```

This might be useful if you want to have more control over the proving functions for each circuit.

## Using the Library

After you have specified the circuits you want to use, you can follow the usual steps to build the library and use it in your project.

### iOS API

The Circom adapter exposes the following functions to be used in the iOS project:

```swift
// Generate a proof for a given circuit zkey, as well as the circuit inputs
// Make sure that the name of the zkey file matches the one you set in the `set_circom_circuits!` macro
generateCircomProof(zkeyPath: zkeyPath, circuitInputs: inputs) -> GenerateProofResult

// Verify a proof for a given circuit zkey
// This works for arbitrary circuits, as long as the zkey file is valid
verifyCircomProof(
    zkeyPath: zkeyPath, proof: generateProofResult.proof, publicInput: generateProofResult.inputs) -> Bool

// Convert a Circom proof to an Ethereum compatible proof
toEthereumProof(proof: generateProofResult.proof) -> ProofCalldata

// Convert a Circom public input to an Ethereum compatible public input
toEthereumInputs(inputs: generateProofResult.inputs) -> [String]
```

As well as the following types:

```swift
public struct G1 {
    public var x: String
    public var y: String
}
    
public struct G2 {
    public var x: [String]
    public var y: [String]
}
    
public struct ProofCalldata {
    public var a: G1
    public var b: G2
    public var c: G1
}
    
public struct GenerateProofResult {
    public var proof: Data
    public var inputs: Data
}
```

### Android API

The circom adapter exposes the equivalent functions and types to be used in the Android project. 


