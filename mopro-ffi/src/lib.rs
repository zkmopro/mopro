uniffi::setup_scaffolding!();

pub mod app_config;

#[cfg(feature = "circom")]
mod circom;
#[cfg(feature = "halo2")]
mod halo2;

#[cfg(feature = "halo2")]
pub use halo2::MoproHalo2;

#[cfg(feature = "circom")]
pub use circom::{generate_circom_proof_wtns, verify_circom_proof};

use std::collections::HashMap;
use uniffi::Record;

#[derive(Debug, thiserror::Error)]
pub enum MoproError {
    #[error("CircomError: {0}")]
    CircomError(String),
    #[error("Halo2Error: {0}")]
    Halo2Error(String),
}

pub type WtnsFn = fn(HashMap<String, Vec<num_bigint::BigInt>>) -> Vec<num_bigint::BigInt>;

#[macro_export]
macro_rules! setup_mopro {
    () => {
        // Setup the FFI bindings for dependent crates
        uniffi::setup_scaffolding!();

        #[derive(Debug, thiserror::Error, uniffi::Error)]
        pub enum MoproErrorExternal {
            #[error("CircomError: {0}")]
            CircomError(String),
            #[error("Halo2Error: {0}")]
            Halo2Error(String),
        }

        impl From<mopro::MoproError> for MoproErrorExternal {
            fn from(e: mopro::MoproError) -> Self {
                match e {
                    mopro::MoproError::CircomError(e) => MoproErrorExternal::CircomError(e),
                    mopro::MoproError::Halo2Error(e) => MoproErrorExternal::Halo2Error(e),
                }
            }
        }
    };
}

#[derive(Debug)]
pub enum FFIError {
    MoproError(MoproError),
    SerializationError(String),
}

#[derive(Debug, Clone, Record)]
pub struct GenerateProofResult {
    pub proof: Vec<u8>,
    pub inputs: Vec<u8>,
}

impl From<MoproError> for FFIError {
    fn from(error: MoproError) -> Self {
        FFIError::MoproError(error)
    }
}
