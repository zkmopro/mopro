use std::sync::RwLock;

use mopro_core::middleware::circom;
use mopro_core::MoproError;
mod utils;

//use rand::rngs::ThreadRng;

#[derive(Debug)]
pub enum FFIError {
    MoproError(mopro_core::MoproError),
    SerializationError(String),
}

#[derive(Debug, Clone)]
pub struct GenerateProofResult {
    pub proof: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct SetupResult {
    pub provingKey: Vec<u8>,
    pub inputs: Vec<u8>,
}

impl From<mopro_core::MoproError> for FFIError {
    fn from(error: mopro_core::MoproError) -> Self {
        FFIError::MoproError(error)
    }
}

pub struct MoproCircom {
    state: RwLock<circom::CircomState>,
}

// TODO: Use setup, prove and verify functions from mopro_core

// TODO: Use FFIError::SerializationError instead
impl MoproCircom {
    pub fn new() -> Self {
        Self {
            state: RwLock::new(circom::CircomState::new()),
        }
    }

    pub fn setup(&self, wasm_path: String, r1cs_path: String) -> Result<SetupResult, MoproError> {
        let mut state_guard = self.state.write().unwrap();
        let (pk, inputs) = state_guard.setup(wasm_path.as_str(), r1cs_path.as_str())?;
        Ok(SetupResult {
            provingKey: circom::serialize_proving_key(&pk),
            inputs: circom::serialize_inputs(&inputs),
        })
    }

    pub fn generate_proof(&self) -> Result<GenerateProofResult, MoproError> {
        let state_guard = self.state.read().unwrap();
        let proof = state_guard.generate_proof()?;
        Ok(GenerateProofResult {
            proof: circom::serialize_proof(&proof),
        })
    }

    pub fn verify_proof(&self, proof: Vec<u8>, public_input: Vec<u8>) -> Result<bool, MoproError> {
        let state_guard = self.state.read().unwrap();
        let deserialized_proof = circom::deserialize_proof(proof);
        let deserialized_public_input = circom::deserialize_inputs(public_input);
        let is_valid = state_guard.verify_proof(deserialized_proof, deserialized_public_input)?;
        Ok(is_valid)
    }
}

fn add(a: u32, b: u32) -> u32 {
    a + b
}

fn hello() -> String {
    "Hello World from Rust".to_string()
}

// XXX: Do we need this?
pub fn init_circom_state() -> Result<(), MoproError> {
    //let mut circom_state = circom::CircomState::new();
    Ok(())
}

// UniFFI expects String type
// See https://mozilla.github.io/uniffi-rs/udl/builtin_types.html
fn run_example(wasm_path: String, r1cs_path: String) -> Result<(), MoproError> {
    circom::run_example(wasm_path.as_str(), r1cs_path.as_str())
}

uniffi::include_scaffolding!("mopro");

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
        let wasm_path =
            "../mopro-core/examples/circom/target/multiplier2_js/multiplier2.wasm".to_string();
        let r1cs_path = "../mopro-core/examples/circom/target/multiplier2.r1cs".to_string();
        match run_example(wasm_path, r1cs_path) {
            Ok(_) => println!("OK"),
            Err(e) => println!("Error: {}", e),
        }
    }

    #[test]
    fn it_sets_up_mopro_circom() {
        let mopro_circom = MoproCircom::new();

        let wasm_path =
            "./../mopro-core/examples/circom/target/multiplier2_js/multiplier2.wasm".to_string();
        let r1cs_path = "./../mopro-core/examples/circom/target/multiplier2.r1cs".to_string();

        match mopro_circom.setup(wasm_path, r1cs_path) {
            Ok(setup_result) => {
                assert!(
                    !setup_result.provingKey.is_empty(),
                    "Proving key should not be empty"
                );
                assert!(
                    !setup_result.inputs.is_empty(),
                    "Inputs should not be empty"
                );
            }
            Err(e) => panic!("Setup failed with error: {:?}", e),
        }
    }
}
