#![allow(unused_variables)]

use crate::{GenerateProofResult, MoproError};
use std::collections::HashMap;

type CircuitInputs = HashMap<String, Vec<Fp>>;

pub(crate) use halo2_proofs::halo2curves::bn256::{Bn256, Fr as Fp, G1Affine};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SerializableProof(pub Vec<u8>);

#[derive(Clone, Debug)]
pub struct SerializablePublicInputs(pub Vec<Fp>);

#[cfg(feature = "halo2")]
pub fn generate_halo2_proof(
    circuit_inputs: HashMap<String, Vec<String>>,
) -> Result<GenerateProofResult, MoproError> {
    panic!("stub implementation: TODO: copy logic from mopro-core to here");
    // let circuit_inputs = deserialize_circuit_inputs(circuit_inputs);

    // let (proof, inputs) = halo2::generate_halo2_proof(circuit_inputs).unwrap();

    // let serialized_proof =
    //     bincode::serialize(&proof).map_err(|e| MoproError::Halo2Error(e.to_string()))?;
    // let serialized_inputs = bincode::serialize(&inputs).expect("Serialization of Inputs failed");

    // Ok(GenerateProofResult {
    //     proof: serialized_proof,
    //     inputs: serialized_inputs,
    // })
}

#[cfg(feature = "halo2")]
pub fn verify_halo2_proof(proof: Vec<u8>, public_inputs: Vec<u8>) -> Result<bool, MoproError> {
    panic!("stub implementation: TODO: copy logic from mopro-core to here");
    // let deserialized_proof: halo2::SerializableProof =
    //     bincode::deserialize(&proof).map_err(|e| MoproError::Halo2Error(e.to_string()))?;
    // let deserialized_inputs: halo2::SerializablePublicInputs =
    //     bincode::deserialize(&public_inputs).map_err(|e| MoproError::Halo2Error(e.to_string()))?;
    // let is_valid = halo2::verify_halo2_proof(deserialized_proof, deserialized_inputs).unwrap();
    // Ok(is_valid)
}

#[cfg(feature = "halo2")]
#[test]
fn test_end_to_end() -> Result<(), MoproError> {
    // We by default compile the Fibonacci Halo2 Circuit
    // TODO - For the future we should consider a stateful circuit to change the keys on the fly.

    let mut inputs = HashMap::new();
    let out = 55u64;
    inputs.insert("out".to_string(), vec![out.to_string()]);

    let expected_output = vec![Fr::from(1), Fr::from(1), Fr::from(out)];
    let expected_output_bytes = bincode::serialize(&SerializablePublicInputs(expected_output))
        .expect("Serialization of Output Expected bytes failed");

    // Step 2: Generate Proof
    let generate_proof_result = generate_halo2_proof(inputs)?;
    let serialized_proof = generate_proof_result.proof;
    let serialized_inputs = generate_proof_result.inputs;

    assert!(serialized_proof.len() > 0);
    assert_eq!(serialized_inputs, expected_output_bytes);

    // Step 3: Verify Proof
    let is_valid = verify_halo2_proof(serialized_proof.clone(), serialized_inputs.clone())?;
    assert!(is_valid);

    Ok(())
}
