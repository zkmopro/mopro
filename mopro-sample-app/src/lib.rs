// This is required to initiate the Mopro Bindings setup
mopro::setup_mopro!();

/// Sample Halo2 circuit
pub mod halo2;

use crate::halo2::deserialize_circuit_inputs;
use std::collections::HashMap;
use std::time::Instant;

struct FibonacciCircuit {}

/// The Halo2 circuit must implementation of the [`mopro::MoproHalo2`] to generate the bindings
impl mopro::MoproHalo2 for FibonacciCircuit {
    fn prove(
        input: HashMap<String, Vec<String>>,
    ) -> Result<mopro::GenerateProofResult, mopro::MoproError> {
        let circuit_inputs = deserialize_circuit_inputs(input).map_err(|e| {
            mopro::MoproError::Halo2Error(format!("Failed to deserialize circuit inputs: {}", e))
        })?;

        let start = Instant::now();

        println!("Proving the circuit with inputs: {:?}", circuit_inputs);

        let (proof, inputs) = halo2::generate_halo2_proof(circuit_inputs).map_err(|e| {
            mopro::MoproError::Halo2Error(format!("Failed to generate the proof: {}", e))
        })?;

        let duration = start.elapsed();
        println!("Proving time: {:?}", duration);

        let serialized_inputs = bincode::serialize(&inputs).map_err(|e| {
            mopro::MoproError::Halo2Error(format!("Serialisation of Inputs failed: {}", e))
        })?;

        Ok(mopro::GenerateProofResult {
            proof,
            inputs: serialized_inputs,
        })
    }

    fn verify(proof: Vec<u8>, public_inputs: Vec<u8>) -> Result<bool, mopro::MoproError> {
        let deserialized_inputs: halo2::SerializablePublicInputs =
            bincode::deserialize(&public_inputs)
                .map_err(|e| mopro::MoproError::Halo2Error(e.to_string()))?;

        let start = Instant::now();

        let is_valid = halo2::verify_halo2_proof(proof, deserialized_inputs).unwrap();

        let duration = start.elapsed();
        println!("Verification time: {:?}", duration);

        Ok(is_valid)
    }
}

// Generate the Halo2 bindings for the Fibonacci circuit
mopro::mopro_halo2_circuit!(FibonacciCircuit);

// Sample Circom circuit

// Generate the Circom bindings for the Multiplier circuit
mopro::mopro_circom_circuit!(multiplier3);
