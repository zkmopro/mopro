pub mod app_config;

#[cfg(feature = "circom")]
mod circom;
#[cfg(feature = "halo2")]
mod halo2;

#[cfg(feature = "circom")]
pub use circom::{
    generate_circom_proof_wtns, to_ethereum_inputs, to_ethereum_proof, verify_circom_proof,
    ProofCalldata, G1, G2,
};

#[cfg(feature = "circom")]
pub use circom_prover::{prover, witness};

#[cfg(feature = "halo2")]
pub use halo2::{Halo2ProveFn, Halo2VerifyFn};

#[cfg(not(feature = "circom"))]
#[macro_export]
macro_rules! circom_app {
    () => {
        fn generate_circom_proof(
            zkey_path: String,
            inputs: std::collections::HashMap<String, Vec<String>>,
        ) -> Result<GenerateProofResult, MoproError> {
            panic!("Circom is not enabled in this build. Please pass `circom` feature to `mopro-ffi` to enable Circom.")
        }

        fn verify_circom_proof(
            zkey_path: String,
            proof_data: Vec<u8>,
            public_inputs: Vec<u8>,
        ) -> Result<bool, MoproError> {
            panic!("Circom is not enabled in this build. Please pass `circom` feature to `mopro-ffi` to enable Circom.")
        }

        fn to_ethereum_proof(proof_data: Vec<u8>) -> ProofCalldata {
            panic!("Circom is not enabled in this build. Please pass `circom` feature to `mopro-ffi` to enable Circom.")
        }

        fn to_ethereum_inputs(public_inputs: Vec<u8>) -> Vec<String> {
            panic!("Circom is not enabled in this build. Please pass `circom` feature to `mopro-ffi` to enable Circom.")
        }
    };
}

#[cfg(not(feature = "halo2"))]
#[macro_export]
macro_rules! halo2_app {
    () => {
        fn generate_halo2_proof(
            srs_path: String,
            pk_path: String,
            inputs: std::collections::HashMap<String, Vec<String>>,
        ) -> Result<GenerateProofResult, MoproError> {
            panic!("Halo2 is not enabled in this build. Please pass `halo2` feature to `mopro-ffi` to enable Halo2.")
        }

        fn verify_halo2_proof(
            srs_path: String,
            vk_path: String,
            proof_data: Vec<u8>,
            public_inputs: Vec<u8>,
        ) -> Result<bool, MoproError> {
            panic!("Halo2 is not enabled in this build. Please pass `halo2` feature to `mopro-ffi` to enable Halo2.")
        }
    };
}

#[derive(Debug, thiserror::Error)]
pub enum MoproError {
    #[error("CircomError: {0}")]
    CircomError(String),
    #[error("Halo2Error: {0}")]
    Halo2Error(String),
}

#[derive(Debug, Clone)]
pub struct GenerateProofResult {
    pub proof: Vec<u8>,
    pub inputs: Vec<u8>,
}

#[cfg(not(feature = "circom"))]
#[derive(Debug, Clone, Default)]
pub struct G1 {
    pub x: String,
    pub y: String,
}

#[cfg(not(feature = "circom"))]
#[derive(Debug, Clone, Default)]
pub struct G2 {
    pub x: Vec<String>,
    pub y: Vec<String>,
}

#[cfg(not(feature = "circom"))]
#[derive(Debug, Clone, Default)]
pub struct ProofCalldata {
    pub a: G1,
    pub b: G2,
    pub c: G1,
}

/// This macro is used to setup the Mopro FFI library
/// It should be included in the `lib.rs` file of the project
///
/// This should be used with the adapter-specific macros, such as `set_circom_circuits!(...)`
/// and `set_halo2_circuits!(...)`, etc.
///
/// # Circom Example
/// ```ignore
/// // Setup the Mopro FFI library
/// mopro_ffi::app!();
///
/// // Generate a Witness Generation function for the `multiplier2` circom circuit
/// rust_witness::witness!(multiplier2);
///
/// // Add `multiplier2` circom circuit to be exposed to the FFI
/// mopro_ffi::set_circom_circuits!(
///     "multiplier2_final.zkey",
///     WitnessFn::RustWitness(multiplier2_witness),
/// )
/// ```
///
/// # Halo2 Example
/// ```ignore
/// // Setup the Mopro FFI library
/// mopro_ffi::app!();
///
/// // Add `Fibonacci` circuit to generate proofs and verify proofs
/// mopro_ffi::set_halo2_circuits!(
///     "plonk_fibonacci_pk.bin",
///     plonk_fibonacci::prove,
///     "plonk_fibonacci_vk.bin",
///     plonk_fibonacci::verify
/// );
/// ```
#[macro_export]
macro_rules! app {
    () => {
        uniffi::setup_scaffolding!("mopro");

        // This should be declared into this macro due to Uniffi's limitation
        // Please refer this issue: https://github.com/mozilla/uniffi-rs/issues/2257
        #[derive(Debug, thiserror::Error, uniffi::Error)]
        pub enum MoproError {
            #[error("CircomError: {0}")]
            CircomError(String),
            #[error("Halo2Error: {0}")]
            Halo2Error(String),
        }

        impl From<mopro_ffi::MoproError> for MoproError {
            fn from(err: mopro_ffi::MoproError) -> Self {
                match err {
                    mopro_ffi::MoproError::CircomError(e) => Self::CircomError(e),
                    mopro_ffi::MoproError::Halo2Error(e) => Self::Halo2Error(e),
                    _ => panic!("Unhandled error type: {}", err),
                }
            }
        }

        #[derive(Debug, Clone, uniffi::Record)]
        pub struct GenerateProofResult {
            pub proof: Vec<u8>,
            pub inputs: Vec<u8>,
        }

        impl From<mopro_ffi::GenerateProofResult> for GenerateProofResult {
            fn from(result: mopro_ffi::GenerateProofResult) -> Self {
                Self {
                    proof: result.proof,
                    inputs: result.inputs,
                }
            }
        }

        #[derive(Debug, Clone, Default, uniffi::Record)]
        pub struct G1 {
            pub x: String,
            pub y: String,
        }

        #[derive(Debug, Clone, Default, uniffi::Record)]
        pub struct G2 {
            pub x: Vec<String>,
            pub y: Vec<String>,
        }

        #[derive(Debug, Clone, Default, uniffi::Record)]
        pub struct ProofCalldata {
            pub a: G1,
            pub b: G2,
            pub c: G1,
        }

        impl From<mopro_ffi::ProofCalldata> for ProofCalldata {
            fn from(result: mopro_ffi::ProofCalldata) -> Self {
                ProofCalldata {
                    a: G1 {
                        x: result.a.x,
                        y: result.a.y,
                    },
                    b: G2 {
                        x: result.b.x,
                        y: result.b.y,
                    },
                    c: G1 {
                        x: result.c.x,
                        y: result.c.y,
                    },
                }
            }
        }

        mopro_ffi::circom_app!(GenerateProofResult, ProofCalldata, MoproError);

        mopro_ffi::halo2_app!(GenerateProofResult, MoproError);
    };
}
