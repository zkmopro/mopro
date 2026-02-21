use crate::MoproError;
use std::sync::Once;

/// Guards one-time initialization of the gnark Go runtime.
static GNARK_INIT: Once = Once::new();

/// Result of a gnark Groth16 BN254 proof generation.
///
/// Both fields are hex-encoded gnark binary serializations:
/// - `proof`: compressed Groth16 proof
/// - `public_inputs`: public witness
#[derive(Debug, Clone)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct GnarkProofResult {
    pub proof: String,
    pub public_inputs: String,
}

/// Generate a Groth16 BN254 proof using gnark.
///
/// # Arguments
///
/// * `r1cs_path` - Path to the `.r1cs` file (CBOR binary)
/// * `pk_path` - Path to the `.pk` file (gnark binary)
/// * `witness_json` - JSON object mapping circuit field names to decimal string values
///
/// # Errors
///
/// Returns [`MoproError::GnarkError`] if proof generation fails.
#[cfg_attr(feature = "uniffi", uniffi::export)]
pub fn generate_gnark_proof(
    r1cs_path: String,
    pk_path: String,
    witness_json: String,
) -> Result<GnarkProofResult, MoproError> {
    GNARK_INIT.call_once(|| {
        rust_gnark::init().expect("Failed to initialize gnark runtime");
    });

    let result = rust_gnark::groth16_prove(&r1cs_path, &pk_path, &witness_json)
        .map_err(|e| MoproError::GnarkError(e.to_string()))?;

    Ok(GnarkProofResult {
        proof: result.proof,
        public_inputs: result.public_inputs,
    })
}

/// Verify a Groth16 BN254 proof using gnark.
///
/// # Arguments
///
/// * `r1cs_path` - Path to the `.r1cs` file
/// * `vk_path` - Path to the `.vk` file (gnark binary)
/// * `proof_result` - The proof result from [`generate_gnark_proof`]
///
/// # Returns
///
/// `true` if the proof is valid, `false` otherwise.
///
/// # Errors
///
/// Returns [`MoproError::GnarkError`] if verification encounters an error.
#[cfg_attr(feature = "uniffi", uniffi::export)]
pub fn verify_gnark_proof(
    r1cs_path: String,
    vk_path: String,
    proof_result: GnarkProofResult,
) -> Result<bool, MoproError> {
    GNARK_INIT.call_once(|| {
        rust_gnark::init().expect("Failed to initialize gnark runtime");
    });

    let inner = rust_gnark::Groth16ProofResult {
        proof: proof_result.proof,
        public_inputs: proof_result.public_inputs,
    };

    rust_gnark::groth16_verify(&r1cs_path, &vk_path, &inner)
        .map_err(|e| MoproError::GnarkError(e.to_string()))
}
