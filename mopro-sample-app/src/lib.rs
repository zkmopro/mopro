// This is required to initiate the Mopro Bindings setup
mopro_ffi::setup_mopro_ffi!();

use mopro_ffi::{GenerateProofResult, Halo2CircuitBindings, MoproHalo2};
use std::collections::HashMap;

// We can derive Halo2 Mopro bindings as long as the circuit implements `MoproHalo2` trait
#[derive(Halo2CircuitBindings)]
struct FibonacciCircuit {
    pub a: u64,
    pub b: u64,
}

// Implementation of the `MoproHalo2` trait for the Fibonacci circuit
impl MoproHalo2 for FibonacciCircuit {
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
