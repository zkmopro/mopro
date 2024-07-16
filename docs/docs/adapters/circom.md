# Circom Adapter

Mopro supports Circom circuits. To develop with Circom, you need to have pre-built `zkey` and `wasm` files for your
circuits. You can find more information on how to generate these files in
the [Circom documentation](https://docs.circom.io).

## Sample Project

You can find a sample project that demonstrates how to use the Circom adapter in
the [mopro-app](https://github.com/vimwitch/mopro-app).

## Setup the rust project

You can follow the instructions in the [Rust Setup](/getting-started/rust-setup.md) guide to create a new Rust project
that builds this library with Circom proofs.

You must make sure that the `Cargo.toml` file has the `mopro-ffi/circom` feature enabled:

```toml
[features]
default = ["mopro-ffi/circom"]
```

## Witness Generation Functions

In order for the Mopro to be able to generate proofs for your chosen circom circuits, you need to provide a witness
generation function for each of the circuits you plan to use to generate proofs for. This function handles the witness
generation for your circuit. You can read more about witness for circom
circuits [here](https://docs.circom.io/background/background/#witness).

The function signature should be:

```rust
pub type WtnsFn = fn(HashMap<String, Vec<BigInt>>) -> Vec<BigInt>;
```

## Implementing the Witness Function

For simplicity, you can use the `witness!` macro provided by the `rust-witness` crate. This macro generates a witness
function for you given the circuit name. You can read more about the `witness!`
macro [here](https://github.com/vimwitch/rust-witness).

To use it, you must add first add the `rust-witness` crate to your `Cargo.toml` regular and build dependencies:

```toml
[dependencies]
# ...
rust-witness = { git = "https://github.com/vimwitch/rust-witness.git" }

[build-dependencies]
# ...
rust-witness = { git = "https://github.com/vimwitch/rust-witness.git" }
```

Then you need to add the following to the `build.rs` file to compile the circom circuits:

```rust
fn main() {
    // ...

    rust_witness::generate_witnesses!("path/to/circom/circuits/files");

    // ...
}
```

And then you can automatically generate the witness functions for all the circuits in the specified folder.

To do so, in the `lib.rs` file, you can add the following:

```rust
rust_witness::generate_witnesses!("your_circuit_name");
```

This will generate the witness function for the specified circuit
following [the naming convention here](https://github.com/vimwitch/rust-witness?tab=readme-ov-file#rust-witness)

## Setting the Circom Circuits

To set Circom circuits, you need to use the `set_circom_circuits!` macro provided by the `mopro-ffi` crate. This macro
should be called in the `lib.rs` file of your project, after the `mopro_ffi::app()` macro, and it should contain a list
of tuples, where the first element is the path to the `zkey` file and the second element is the witness generation
function.

For example:

```rust
mopro_ffi::set_circom_circuits! {
    ("your_circuit.zkey", your_circuit_wtns_gen_fn),
    ("another_circuit.zkey", another_circuit_wtns_gen_fn),
}
```

Under the hood, the `set_circom_circuits!` macro will generate a `get_circom_wtns_fn` function that will be used to get
the witness generation
function for a given circuit `zkey` file.

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

## Using the Library

After you have specified the circuits you want to use, you can follow the usual steps to build the library and use it in
your project.

### IOS

The Circom adapter exposes the following functions to be used in the IOS project:

```swift
// Generate a proof for a given circuit zkey, as well as the circuit inputs
// Make sure that the key was set in the Rust library
generateCircomProof(zkeyPath: zkeyPath, circuitInputs: inputs) -> GenerateProofResult

// Verify a proof for a given circuit zkey (arbitrary circuit)
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

### Android

The circom adapter exposes the equivalent functions and types to be used in the Android project. 


