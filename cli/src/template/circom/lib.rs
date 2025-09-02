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
    use crate::circom::{generate_circom_proof, verify_circom_proof, ProofLib};

    #[test]
    fn test_multiplier2() {
        // Override the default witness function for this test
        crate::circom_register(
            "multiplier2_final.zkey",
            circom_prover::witness::WitnessFn::RustWitness(crate::multiplier2_witness),
        );

        let zkey_path = "./test-vectors/circom/multiplier2_final.zkey".to_string();
        let circuit_inputs = "{\"a\": 2, \"b\": 3}".to_string();
        let result = generate_circom_proof(zkey_path.clone(), circuit_inputs, ProofLib::Arkworks);
        assert!(result.is_ok());
        let proof = result.unwrap();
        assert!(verify_circom_proof(zkey_path, proof, ProofLib::Arkworks).is_ok());
    }
}
