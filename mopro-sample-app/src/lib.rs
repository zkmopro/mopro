// This will be placed into a `setup!()` macro
uniffi::setup_scaffolding!();
// uniffi::use_udl_error!(mopro_ffi, mopro_ffi::MoproError); // TODO - solve the issue
uniffi::use_udl_record!(mopro_ffi, GenerateProofResult);

use mopro_ffi::{GenerateProofResult, Halo2Mopro, MoproError, WtnsFn};
use std::collections::{HashMap, VecDeque};
use std::fmt::Display;

// TODO - replace with MoproError - this will all be removed
#[derive(Debug, uniffi::Error)]
pub enum MoproError1 {
    CircomError(String),
    Halo2Error(String),
}

impl Display for MoproError1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MoproError1::CircomError(e) => write!(f, "CircomError: {}", e),
            MoproError1::Halo2Error(e) => write!(f, "Halo2Error: {}", e),
        }
    }
}

// Here we generate all binding functions for the circuit
#[derive(Halo2Mopro)]
struct FibonacciCircuit {
    pub a: u64,
    pub b: u64,
}

// Here we implement all of the logic for required functions
impl Halo2Mopro for FibonacciCircuit {
    fn prove(input: HashMap<String, Vec<String>>) -> Result<GenerateProofResult, MoproError> {
        let a = input.get("a").unwrap();
        let b = input.get("b").unwrap();
        let a = a[0].parse::<u64>().unwrap();
        let b = b[0].parse::<u64>().unwrap();

        let _ = FibonacciCircuit { a, b };

        let proof = Vec::new();
        let public_inputs = Vec::new();

        Ok(GenerateProofResult {
            proof,
            inputs: public_inputs,
        })
    }

    fn verify(proof: Vec<u8>, public_inputs: Vec<u8>) -> Result<bool, MoproError> {
        Ok(true)
    }
}
