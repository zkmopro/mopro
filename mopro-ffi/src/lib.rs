#[cfg(feature = "circom")]
mod circom;
#[cfg(feature = "halo2")]
mod halo2;

use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MoproError {
    #[error("CircomError: {0}")]
    CircomError(String),
    #[error("Halo2Error: {0}")]
    Halo2Error(String),
}

use circom::{generate_circom_proof, to_ethereum_inputs, to_ethereum_proof, verify_circom_proof};

#[cfg(feature = "halo2")]
use halo2::{generate_halo2_proof, verify_halo2_proof};

#[cfg(not(feature = "halo2"))]
pub fn generate_halo2_proof(
    _: HashMap<String, Vec<String>>,
) -> Result<GenerateProofResult, MoproError> {
    Err(MoproError::Halo2Error(
        "Project does not have Halo2 feature enabled".to_string(),
    ))
}

#[cfg(not(feature = "halo2"))]
pub fn verify_halo2_proof(_: Vec<u8>, _: Vec<u8>) -> Result<bool, MoproError> {
    Err(MoproError::Halo2Error(
        "Project does not have Halo2 feature enabled".to_string(),
    ))
}

#[cfg(not(feature = "circom"))]
pub fn generate_circom_proof(
    _: String,
    _: HashMap<String, Vec<String>>,
) -> Result<GenerateProofResult, MoproError> {
    Err(MoproError::CircomError("Project is compiled for Halo2 proving system. This function is currently not supported in Halo2.".to_string()))
}

#[cfg(not(feature = "circom"))]
pub fn verify_circom_proof(_: String, _: Vec<u8>, _: Vec<u8>) -> Result<bool, MoproError> {
    Err(MoproError::CircomError("Project is compiled for Halo2 proving system. This function is currently not supported in Halo2.".to_string()))
}

#[cfg(not(feature = "circom"))]
pub fn to_ethereum_proof(_: Vec<u8>) -> ProofCalldata {
    panic!("not built with circom");
}

#[cfg(not(feature = "circom"))]
pub fn to_ethereum_inputs(_: Vec<u8>) -> Vec<String> {
    panic!("not built with circom");
}

#[derive(Debug)]
pub enum FFIError {
    MoproError(MoproError),
    SerializationError(String),
}

#[derive(Debug, Clone)]
pub struct GenerateProofResult {
    pub proof: Vec<u8>,
    pub inputs: Vec<u8>,
}

impl From<MoproError> for FFIError {
    fn from(error: MoproError) -> Self {
        FFIError::MoproError(error)
    }
}

#[derive(Debug, Clone, Default)]
pub struct G1 {
    pub x: String,
    pub y: String,
}

#[derive(Debug, Clone, Default)]
pub struct G2 {
    pub x: Vec<String>,
    pub y: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct ProofCalldata {
    pub a: G1,
    pub b: G2,
    pub c: G1,
}

uniffi::include_scaffolding!("mopro");
