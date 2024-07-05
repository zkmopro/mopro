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

/// Re-export the required dependencies to be used by the `mopro_<adapter>_circuit!` macros
pub mod reexports {
    pub use paste::paste;
}

/// This macro **must** be called from the top of your `lib.rs` or `main.rs` file to setup the FFI bindings
#[macro_export]
macro_rules! setup_mopro {
    () => {
        // Setup the FFI bindings for dependent crates
        uniffi::setup_scaffolding!();

        #[derive(Debug, uniffi::Error)]
        pub enum MoproErrorExternal {
            CircomError(String),
            Halo2Error(String),
        }

        impl std::fmt::Display for MoproErrorExternal {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    MoproErrorExternal::CircomError(e) => write!(f, "CircomError: {}", e),
                    MoproErrorExternal::Halo2Error(e) => write!(f, "Halo2Error: {}", e),
                }
            }
        }

        impl From<mopro_ffi::MoproError> for MoproErrorExternal {
            fn from(e: mopro_ffi::MoproError) -> Self {
                match e {
                    mopro_ffi::MoproError::CircomError(e) => MoproErrorExternal::CircomError(e),
                    mopro_ffi::MoproError::Halo2Error(e) => MoproErrorExternal::Halo2Error(e),
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
