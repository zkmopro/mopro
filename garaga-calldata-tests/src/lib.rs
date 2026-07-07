//! Integration tests for Circom/Groth16 Garaga calldata helpers in the CLI template.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum MoproError {
    #[error("CircomError: {0}")]
    CircomError(String),
}

mod circom;

pub use circom::generate_circom_groth16_garaga_calldata;

#[cfg(test)]
mod tests {
    use super::*;
    use circom::garaga_convert::{to_groth16_proof, to_groth16_vk};
    use circom::snarkjs_types::{
        parse_snarkjs_proof_json, parse_snarkjs_public_json, parse_snarkjs_vk_json,
    };

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
