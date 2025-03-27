#![allow(unexpected_cfgs)]
pub mod app_config;

#[cfg(feature = "circom")]
mod circom;
#[cfg(feature = "halo2")]
mod halo2;

#[cfg(feature = "circom")]
pub use circom::{generate_circom_proof_wtns, verify_circom_proof};

#[cfg(feature = "circom")]
pub use circom_prover::{prover, witness};

#[cfg(feature = "halo2")]
pub use halo2::{Halo2ProveFn, Halo2VerifyFn};

#[cfg(not(feature = "circom"))]
#[macro_export]
macro_rules! circom_app {
    ($result:ty, $proof:ty, $err:ty, $proof_lib:ty) => {
        // TODO: fix this if CLI template can be customized
        #[allow(dead_code)]
        #[cfg_attr(not(disable_uniffi_export), uniffi::export)]
        fn generate_circom_proof(
            zkey_path: String,
            circuit_inputs: String,
            proof_lib: $proof_lib,
        ) -> Result<$result, $err> {
            panic!("Circom is not enabled in this build. Please pass `circom` feature to `mopro-ffi` to enable Circom.")
        }

        // TODO: fix this if CLI template can be customized
        #[allow(dead_code)]
        #[cfg_attr(not(disable_uniffi_export), uniffi::export)]
        fn verify_circom_proof(
            zkey_path: String,
            proof_result: $result,
            proof_lib: $proof_lib,
        ) -> Result<bool, $err> {
            panic!("Circom is not enabled in this build. Please pass `circom` feature to `mopro-ffi` to enable Circom.")
        }
    };
}

#[cfg(not(feature = "halo2"))]
#[macro_export]
macro_rules! halo2_app {
    ($result:ty, $err:ty) => {
        // TODO: fix this if CLI template can be customized
        #[allow(dead_code)]
        #[cfg_attr(not(disable_uniffi_export), uniffi::export)]
        fn generate_halo2_proof(
            srs_path: String,
            pk_path: String,
            inputs: std::collections::HashMap<String, Vec<String>>,
        ) -> Result<$result, $err> {
            panic!("Halo2 is not enabled in this build. Please pass `halo2` feature to `mopro-ffi` to enable Halo2.")
        }

        // TODO: fix this if CLI template can be customized
        #[allow(dead_code)]
        #[cfg_attr(not(disable_uniffi_export), uniffi::export)]
        fn verify_halo2_proof(
            srs_path: String,
            vk_path: String,
            proof_data: Vec<u8>,
            public_inputs: Vec<u8>,
        ) -> Result<bool, $err> {
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

//
// Circom Proof
//
#[derive(Debug, Clone)]
pub struct CircomProofResult {
    pub proof: CircomProof,
    pub inputs: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct CircomProof {
    pub a: G1,
    pub b: G2,
    pub c: G1,
    pub protocol: String,
    pub curve: String,
}

#[derive(Debug, Clone, Default)]
pub struct G1 {
    pub x: String,
    pub y: String,
    pub z: String,
}

#[derive(Debug, Clone, Default)]
pub struct G2 {
    pub x: Vec<String>,
    pub y: Vec<String>,
    pub z: Vec<String>,
}
//
// Halo2 Proof
//
#[derive(Debug, Clone)]
pub struct Halo2ProofResult {
    pub proof: Vec<u8>,
    pub inputs: Vec<u8>,
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

        //
        // Halo2 Section
        //
        #[derive(Debug, Clone, uniffi::Record)]
        pub struct Halo2ProofResult {
            pub proof: Vec<u8>,
            pub inputs: Vec<u8>,
        }

        impl From<mopro_ffi::Halo2ProofResult> for Halo2ProofResult {
            fn from(result: mopro_ffi::Halo2ProofResult) -> Self {
                Self {
                    proof: result.proof,
                    inputs: result.inputs,
                }
            }
        }
        // End of Halo2 Section

        //
        // Circom Section
        //
        #[derive(Debug, Clone, uniffi::Record)]
        pub struct CircomProofResult {
            pub proof: CircomProof,
            pub inputs: Vec<String>,
        }

        impl From<mopro_ffi::CircomProofResult> for CircomProofResult {
            fn from(result: mopro_ffi::CircomProofResult) -> Self {
                Self {
                    proof: result.proof.into(),
                    inputs: result.inputs,
                }
            }
        }

        impl Into<mopro_ffi::CircomProofResult> for CircomProofResult {
            fn into(self) -> mopro_ffi::CircomProofResult {
                mopro_ffi::CircomProofResult {
                    proof: self.proof.into(),
                    inputs: self.inputs,
                }
            }
        }

        #[derive(Debug, Clone, Default, uniffi::Record)]
        pub struct G1 {
            pub x: String,
            pub y: String,
            pub z: String,
        }

        #[derive(Debug, Clone, Default, uniffi::Record)]
        pub struct G2 {
            pub x: Vec<String>,
            pub y: Vec<String>,
            pub z: Vec<String>,
        }

        #[derive(Debug, Clone, Default, uniffi::Record)]
        pub struct CircomProof {
            pub a: G1,
            pub b: G2,
            pub c: G1,
            pub protocol: String,
            pub curve: String,
        }

        impl From<mopro_ffi::CircomProof> for CircomProof {
            fn from(proof: mopro_ffi::CircomProof) -> Self {
                CircomProof {
                    a: G1 {
                        x: proof.a.x,
                        y: proof.a.y,
                        z: proof.a.z,
                    },
                    b: G2 {
                        x: proof.b.x,
                        y: proof.b.y,
                        z: proof.b.z,
                    },
                    c: G1 {
                        x: proof.c.x,
                        y: proof.c.y,
                        z: proof.c.z,
                    },
                    protocol: proof.protocol,
                    curve: proof.curve,
                }
            }
        }

        impl Into<mopro_ffi::CircomProof> for CircomProof {
            fn into(self) -> mopro_ffi::CircomProof {
                mopro_ffi::CircomProof {
                    a: mopro_ffi::G1 {
                        x: self.a.x,
                        y: self.a.y,
                        z: self.a.z,
                    },
                    b: mopro_ffi::G2 {
                        x: self.b.x,
                        y: self.b.y,
                        z: self.b.z,
                    },
                    c: mopro_ffi::G1 {
                        x: self.c.x,
                        y: self.c.y,
                        z: self.c.z,
                    },
                    protocol: self.protocol,
                    curve: self.curve,
                }
            }
        }
        // End of Circom Section

        #[derive(Debug, Clone, Default, uniffi::Enum)]
        pub enum ProofLib {
            #[default]
            Arkworks,
            Rapidsnark,
        }

        mopro_ffi::circom_app!(CircomProofResult, CircomProof, MoproError, ProofLib);

        mopro_ffi::halo2_app!(Halo2ProofResult, MoproError);
    };
}
