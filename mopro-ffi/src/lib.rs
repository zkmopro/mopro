use std::error;

use mopro_core::middleware::circom;
use mopro_core::MoproError;

mod utils;

#[derive(Debug)]
pub enum FFIError {
    MoproError(mopro_core::MoproError),
    SerializationError(String),
}

impl From<mopro_core::MoproError> for FFIError {
    fn from(error: mopro_core::MoproError) -> Self {
        FFIError::MoproError(error)
    }
}

// name of the .udl file
uniffi::include_scaffolding!("mopro_uniffi");

fn add(a: u32, b: u32) -> u32 {
    a + b
}

fn hello() -> String {
    "Hello World from Rust".to_string()
}

// TODO: Use setup, prove and verify functions from mopro_core

// Result<( ProvingKey<Bn254>, CircomCircuit<Bn254>, ThreadRng, Vec<<Bn254 as Pairing>::ScalarField>,
fn setup(wasm_path: String, r1cs_path: String) -> Result<(), MoproError> {
    let res = circom::setup(wasm_path.as_str(), r1cs_path.as_str())?;

    // TODO: Implement FFI-friendly types and return this
    // Result<( ProvingKey<Bn254>, CircomCircuit<Bn254>, ThreadRng, Vec<<Bn254 as Pairing>::ScalarField>,
    let (pk, circuit, rng, s) = res;

    Err(MoproError::CircomError("Not implemented yet".to_string()))
}

// UniFFI expects String type
// See https://mozilla.github.io/uniffi-rs/udl/builtin_types.html
fn run_example(wasm_path: String, r1cs_path: String) -> Result<(), MoproError> {
    circom::run_example(wasm_path.as_str(), r1cs_path.as_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn run_example_ok_or_err() {
        let wasm_path = "./examples/circom/target/multiplier2_js/multiplier2.wasm".to_string();
        let r1cs_path = "./examples/circom/target/multiplier2.r1cs".to_string();
        match run_example(wasm_path, r1cs_path) {
            Ok(_) => println!("OK"),
            Err(e) => println!("Error: {}", e),
        }
    }
}
