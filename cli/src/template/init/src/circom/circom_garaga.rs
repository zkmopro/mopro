use super::garaga_convert::{to_groth16_proof_from_mopro, to_groth16_vk};
use super::snarkjs_types::{parse_snarkjs_vk_json, GROTH16_PROTOCOL, SNARKJS_BN128_CURVE};
use super::{CircomProof, CircomProofResult};
use crate::MoproError;
use garaga_rs::calldata::full_proof_with_hints::groth16::{
    get_groth16_calldata_felt, Groth16Proof, Groth16VerificationKey,
};
use garaga_rs::definitions::CurveID;

fn garaga_calldata_core(
    garaga_proof: Groth16Proof,
    garaga_vk: Groth16VerificationKey,
) -> Result<Vec<String>, MoproError> {
    let felts = get_groth16_calldata_felt(&garaga_proof, &garaga_vk, CurveID::BN254)
        .map_err(|e| MoproError::CircomError(format!("Garaga calldata error: {e}")))?;

    Ok(felts.into_iter().map(|f| f.to_string()).collect())
}

fn validate_public_input_count(
    public_inputs: &[String],
    n_public: usize,
) -> Result<(), MoproError> {
    if public_inputs.len() != n_public {
        return Err(MoproError::CircomError(format!(
            "Garaga calldata error: public input count mismatch: got {}, nPublic {}",
            public_inputs.len(),
            n_public
        )));
    }
    Ok(())
}

fn validate_mopro_groth16_bn254(proof: &CircomProof) -> Result<(), String> {
    if proof.protocol != GROTH16_PROTOCOL {
        return Err(format!(
            "unsupported proof protocol: {} (expected {GROTH16_PROTOCOL})",
            proof.protocol
        ));
    }
    let curve = proof.curve.to_ascii_lowercase();
    if curve != SNARKJS_BN128_CURVE && curve != "bn254" {
        return Err(format!(
            "unsupported curve: {} (BN254/{SNARKJS_BN128_CURVE} only in v1)",
            proof.curve
        ));
    }
    Ok(())
}

fn circom_proof_to_groth16(
    proof: &CircomProof,
    public_inputs: &[String],
) -> Result<Groth16Proof, String> {
    validate_mopro_groth16_bn254(proof)?;
    to_groth16_proof_from_mopro(
        &proof.a.x,
        &proof.a.y,
        &proof.b.x,
        &proof.b.y,
        &proof.c.x,
        &proof.c.y,
        public_inputs,
    )
}

/// Build Starknet-compatible Groth16 calldata (BN254) from a [`CircomProofResult`].
///
/// Prove with [`super::generate_circom_proof`], then pass the result here together with
/// `verification_key_json` (contents of snarkjs `verification_key.json` from a one-time
/// zkey export).
///
/// `proof_result.inputs` has the same content as SnarkJS `public.json` (decimal field
/// elements). No separate public-inputs file is required.
///
/// Returns each Starknet felt as a decimal string, suitable for Flutter/Dart and
/// `starknet.dart` invoke calldata. Does not send transactions.
#[cfg_attr(feature = "uniffi", uniffi::export)]
pub fn generate_circom_groth16_garaga_calldata(
    proof_result: CircomProofResult,
    verification_key_json: String,
) -> Result<Vec<String>, MoproError> {
    let vk = parse_snarkjs_vk_json(&verification_key_json)
        .map_err(|e| MoproError::CircomError(format!("Garaga calldata error: {e}")))?;
    validate_public_input_count(&proof_result.inputs, vk.n_public)?;

    let garaga_proof = circom_proof_to_groth16(&proof_result.proof, &proof_result.inputs)
        .map_err(|e| MoproError::CircomError(format!("Garaga calldata error: {e}")))?;
    let garaga_vk = to_groth16_vk(&vk)
        .map_err(|e| MoproError::CircomError(format!("Garaga calldata error: {e}")))?;

    garaga_calldata_core(garaga_proof, garaga_vk)
}
