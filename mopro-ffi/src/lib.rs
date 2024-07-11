pub mod app_config;

#[cfg(feature = "circom")]
mod circom;
#[cfg(feature = "halo2")]
mod halo2;

#[cfg(feature = "circom")]
pub use circom::{
    generate_circom_proof_wtns, serialization::to_ethereum_inputs,
    serialization::to_ethereum_proof, verify_circom_proof, zkey_witness_map, WtnsFn,
};

#[cfg(feature = "halo2")]
pub use halo2::{Halo2ProveFn, Halo2VerifyFn};

#[cfg(not(feature = "circom"))]
#[macro_export]
macro_rules! circom_app {
    () => {

        fn generate_circom_proof(
            in0: String,
            in1: HashMap<String, Vec<String>>,
        ) -> Result<GenerateProofResult, MoproError> {
            panic!("Circom is not enabled in this build. Please pass `circom` feature to `mopro-ffi` to enable Circom.")
        }

        fn verify_circom_proof(
            in0: String,
            in1: Vec<u8>,
            in2: Vec<u8>,
        ) -> Result<bool, MoproError> {
            panic!("Circom is not enabled in this build. Please pass `circom` feature to `mopro-ffi` to enable Circom.")
        }

        fn to_ethereum_proof(in0: Vec<u8>) -> ProofCalldata {
            panic!("Circom is not enabled in this build. Please pass `circom` feature to `mopro-ffi` to enable Circom.")
        }

        fn to_ethereum_inputs(in0: Vec<u8>) -> Vec<String> {
            panic!("Circom is not enabled in this build. Please pass `circom` feature to `mopro-ffi` to enable Circom.")
        }
    };
}

use thiserror::Error;

#[derive(Debug, Error)]
pub enum MoproError {
    #[error("CircomError: {0}")]
    CircomError(String),
    #[error("Halo2Error: {0}")]
    Halo2Error(String),
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

#[macro_export]
macro_rules! halo2_app {
    () => {
        fn generate_halo2_proof(
            in0: String,
            in1: String,
            in2: HashMap<String, Vec<String>>,
        ) -> Result<GenerateProofResult, MoproError> {
            if let Ok((prove_fn, _)) = key_halo2_circuit_map(in1.as_str()) {
                prove_fn(&in0, &in1, in2)
            } else {
                Err(MoproError::Halo2Error("Unknown circuit name".to_string()))
            }
        }

        fn verify_halo2_proof(
            in0: String,
            in1: String,
            in2: Vec<u8>,
            in3: Vec<u8>,
        ) -> Result<bool, MoproError> {
            if let Ok((_, verify_fn)) = key_halo2_circuit_map(in1.as_str()) {
                verify_fn(&in0, &in1, in2, in3)
            } else {
                Err(MoproError::Halo2Error("Unknown circuit name".to_string()))
            }
        }
    };
}

// This macro should be used in dependent crates
//
// This macro handles getting relevant functions into
// scope and calling uniffi
//
// There should be a user defined `zkey_witness_map` function
// that maps zkey file stub to a witness generation function
// see test-e2e/src/lib.rs for an example
#[macro_export]
macro_rules! app {
    () => {
        use mopro_ffi::{GenerateProofResult, MoproError, ProofCalldata, G1, G2};
        use std::collections::HashMap;

        mopro_ffi::circom_app!();

        mopro_ffi::halo2_app!();

        uniffi::include_scaffolding!("mopro");
    };
}
