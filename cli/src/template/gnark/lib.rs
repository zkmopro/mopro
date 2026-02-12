#[cfg(not(target_arch = "wasm32"))]
mod gnark;
#[cfg(not(target_arch = "wasm32"))]
pub use gnark::{generate_gnark_proof, verify_gnark_proof, GnarkProofResult};

#[cfg(test)]
#[cfg(not(target_arch = "wasm32"))]
mod gnark_tests {
    use crate::gnark::{generate_gnark_proof, verify_gnark_proof};

    const R1CS_PATH: &str = "./test-vectors/gnark/cubic_circuit.r1cs";
    const PK_PATH: &str = "./test-vectors/gnark/cubic_circuit.pk";
    const VK_PATH: &str = "./test-vectors/gnark/cubic_circuit.vk";

    #[test]
    fn test_gnark_cubic_circuit() {
        // x=3: x^3 + x + 5 = 27 + 3 + 5 = 35
        let witness_json = r#"{"X": "3", "Y": "35"}"#.to_string();

        let result = generate_gnark_proof(
            R1CS_PATH.to_string(),
            PK_PATH.to_string(),
            witness_json,
        );
        assert!(result.is_ok(), "Proof generation should succeed");

        let proof_result = result.unwrap();
        assert!(!proof_result.proof.is_empty(), "Proof should not be empty");
        assert!(
            !proof_result.public_inputs.is_empty(),
            "Public inputs should not be empty"
        );

        let valid = verify_gnark_proof(
            R1CS_PATH.to_string(),
            VK_PATH.to_string(),
            proof_result,
        );
        assert!(valid.is_ok(), "Verification should not error");
        assert!(valid.unwrap(), "Proof should be valid");
    }
}
