// --- Circom Example of using groth16 proving and verifying circuits ---

// Module containing the Circom circuit logic (Multiplier2)
#[macro_use]
mod circom;

rust_witness::witness!(multiplier2);

set_circom_circuits! {
    ("multiplier2_final.zkey", circom_prover::witness::WitnessFn::RustWitness(multiplier2_witness))
}

#[cfg(test)]
mod circom_tests {
    use super::*;

    #[test]
    fn test_multiplier2() {
        let zkey_path = "./test-vectors/circom/multiplier2_final.zkey".to_string();
        let circuit_inputs = "{\"a\": 2, \"b\": 3}".to_string();
        let result = circom::generate_circom_proof(zkey_path.clone(), circuit_inputs, circom::ProofLib::Arkworks);
        assert!(result.is_ok());
        let proof = result.unwrap();
        assert!(circom::verify_circom_proof(zkey_path, proof, circom::ProofLib::Arkworks).is_ok());
    }
}
