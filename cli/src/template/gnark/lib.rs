#[cfg(not(target_arch = "wasm32"))]
mod gnark;
#[cfg(not(target_arch = "wasm32"))]
pub use gnark::{
    generate_gnark_proof, generate_gnark_plonk_proof,
    verify_gnark_proof, verify_gnark_plonk_proof,
    GnarkProofResult, GnarkPlonkProofResult,
};

#[cfg(test)]
#[cfg(not(target_arch = "wasm32"))]
mod gnark_tests {
    use crate::gnark::{
        generate_gnark_proof, generate_gnark_plonk_proof,
        verify_gnark_proof, verify_gnark_plonk_proof,
    };

    const R1CS_PATH: &str = "./test-vectors/gnark/cubic_circuit.r1cs";
    const GROTH16_PK_PATH: &str = "./test-vectors/gnark/cubic_circuit.pk";
    const GROTH16_VK_PATH: &str = "./test-vectors/gnark/cubic_circuit.vk";

    const SCS_PATH: &str = "./test-vectors/gnark/cubic_circuit_plonk.scs";
    const PLONK_PK_PATH: &str = "./test-vectors/gnark/cubic_circuit_plonk.pk";
    const PLONK_VK_PATH: &str = "./test-vectors/gnark/cubic_circuit_plonk.vk";

    const WITNESS_JSON: &str = r#"{"X": "3", "Y": "35"}"#;

    #[test]
    fn test_gnark_groth16_cubic_circuit() {
        let result = generate_gnark_proof(
            R1CS_PATH.to_string(),
            GROTH16_PK_PATH.to_string(),
            WITNESS_JSON.to_string(),
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
            GROTH16_VK_PATH.to_string(),
            proof_result,
        );
        assert!(valid.is_ok(), "Verification should not error");
        assert!(valid.unwrap(), "Proof should be valid");
    }

    #[test]
    fn test_gnark_plonk_cubic_circuit() {
        let result = generate_gnark_plonk_proof(
            SCS_PATH.to_string(),
            PLONK_PK_PATH.to_string(),
            WITNESS_JSON.to_string(),
        );
        assert!(result.is_ok(), "PLONK proof generation should succeed");

        let proof_result = result.unwrap();
        assert!(!proof_result.proof.is_empty(), "Proof should not be empty");
        assert!(
            !proof_result.public_inputs.is_empty(),
            "Public inputs should not be empty"
        );

        let valid = verify_gnark_plonk_proof(
            SCS_PATH.to_string(),
            PLONK_VK_PATH.to_string(),
            proof_result,
        );
        assert!(valid.is_ok(), "Verification should not error");
        assert!(valid.unwrap(), "Proof should be valid");
    }
}
