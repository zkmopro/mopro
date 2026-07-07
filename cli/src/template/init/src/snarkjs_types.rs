use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct SnarkJsProof {
    pub pi_a: Vec<String>,
    pub pi_b: Vec<Vec<String>>,
    pub pi_c: Vec<String>,
    pub protocol: String,
    pub curve: String,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct SnarkJsVerificationKey {
    pub protocol: String,
    pub curve: String,
    #[serde(rename = "nPublic")]
    pub n_public: usize,
    pub vk_alpha_1: Vec<String>,
    pub vk_beta_2: Vec<Vec<String>>,
    pub vk_gamma_2: Vec<Vec<String>>,
    pub vk_delta_2: Vec<Vec<String>>,
    #[serde(rename = "IC")]
    pub ic: Vec<Vec<String>>,
}

pub(crate) const SNARKJS_BN128_CURVE: &str = "bn128";
pub(crate) const GROTH16_PROTOCOL: &str = "groth16";

pub(crate) fn parse_snarkjs_proof_json(s: &str) -> Result<SnarkJsProof, String> {
    let proof: SnarkJsProof =
        serde_json::from_str(s).map_err(|e| format!("invalid proof JSON: {e}"))?;
    validate_proof(&proof)?;
    Ok(proof)
}

pub(crate) fn parse_snarkjs_public_json(s: &str) -> Result<Vec<String>, String> {
    let inputs: Vec<String> =
        serde_json::from_str(s).map_err(|e| format!("invalid public inputs JSON: {e}"))?;
    if inputs.is_empty() {
        return Err("public inputs must not be empty".to_string());
    }
    Ok(inputs)
}

pub(crate) fn parse_snarkjs_vk_json(s: &str) -> Result<SnarkJsVerificationKey, String> {
    let vk: SnarkJsVerificationKey =
        serde_json::from_str(s).map_err(|e| format!("invalid verification key JSON: {e}"))?;
    validate_vk(&vk)?;
    Ok(vk)
}

fn validate_proof(proof: &SnarkJsProof) -> Result<(), String> {
    if proof.protocol != GROTH16_PROTOCOL {
        return Err(format!(
            "unsupported proof protocol: {} (expected {GROTH16_PROTOCOL})",
            proof.protocol
        ));
    }
    if proof.curve != SNARKJS_BN128_CURVE {
        return Err(format!(
            "unsupported curve: {} (BN254/{SNARKJS_BN128_CURVE} only in v1)",
            proof.curve
        ));
    }
    if proof.pi_a.len() < 2 || proof.pi_c.len() < 2 {
        return Err("invalid G1 point in proof".to_string());
    }
    if proof.pi_b.len() < 2 || proof.pi_b[0].len() < 2 || proof.pi_b[1].len() < 2 {
        return Err("invalid G2 point in proof".to_string());
    }
    Ok(())
}

fn validate_vk(vk: &SnarkJsVerificationKey) -> Result<(), String> {
    if vk.protocol != GROTH16_PROTOCOL {
        return Err(format!(
            "unsupported vk protocol: {} (expected {GROTH16_PROTOCOL})",
            vk.protocol
        ));
    }
    if vk.curve != SNARKJS_BN128_CURVE {
        return Err(format!(
            "unsupported curve: {} (BN254/{SNARKJS_BN128_CURVE} only in v1)",
            vk.curve
        ));
    }
    if vk.ic.len() != vk.n_public + 1 {
        return Err(format!(
            "IC length mismatch: expected {} points (nPublic + 1), got {}",
            vk.n_public + 1,
            vk.ic.len()
        ));
    }
    Ok(())
}
