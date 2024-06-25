#[cfg(all(feature = "halo2", feature = "circom"))]
compile_error!(
    "Cannot enable both `halo2` and `circom` features at the same time
Please enable only one of them"
);

mod circom;
mod halo2;

// We require that each adapter implements the same set of default functions
// As well as allow an adapter to export its own unique functions as long as
// There is as default (`dummy`) implementation for when the adapter is not enabled.
#[cfg(feature = "circom")]
use circom as adapter;
#[cfg(feature = "halo2")]
use halo2 as adapter;

use std::collections::HashMap;

use mopro_core::MoproError;

// A set of shared functions that each adapter is required to implement.
// We wrap these functions in another layer of abstraction to enforce consistent types.
// Adapter does not need to implement the `dummy` version as another adapter will provide it.

pub fn initialize_mopro() -> Result<(), MoproError> {
    adapter::initialize_mopro()
}

pub fn initialize_mopro_dylib(dylib_path: String) -> Result<(), MoproError> {
    adapter::initialize_mopro_dylib(dylib_path)
}

pub fn generate_proof_static(
    inputs: HashMap<String, Vec<String>>,
) -> Result<GenerateProofResult, MoproError> {
    adapter::generate_proof_static(inputs)
}

pub fn verify_proof_static(proof: Vec<u8>, public_input: Vec<u8>) -> Result<bool, MoproError> {
    adapter::verify_proof_static(proof, public_input)
}

// A set of unique functions that each adapter can implement, which we directly re-export.
// The adapter must provide a default (`dummy`) implementation for when the adapter is not enabled.

pub use circom::{arkworks_pippenger, metal_msm};
pub use circom::{to_ethereum_inputs, to_ethereum_proof};
pub use circom::{BenchmarkResult, MoproCircom, ProofCalldata, G1, G2};

#[derive(Debug)]
pub enum FFIError {
    MoproError(mopro_core::MoproError),
    SerializationError(String),
}

#[derive(Debug, Clone)]
pub struct GenerateProofResult {
    pub proof: Vec<u8>,
    pub inputs: Vec<u8>,
}

impl From<MoproError> for FFIError {
    fn from(error: MoproError) -> Self {
        FFIError::MoproError(error)
    }
}

// Test functions (TODO - consider removing)

fn add(a: u32, b: u32) -> u32 {
    a + b
}

fn hello() -> String {
    "Hello World from Rust".to_string()
}

// TODO: Remove me
// UniFFI expects String type
// See https://mozilla.github.io/uniffi-rs/udl/builtin_types.html
// fn run_example(wasm_path: String, r1cs_path: String) -> Result<(), MoproError> {
//     circom::run_example(wasm_path.as_str(), r1cs_path.as_str())
// }

uniffi::include_scaffolding!("mopro");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
