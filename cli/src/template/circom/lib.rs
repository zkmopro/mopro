// --- Circom Example of using groth16 proving and verifying circuits ---

// Module containing the Circom circuit logic (Multiplier2)

rust_witness::witness!(multiplier2);

mopro_ffi::set_circom_circuits! {
    ("multiplier2_final.zkey", mopro_ffi::witness::WitnessFn::RustWitness(multiplier2_witness))
}

#[cfg(test)]
mod circom_tests {
    use super::*;

    #[test]
    fn test_multiplier2() {
        let zkey_path = "./test-vectors/circom/multiplier2_final.zkey".to_string();
        let circuit_inputs = "{\"a\": 2, \"b\": 3}".to_string();
        let result = generate_circom_proof(zkey_path.clone(), circuit_inputs, ProofLib::Arkworks);
        assert!(result.is_ok());
        let proof = result.unwrap();
        assert!(verify_circom_proof(zkey_path, proof, ProofLib::Arkworks).is_ok());
    }
}
