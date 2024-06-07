#![allow(unused_variables)]

use std::collections::HashMap;

#[cfg(feature = "halo2")]
use mopro_core::middleware::halo2;
#[cfg(feature = "halo2")]
use mopro_core::middleware::halo2::deserialize_circuit_inputs;
use mopro_core::MoproError;

use crate::GenerateProofResult;

#[cfg(feature = "halo2")]
pub fn generate_halo2_proof2(
    circuit_inputs: HashMap<String, Vec<String>>,
) -> Result<GenerateProofResult, MoproError> {
    let circuit_inputs = deserialize_circuit_inputs(circuit_inputs);

    let (proof, inputs) = halo2::generate_halo2_proof2(circuit_inputs).unwrap();

    let serialized_proof =
        bincode::serialize(&proof).map_err(|e| MoproError::Halo2Error(e.to_string()))?;
    let serialized_inputs = bincode::serialize(&inputs).expect("Serialization of Inputs failed");

    Ok(GenerateProofResult {
        proof: serialized_proof,
        inputs: serialized_inputs,
    })
}

#[cfg(not(feature = "halo2"))]
pub fn generate_halo2_proof2(
    inputs: HashMap<String, Vec<String>>,
) -> Result<GenerateProofResult, MoproError> {
    Err(MoproError::Halo2Error("Project is compiled for Circom proving system. This function is currently not supported in Circom.".to_string()))
}

#[cfg(feature = "halo2")]
pub fn verify_halo2_proof2(proof: Vec<u8>, public_inputs: Vec<u8>) -> Result<bool, MoproError> {
    let deserialized_proof: halo2::SerializableProof =
        bincode::deserialize(&proof).map_err(|e| MoproError::Halo2Error(e.to_string()))?;
    let deserialized_inputs: halo2::SerializablePublicInputs =
        bincode::deserialize(&public_inputs).map_err(|e| MoproError::Halo2Error(e.to_string()))?;
    let is_valid = halo2::verify_halo2_proof2(deserialized_proof, deserialized_inputs).unwrap();
    Ok(is_valid)
}

#[cfg(not(feature = "halo2"))]
pub fn verify_halo2_proof2(proof: Vec<u8>, inputs: Vec<u8>) -> Result<bool, MoproError> {
    Err(MoproError::Halo2Error(
        "Project does not have Halo2 feature enabled".to_string(),
    ))
}
