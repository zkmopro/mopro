# Halo2 Adapter

Mopro supports Halo2 circuits. To develop with Halo2 you will need to be familiar with the Halo2 library and how to
generate the proving and verifying keys for your circuits. You can read more about Halo2 in
the [Halo2 documentation](https://zcash.github.io/halo2/).

This adapter is designed to work with both the Zcash Halo2 library and the PSE's Halo2 fork.

## Sample Project

You can find a sample project that demonstrates how to use the Halo2
adapter [here](https://github.com/ElusAegis/halo2-rsa-mopro).

## Setup the rust project

You can mostly follow the instructions in the [Rust Setup](/getting-started/rust-setup.md) guide to create a new Rust
project
that builds this library with Circom proofs.

However, there are some differences in the `Cargo.toml` file. You must make sure that the `mopro-ffi/halo` feature is
enabled:

```toml
[features]
default = ["mopro-ffi/halo"]
```

Furthermore, you can delete the `rust-witness` regular and build dependency from the `Cargo.toml` file, as it is not
needed for Halo2 circuits,
as well as delete the `rust_witness::generate_witnesses!("...")` macro from the `lib.rs` file.

## Implementing the Halo2 Circuit

The Halo2 adapter was designed to put the least amount of restrictions on the user as possible. This means that you can
implement your Halo2 circuits in any way you see fit. However, there are some conventions that you should follow to make
the integration with Mopro work.

### Proving Function

When generating a proof for a Halo2 circuit, the Mopro will do a call to the proving function that you provide. This
function should have the following signature:

```rust
pub type Halo2ProveFn =
fn(&str, &str, HashMap<String, Vec<String>>) -> Result<GenerateProofResult, Box<dyn std::error::Error>>;
```

Where the first two arguments are the path to the `srs` and `proving` key files, and the second argument is a map of the
inputs for the circuit.

It is then your responsibility to load the keys and the circuit, as well as to deserialize the inputs and generate the
proof.
You can use any serialization method you want, as long as you can serialize and deserialize the inputs on the target
platform.

The result of the function should be a `GenerateProofResult` struct, which contains the proof and the public inputs in
the form of `Vec<u8>`. It is up to you to serialize the proof and the public inputs in a way that you can deserialize.

You can find an example of a proving function in
the [Halo2 Fibonacci sample project](https://github.com/ElusAegis/halo2-fibonacci-sample/blob/main/src/lib.rs).

### Verifying Function

When verifying a proof for a Halo2 circuit, the Mopro will do a call to the verifying function that you provide. This
function should have the following signature:

```rust
pub type Halo2VerifyFn = fn(&str, &str, Vec<u8>, Vec<u8>) -> Result<bool, Box<dyn std::error::Error>>;
```

Where the first two arguments are the path to the `srs` and `verifying` key files, and the last two arguments are the
proof and the public inputs.

It is then your responsibility to load the keys and the circuit, as well as to deserialize the proof and the public
inputs and verify the proof. Make sure that your deserialization method is compatible with the serialization method you
used in the proving function.

The result of the function should be a `bool`, which indicates whether the proof is valid or not.

You can find an example of a verifying function in
the [Halo2 Fibonacci sample project](https://github.com/ElusAegis/halo2-fibonacci-sample/blob/main/src/lib.rs).

### Setting the Halo2 Circuits

To set the Halo2 circuits in your project, you need to use the `set_halo2_circuits!` macro. This macro should be called
in the `lib.rs` file of your project, after the `mopro_ffi::app()` macro, and it should contain a list of tuples, where
each tuple contains the name to the proving key file, the proving function, the name to the verifying key file, and the
verifying function.

For example:

```rust
mopro_ffi::set_halo2_circuits! {
    ("fibonacci_pk.bin", halo2_fibonacci::prove, "fibonacci_vk.bin", halo2_fibonacci::verify),
}
```

Under the hood, the `set_halo2_circuits!` macro will generate two functions called `get_halo2_proving_circuit` and
`get_halo2_verifying_circuit` that will be used by the Mopro to call the proving and verifying functions respectively
for
each circuit based on the provided proving or verifying key files.

### Manual Configuration

You can optionally set only the proving or verifying function for a circuit by manually setting
the `get_halo2_proving_circuit`
or `get_halo2_verifying_circuit` functions in the `lib.rs` file. However, this is exclusive with
the `set_halo2_circuits!` macro, so you can't use both in the same project.

For example:

```rust
fn get_halo2_proving_circuit(circuit: &str) -> Result<Halo2ProveFn, MoproError> {
    match circuit {
        "fibonacci" => Ok(halo2_fibonacci::prove),
        _ => Err(MoproError::CircuitNotFound),
    }
}
```

```rust
fn get_halo2_verifying_circuit(circuit: &str) -> Result<Halo2VerifyFn, MoproError> {
    match circuit {
        "fibonacci" => Ok(halo2_fibonacci::verify),
        _ => Err(MoproError::CircuitNotFound),
    }
}
```

This might be useful if you want to have more control over the proving and verifying functions for each circuit or
if you want to only add the proving or verifying function for a circuit.

## Using the Library

After you have specified the circuits you want to use, you can follow the usual steps to build the library and use it
in your project.

### IOS

The Halo2 adapter exposes the following functions to be used in the IOS project:

```swift
// Generate a proof for a Halo2 circuit given the srs and proving key files, as well as the circuit inputs
// Make sure that the key was set in the Rust library
generateHalo2Proof(srsPath: srsPath, pkPath: pkPath, circuitInputs: inputs) -> GenerateProofResult

// Verify a proof for a Halo2 circuit given the srs and verifying key files, as well as the proof and public inputs
// Make sure that the key was set in the Rust library
verifyHalo2Proof(srsPath: srsPath, vkPath: vkPath, proof: generateProofResult.proof, publicInput: generateProofResult.inputs) -> Bool
```

As well as the following types:

```swift
public struct GenerateProofResult {
    public var proof: Data
    public var inputs: Data
}
```

### Android

The circom adapter exposes the equivalent functions and types to be used in the Android project. 


