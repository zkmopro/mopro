//! Integration tests for Circom/Groth16 Garaga calldata helpers in the CLI template.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum MoproError {
    #[error("CircomError: {0}")]
    CircomError(String),
}

mod circom;

pub use circom::{
    generate_circom_groth16_garaga_calldata,
    generate_circom_groth16_garaga_calldata_from_proof_result,
};

#[cfg(test)]
mod tests {
    use super::*;
    use circom::garaga_convert::{to_groth16_proof, to_groth16_vk};
    use circom::snarkjs_types::{
        parse_snarkjs_proof_json, parse_snarkjs_public_json, parse_snarkjs_vk_json, SnarkJsProof,
    };
    use circom::{CircomProof, CircomProofResult, G1, G2};

    fn fixture_dir() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../cli/src/template/init/test-vectors/circom/garaga/bn254")
    }

    fn read_fixture(name: &str) -> String {
        std::fs::read_to_string(fixture_dir().join(name)).unwrap()
    }

    #[test]
    fn test_parse_snarkjs_proof_json_valid() {
        parse_snarkjs_proof_json(&read_fixture("proof.json")).unwrap();
    }

    #[test]
    fn test_parse_snarkjs_public_json_valid() {
        let inputs = parse_snarkjs_public_json(&read_fixture("public.json")).unwrap();
        assert!(!inputs.is_empty());
    }

    #[test]
    fn test_parse_snarkjs_vk_json_valid() {
        let vk = parse_snarkjs_vk_json(&read_fixture("verification_key.json")).unwrap();
        assert_eq!(vk.ic.len(), vk.n_public + 1);
    }

    #[test]
    fn test_parse_snarkjs_proof_rejects_non_bn128() {
        let mut json = read_fixture("proof.json");
        json = json.replace("\"bn128\"", "\"bls12381\"");
        let err = parse_snarkjs_proof_json(&json).unwrap_err();
        assert!(err.contains("unsupported curve"));
    }

    #[test]
    fn test_garaga_convert_bn254_fixtures() {
        let proof = parse_snarkjs_proof_json(&read_fixture("proof.json")).unwrap();
        let public = parse_snarkjs_public_json(&read_fixture("public.json")).unwrap();
        let vk = parse_snarkjs_vk_json(&read_fixture("verification_key.json")).unwrap();

        let garaga_proof = to_groth16_proof(&proof, &public).unwrap();
        assert_eq!(
            garaga_proof.a.x.to_string(),
            "16867095230114469303111269582801754677348924111782514818746093562477643731718"
        );

        let garaga_vk = to_groth16_vk(&vk).unwrap();
        assert_eq!(garaga_vk.ic.len(), vk.n_public + 1);
    }

    fn snarkjs_proof_to_circom_proof(proof: &SnarkJsProof) -> CircomProof {
        CircomProof {
            a: G1 {
                x: proof.pi_a[0].clone(),
                y: proof.pi_a[1].clone(),
                z: proof.pi_a.get(2).cloned().unwrap_or_else(|| "1".to_string()),
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
                z: proof.pi_c.get(2).cloned().unwrap_or_else(|| "1".to_string()),
            },
            protocol: proof.protocol.clone(),
            curve: proof.curve.clone(),
        }
    }

    #[test]
    fn test_public_inputs_match_serialized_vec() {
        let inputs = parse_snarkjs_public_json(&read_fixture("public.json")).unwrap();
        let round_trip: Vec<String> =
            serde_json::from_str(&serde_json::to_string(&inputs).unwrap()).unwrap();
        assert_eq!(inputs, round_trip);
    }

    #[test]
    fn test_generate_circom_groth16_garaga_calldata_from_proof_result_bn254_golden() {
        let snarkjs_proof = parse_snarkjs_proof_json(&read_fixture("proof.json")).unwrap();
        let public_inputs = parse_snarkjs_public_json(&read_fixture("public.json")).unwrap();
        let vk = read_fixture("verification_key.json");
        let expected: Vec<String> =
            serde_json::from_str(&read_fixture("expected_garaga_calldata.json")).unwrap();

        let proof_result = CircomProofResult {
            proof: snarkjs_proof_to_circom_proof(&snarkjs_proof),
            inputs: public_inputs,
        };

        let got =
            generate_circom_groth16_garaga_calldata_from_proof_result(proof_result, vk).unwrap();
        assert_eq!(got, expected);
    }

    #[test]
    fn test_generate_circom_groth16_garaga_calldata_bn254_golden() {
        let proof = read_fixture("proof.json");
        let public = read_fixture("public.json");
        let vk = read_fixture("verification_key.json");
        let expected: Vec<String> =
            serde_json::from_str(&read_fixture("expected_garaga_calldata.json")).unwrap();

        let got = generate_circom_groth16_garaga_calldata(proof, public, vk).unwrap();
        assert_eq!(got, expected);
        assert_eq!(got.len(), 1936);
        assert_eq!(got[0], "1935");
    }
}
