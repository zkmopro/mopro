#![allow(unused_variables)]

#[cfg(feature = "halo2")]
pub(crate) use common::*;

/// Module that contains all the shared adapter functionality implemented for the Halo2 adapter.
/// As the adapter is only used when the `halo2` feature is enabled,
/// we make the compiler avoid the shared functions of module when the feature is not enabled.
#[cfg(feature = "halo2")]
mod common {
    use std::collections::HashMap;

    use mopro_core::middleware::halo2;
    use mopro_core::middleware::halo2::deserialize_circuit_inputs;
    use mopro_core::MoproError;

    use crate::GenerateProofResult;

    pub fn generate_proof_static(
        circuit_inputs: HashMap<String, Vec<String>>,
    ) -> Result<GenerateProofResult, MoproError> {
        let circuit_inputs = deserialize_circuit_inputs(circuit_inputs);

        let (proof, inputs) = halo2::generate_halo2_proof(circuit_inputs).unwrap();

        let serialized_proof =
            bincode::serialize(&proof).map_err(|e| MoproError::Halo2Error(e.to_string()))?;
        let serialized_inputs =
            bincode::serialize(&inputs).expect("Serialization of Inputs failed");

        Ok(GenerateProofResult {
            proof: serialized_proof,
            inputs: serialized_inputs,
        })
    }

    pub fn verify_proof_static(proof: Vec<u8>, public_inputs: Vec<u8>) -> Result<bool, MoproError> {
        let deserialized_proof: halo2::SerializableProof =
            bincode::deserialize(&proof).map_err(|e| MoproError::Halo2Error(e.to_string()))?;
        let deserialized_inputs: halo2::SerializablePublicInputs =
            bincode::deserialize(&public_inputs)
                .map_err(|e| MoproError::Halo2Error(e.to_string()))?;
        let is_valid = halo2::verify_halo2_proof(deserialized_proof, deserialized_inputs).unwrap();
        Ok(is_valid)
    }

    pub fn initialize_mopro() -> Result<(), MoproError> {
        panic!("Mopro Halo2 does not implement initialization yet.");
    }

    pub fn initialize_mopro_dylib(dylib_path: String) -> Result<(), MoproError> {
        panic!("Mopro Halo2 does not implement dylib initialization yet.");
    }

    #[cfg(test)]
    #[test]
    #[cfg(feature = "circom")]
    fn test_end_to_end() -> Result<(), MoproError> {
        // Paths to your wasm and zkey files
        let wasm_path =
            "./../mopro-core/examples/circom/multiplier2/target/multiplier2_js/multiplier2.wasm";
        let zkey_path = "./../mopro-core/examples/circom/multiplier2/target/multiplier2_final.zkey";

        // Create a new MoproCircom instance
        let mopro_circom = MoproCircom::new();

        // Step 1: Initialize
        let init_result = mopro_circom.initialize(zkey_path.to_string(), wasm_path.to_string());
        assert!(init_result.is_ok());

        let mut inputs = HashMap::new();
        let a = BigUint::from_str(
            "21888242871839275222246405745257275088548364400416034343698204186575808495616",
        )
        .unwrap();
        let b = BigUint::from(1u8);
        let c = a.clone() * b.clone();
        inputs.insert("a".to_string(), vec![a.to_string()]);
        inputs.insert("b".to_string(), vec![b.to_string()]);
        // output = [public output c, public input a]
        let expected_output = vec![Fr::from(c), Fr::from(a)];
        let circom_outputs = circom::serialization::SerializableInputs(expected_output);
        let serialized_outputs = circom::serialization::serialize_inputs(&circom_outputs);

        // Step 2: Generate Proof
        let generate_proof_result = mopro_circom.generate_proof(inputs)?;
        let serialized_proof = generate_proof_result.proof;
        let serialized_inputs = generate_proof_result.inputs;

        assert!(serialized_proof.len() > 0);
        assert_eq!(serialized_inputs, serialized_outputs);

        // Step 3: Verify Proof
        let is_valid =
            mopro_circom.verify_proof(serialized_proof.clone(), serialized_inputs.clone())?;
        assert!(is_valid);

        // Step 4: Convert Proof to Ethereum compatible proof
        let proof_calldata = crate::adapter::to_ethereum_proof(serialized_proof);
        let inputs_calldata = crate::adapter::to_ethereum_inputs(serialized_inputs);
        assert!(proof_calldata.a.x.len() > 0);
        assert!(inputs_calldata.len() > 0);

        Ok(())
    }
}
