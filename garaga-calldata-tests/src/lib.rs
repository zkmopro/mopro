//! Integration tests for Circom/Groth16 Garaga calldata helpers in the CLI template.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum MoproError {
    #[error("CircomError: {0}")]
    CircomError(String),
}

mod circom;

pub use circom::{generate_circom_groth16_garaga_calldata, CircomProof, CircomProofResult, G1, G2};

#[cfg(test)]
mod tests {
    use super::*;
    use circom::garaga_convert::{
        parse_biguint, public_inputs_to_biguint, to_groth16_proof_from_mopro, to_groth16_vk,
    };
    use circom::snarkjs_types::parse_snarkjs_vk_json;
    use circom::{CircomProof, CircomProofResult, G1, G2};
    use num_bigint::BigUint;
    use serde::Deserialize;

    /// Test-only: load SnarkJS `proof.json` fixtures into a [`CircomProof`].
    #[derive(Debug, Deserialize)]
    struct FixtureProof {
        pi_a: Vec<String>,
        pi_b: Vec<Vec<String>>,
        pi_c: Vec<String>,
        protocol: String,
        curve: String,
    }

    fn fixture_dir() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("fixtures/bn254")
    }

    fn read_fixture(name: &str) -> String {
        std::fs::read_to_string(fixture_dir().join(name)).unwrap()
    }

    fn load_fixture_proof_result() -> CircomProofResult {
        let proof: FixtureProof = serde_json::from_str(&read_fixture("proof.json")).unwrap();
        let inputs: Vec<String> = serde_json::from_str(&read_fixture("public.json")).unwrap();
        CircomProofResult {
            proof: CircomProof {
                a: G1 {
                    x: proof.pi_a[0].clone(),
                    y: proof.pi_a[1].clone(),
                    z: proof
                        .pi_a
                        .get(2)
                        .cloned()
                        .unwrap_or_else(|| "1".to_string()),
                },
                b: G2 {
                    x: proof.pi_b[0].clone(),
                    y: proof.pi_b[1].clone(),
                    z: proof
                        .pi_b
                        .get(2)
                        .cloned()
                        .unwrap_or_else(|| vec!["1".to_string(), "0".to_string()]),
                },
                c: G1 {
                    x: proof.pi_c[0].clone(),
                    y: proof.pi_c[1].clone(),
                    z: proof
                        .pi_c
                        .get(2)
                        .cloned()
                        .unwrap_or_else(|| "1".to_string()),
                },
                protocol: proof.protocol,
                curve: proof.curve,
            },
            inputs,
        }
    }

    #[test]
    fn test_parse_snarkjs_vk_json_valid() {
        let vk = parse_snarkjs_vk_json(&read_fixture("verification_key.json")).unwrap();
        assert_eq!(vk.ic.len(), vk.n_public + 1);
    }

    /// BN254 scalar-field order (Fr), decimal.
    fn bn254_scalar_field() -> BigUint {
        BigUint::parse_bytes(
            b"21888242871839275222246405745257275088548364400416034343698204186575808495617",
            10,
        )
        .unwrap()
    }

    /// BN254 base-field modulus (Fq), decimal.
    fn bn254_base_field() -> BigUint {
        BigUint::parse_bytes(
            b"21888242871839275222246405745257275088696311157297823662689037894645226208583",
            10,
        )
        .unwrap()
    }

    #[test]
    fn test_garaga_convert_bn254_from_mopro_fixtures() {
        let proof_result = load_fixture_proof_result();
        let vk = parse_snarkjs_vk_json(&read_fixture("verification_key.json")).unwrap();

        let garaga_proof = to_groth16_proof_from_mopro(
            &proof_result.proof.a.x,
            &proof_result.proof.a.y,
            &proof_result.proof.b.x,
            &proof_result.proof.b.y,
            &proof_result.proof.c.x,
            &proof_result.proof.c.y,
            &proof_result.inputs,
        )
        .unwrap();
        assert_eq!(
            garaga_proof.a.x.to_string(),
            "16867095230114469303111269582801754677348924111782514818746093562477643731718"
        );

        let garaga_vk = to_groth16_vk(&vk).unwrap();
        assert_eq!(garaga_vk.ic.len(), vk.n_public + 1);
    }

    #[test]
    fn test_parse_biguint_rejects_outside_base_field() {
        let outside = bn254_base_field().to_string();
        let err = parse_biguint(&outside).unwrap_err();
        assert!(err.contains("outside BN254 base field"));
    }

    #[test]
    fn test_parse_biguint_preserves_decimal_parse_errors() {
        let err = parse_biguint("not-a-number").unwrap_err();
        assert!(err.starts_with("invalid coordinate 'not-a-number':"));
    }

    #[test]
    fn test_public_inputs_to_biguint_rejects_outside_scalar_field() {
        let outside = bn254_scalar_field().to_string();
        let err = public_inputs_to_biguint(std::slice::from_ref(&outside)).unwrap_err();
        assert!(err.contains("outside BN254 scalar field"));
        assert!(err.contains(&outside));
    }

    #[test]
    fn test_public_inputs_to_biguint_preserves_decimal_parse_errors() {
        let err = public_inputs_to_biguint(&["not-a-number".to_string()]).unwrap_err();
        assert!(err.starts_with("invalid coordinate 'not-a-number':"));
    }

    #[test]
    fn test_public_inputs_to_biguint_allows_empty() {
        let converted = public_inputs_to_biguint(&[]).unwrap();
        assert!(converted.is_empty());
    }

    #[test]
    fn test_public_inputs_match_serialized_vec() {
        let inputs: Vec<String> = serde_json::from_str(&read_fixture("public.json")).unwrap();
        let round_trip: Vec<String> =
            serde_json::from_str(&serde_json::to_string(&inputs).unwrap()).unwrap();
        assert_eq!(inputs, round_trip);
    }

    #[test]
    fn test_generate_circom_groth16_garaga_calldata_bn254_golden() {
        let proof_result = load_fixture_proof_result();
        let vk = read_fixture("verification_key.json");
        let expected: Vec<String> =
            serde_json::from_str(&read_fixture("expected_garaga_calldata.json")).unwrap();

        let got = generate_circom_groth16_garaga_calldata(proof_result, vk).unwrap();
        assert_eq!(got, expected);
        assert_eq!(got.len(), 1936);
        assert_eq!(got[0], "1935");
    }

    #[test]
    fn test_generate_circom_groth16_garaga_calldata_rejects_oversized_public_inputs() {
        let mut proof_result = load_fixture_proof_result();
        proof_result.inputs.push("0".to_string());
        let vk = read_fixture("verification_key.json");

        let err = generate_circom_groth16_garaga_calldata(proof_result, vk).unwrap_err();
        match err {
            MoproError::CircomError(msg) => {
                assert!(msg.contains("public input count mismatch"));
            }
        }
    }

    #[test]
    fn test_generate_circom_groth16_garaga_calldata_rejects_public_input_outside_scalar_field() {
        let mut proof_result = load_fixture_proof_result();
        let outside = bn254_scalar_field().to_string();
        proof_result.inputs = vec![outside.clone()];
        let vk = read_fixture("verification_key.json");

        let err = generate_circom_groth16_garaga_calldata(proof_result, vk).unwrap_err();
        match err {
            MoproError::CircomError(msg) => {
                assert!(msg.contains("outside BN254 scalar field"));
                assert!(msg.contains(&outside));
            }
        }
    }
}
