// --- Circom Example of using groth16 proving and verifying circuits ---

// Module containing the Circom circuit logic (Multiplier2)
#[macro_use]
mod circom;

rust_witness::witness!(multiplier2);
rust_witness::witness!(multiplier2bls);
witnesscalc_adapter::witness!(multiplier2_wc);

set_circom_circuits! {
    ("multiplier2_final.zkey", circom_prover::witness::WitnessFn::RustWitness(multiplier2_witness)),
    ("multiplier2_bls_final.zkey", circom_prover::witness::WitnessFn::RustWitness(multiplier2bls_witness)),
    ("multiplier2_wc_final.zkey", circom_prover::witness::WitnessFn::WitnessCalc(multiplier2_wc_witness)),
}

#[cfg(test)]
mod circom_tests {
    use crate::circom::{generate_circom_proof, verify_circom_proof, ProofLib};

    const ZKEY_PATH: &str = "./test-vectors/circom/multiplier2_final.zkey";

    #[test]
    fn test_multiplier2() {
        let circuit_inputs = "{\"a\": 2, \"b\": 3}".to_string();
        let result =
            generate_circom_proof(ZKEY_PATH.to_string(), circuit_inputs, ProofLib::Arkworks);
        assert!(result.is_ok());
        let proof = result.unwrap();
        assert!(verify_circom_proof(ZKEY_PATH.to_string(), proof, ProofLib::Arkworks).is_ok());
    }
}
