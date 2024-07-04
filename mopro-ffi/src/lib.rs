uniffi::setup_scaffolding!();

pub mod app_config;

#[cfg(feature = "circom")]
mod circom;
#[cfg(feature = "halo2")]
mod halo2;

#[cfg(feature = "halo2")]
pub use {halo2::MoproHalo2, mopro_macro::Halo2CircuitBindings};

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
macro_rules! setup_mopro_ffi {
    () => {
        // Setup the FFI bindings for dependent crates
        uniffi::setup_scaffolding!();

        use mopro_ffi::MoproError;

        #[derive(Debug, thiserror::Error, uniffi::Error)]
        pub enum MoproErrorExternal {
            #[error("CircomError: {0}")]
            CircomError(String),
            #[error("Halo2Error: {0}")]
            Halo2Error(String),
        }

        impl From<MoproError> for MoproErrorExternal {
            fn from(e: MoproError) -> Self {
                match e {
                    MoproError::CircomError(e) => MoproErrorExternal::CircomError(e),
                    MoproError::Halo2Error(e) => MoproErrorExternal::Halo2Error(e),
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
