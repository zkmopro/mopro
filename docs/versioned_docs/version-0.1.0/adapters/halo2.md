# Halo2 Adapter

Mopro supports the use of Halo2 circuits, allowing for both the Halo2 library from Zcash and the PSE's Halo2 fork. To effectively work with Halo2 circuits in Mopro, you will need to understand how to generate proving and verifying keys as well as how Halo2 circuits work, and have some experience in Rust. For more details, please refer to the [Halo2 documentation](https://zcash.github.io/halo2/).

## Samples

Explore how the Halo2 adapter is implemented by checking out this [Sample Mopro Halo2-Adapter Project](https://github.com/zkmopro/halo2-app) or the [test-e2e](https://github.com/zkmopro/mopro/tree/main/test-e2e) where we maintain (and test) each adapter.

## Setting Up the Rust Project

You can start by following the general instructions in the [Rust Setup Guide](/setup/rust-setup.md) to create a new Rust project for building libraries with Circom proofs. However, you will need to perform these specific adjustments for Halo2:

In your `Cargo.toml` file, ensure the `halo2` feature is activated for `mopro-ffi`:

```toml
[features]
default = ["mopro-ffi/halo2"]
```

Then, remove the `rust-witness` dependency from `[dependencies]` and `[build-dependencies]` as it is unnecessary for Halo2 circuits.
Likewise, remove the `rust_witness::transpile::transpile_wasm!` macro call from the `build.rs` file and any `rust_witness::witness!` and `mopro_ffi::set_circom_circuits!` calls from the `lib.rs`.

## Implementing the Halo2 Circuit

The design of the Halo2 adapter minimizes restrictions, allowing flexibility in how you implement your circuits while following a few conventions to ensure compatibility with Mopro.

### Proving Function

When generating a proof for a Halo2 circuit, the Mopro will do a call to the proving function that you provide. This function should have the following signature:

```rust
pub type Halo2ProveFn = fn(&str, &str, HashMap<String, Vec<String>>) -> Result<GenerateProofResult, Box<dyn std::error::Error>>;
```

The first two arguments are the path to the `srs` and `proving` key files, and the third argument is a map of the inputs for the circuit.

It is then your responsibility to load the keys from the path and set up the circuit, as well as to deserialize the inputs and generate the proof. You can use any serialization method you want, as long as you can serialize and deserialize the inputs on the target platform.

The result of the function should be a `GenerateProofResult` struct, which contains the proof and the public inputs in the form of `Vec<u8>`. It is up to you to serialize the proof and the public inputs in a way that you can deserialize.

You can find an example of a proving function in the [Halo2 Fibonacci circuit sample](https://github.com/ElusAegis/halo2-fibonacci-sample/blob/main/src/lib.rs).

### Verifying Function

When verifying a proof for a Halo2 circuit, the Mopro will do a call to the verifying function that you provide. This function should have the following signature:

```rust
pub type Halo2VerifyFn = fn(&str, &str, Vec<u8>, Vec<u8>) -> Result<bool, Box<dyn std::error::Error>>;
```

The first two arguments are the path to the `srs` and `verifying` key files, and the last two arguments are the serialised proof and the public inputs.

It is then your responsibility to load the keys from the path and set up the circuit, as well as to deserialize the proof and the public inputs and verify the proof. 
Make sure that your deserialization method is compatible with the serialization method you used in the proving function.

The result of the function should be a `bool`, which indicates whether the proof is valid or not.

You can find an example of a verifying function in the [Halo2 Fibonacci sample project](https://github.com/ElusAegis/halo2-fibonacci-sample/blob/main/src/lib.rs).

### Setting the Halo2 Circuits

To set the Halo2 circuits in your project, you need to use the `set_halo2_circuits!` macro. This macro should be called in the `lib.rs` file of your project, after the `mopro_ffi::app()` macro, and it should contain a list of tuples, where each tuple contains the name to the proving key file, the proving function, the name to the verifying key file, and the verifying function.

For example:

```rust
mopro_ffi::set_halo2_circuits! {
    ("plonk_fibonacci_pk.bin", plonk_fibonacci::prove, "plonk_fibonacci_vk.bin", plonk_fibonacci::verify),
}
```

Under the hood, the `set_halo2_circuits!` macro will generate two functions called `get_halo2_proving_circuit` and `get_halo2_verifying_circuit` that will be used by the Mopro to select and call the proving and verifying functions respectively for each circuit based on the provided proving or verifying key files.

### Manual Configuration

You can optionally set only the proving or verifying function for a circuit by manually setting the `get_halo2_proving_circuit` and `get_halo2_verifying_circuit` functions in the `lib.rs` file. However, this is exclusive with the `set_halo2_circuits!` macro, so you can't use both in the same project. Also, you must implement both functions, even if you only want to use one of them.

For example:

```rust
fn get_halo2_proving_circuit(circuit: &str) -> Result<Halo2ProveFn, MoproError> {
    match circuit {
        "plonk_fibonacci_pk.bin" => Ok(plonk_fibonacci::prove),
        _ => Err(MoproError::CircuitNotFound),
    }
}

fn get_halo2_verifying_circuit(circuit: &str) -> Result<Halo2VerifyFn, MoproError> {
    match circuit {
        "fibonacci_vk.bin" => Ok(plonk_fibonacci::verify),
        _ => Err(MoproError::CircuitNotFound),
    }
}
```

This might be useful if you want to have more control over the proving and verifying functions for each circuit or if you want to only add the proving or verifying function for a circuit.

## Using the Library

After you have specified the circuits you want to use, you can follow the usual steps to build the library and use it in your project.

### iOS API

The Halo2 adapter exposes the following functions to be used in the iOS project:

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

### Android API

The Halo2 adapter exposes the equivalent functions and types to be used in the Android project. 


