use super::garaga_convert::{to_groth16_proof, to_groth16_vk};
use super::snarkjs_types::{
    parse_snarkjs_proof_json, parse_snarkjs_public_json, parse_snarkjs_vk_json,
};
use crate::MoproError;
use garaga_rs::calldata::full_proof_with_hints::groth16::get_groth16_calldata_felt;
use garaga_rs::definitions::CurveID;

/// Build Starknet-compatible Groth16 calldata (BN254) from SnarkJS JSON artifacts.
///
/// `proof_json` — contents of snarkjs `proof.json`
/// `public_json` — contents of snarkjs `public.json` (array of decimal field elements)
/// `verification_key_json` — contents of snarkjs `verification_key.json`
///
/// Returns each Starknet felt as a decimal string, suitable for Flutter/Dart and
/// `starknet.dart` invoke calldata. Does not send transactions.
#[cfg_attr(feature = "uniffi", uniffi::export)]
pub fn generate_circom_groth16_garaga_calldata(
    proof_json: String,
    public_json: String,
    verification_key_json: String,
) -> Result<Vec<String>, MoproError> {
    let proof = parse_snarkjs_proof_json(&proof_json)
        .map_err(|e| MoproError::CircomError(format!("Garaga calldata error: {e}")))?;
    let public_inputs = parse_snarkjs_public_json(&public_json)
        .map_err(|e| MoproError::CircomError(format!("Garaga calldata error: {e}")))?;
    let vk = parse_snarkjs_vk_json(&verification_key_json)
        .map_err(|e| MoproError::CircomError(format!("Garaga calldata error: {e}")))?;

    let garaga_proof = to_groth16_proof(&proof, &public_inputs)
        .map_err(|e| MoproError::CircomError(format!("Garaga calldata error: {e}")))?;
    let garaga_vk = to_groth16_vk(&vk)
        .map_err(|e| MoproError::CircomError(format!("Garaga calldata error: {e}")))?;

    let felts = get_groth16_calldata_felt(&garaga_proof, &garaga_vk, CurveID::BN254)
        .map_err(|e| MoproError::CircomError(format!("Garaga calldata error: {e}")))?;

    Ok(felts.into_iter().map(|f| f.to_string()).collect())
}
