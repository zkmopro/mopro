use serde::Deserialize;

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

pub(crate) fn parse_snarkjs_vk_json(s: &str) -> Result<SnarkJsVerificationKey, String> {
    let vk: SnarkJsVerificationKey =
        serde_json::from_str(s).map_err(|e| format!("invalid verification key JSON: {e}"))?;
    validate_vk(&vk)?;
    Ok(vk)
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
