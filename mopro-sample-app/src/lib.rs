// This is required to initiate the Mopro Bindings setup
mopro_ffi::setup_mopro_ffi!();

// Sample Halo2 circuit

use mopro_ffi::{mopro_circom_circuit, mopro_halo2_circuit, GenerateProofResult, MoproHalo2};
use std::collections::HashMap;

// A sample Halo2 circuit
struct FibonacciCircuit {
    pub _a: u64,
    pub _b: u64,
}

/// The Halo2 circuit must implementation of the [`MoproHalo2`] to generate the bindings
impl MoproHalo2 for FibonacciCircuit {
    fn prove(input: HashMap<String, Vec<String>>) -> Result<GenerateProofResult, MoproError> {
        let a = input.get("a").unwrap();
        let b = input.get("b").unwrap();
        let a = a[0].parse::<u64>().unwrap();
        let b = b[0].parse::<u64>().unwrap();

        let _ = FibonacciCircuit { _a: a, _b: b };

        let proof = Vec::new();
        let public_inputs = Vec::new();

        Ok(GenerateProofResult {
            proof,
            inputs: public_inputs,
        })
    }

    fn verify(_proof: Vec<u8>, _public_inputs: Vec<u8>) -> Result<bool, MoproError> {
        Ok(true)
    }
}

// Generate the Halo2 bindings for the Fibonacci circuit
mopro_halo2_circuit!(FibonacciCircuit);

// Sample Circom circuit

// Generate the Circom bindings for the Multiplier circuit
mopro_circom_circuit!(multiplier3);
