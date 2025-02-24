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
            in0: String,
            in1: std::collections::HashMap<String, Vec<String>>,
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

#[cfg(not(feature = "halo2"))]
#[macro_export]
macro_rules! halo2_app {
    () => {
        fn generate_halo2_proof(
            in0: String,
            in1: String,
            in2: std::collections::HashMap<String, Vec<String>>,
        ) -> Result<GenerateProofResult, MoproError> {
            panic!("Halo2 is not enabled in this build. Please pass `halo2` feature to `mopro-ffi` to enable Halo2.")
        }

        fn verify_halo2_proof(
            in0: String,
            in1: String,
            in2: Vec<u8>,
            in3: Vec<u8>,
        ) -> Result<bool, MoproError> {
            panic!("Halo2 is not enabled in this build. Please pass `halo2` feature to `mopro-ffi` to enable Halo2.")
        }
    };
}

#[derive(Debug, thiserror::Error)]
pub enum CircomCircuitError {
    #[error("Unknown ZKEY: {0}")]
    UnknownZKey(String),
}

#[derive(Debug, thiserror::Error)]
pub enum Halo2CircuitError {
    #[error("Unknown Proving Key: {0}")]
    UnknownProvingKey(String),
    #[error("Unknown Verifying Key: {0}")]
    UnknownVerifyingKey(String),
}

uniffi::setup_scaffolding!();

#[derive(Debug, Clone, uniffi::Object)]
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
        // These are mandatory imports for the uniffi to pick them up and match with UDL
        use mopro_ffi::{
            CircomCircuitError, GenerateProofResult, Halo2CircuitError, ProofCalldata, G1, G2,
        };

        uniffi::setup_scaffolding!("mopro");

        // This should be declared into this macro due to Uniffi's limitation
        // Please refer this comment: https://github.com/mozilla/uniffi-rs/issues/2257#issuecomment-2395668332
        #[derive(Debug, thiserror::Error, uniffi::Error)]
        pub enum MoproError {
            #[error("CircomError: {0}")]
            CircomError(String),
            #[error("Halo2Error: {0}")]
            Halo2Error(String),
        }

        impl From<uniffi::deps::anyhow::Error> for MoproError {
            fn from(err: uniffi::deps::anyhow::Error) -> Self {
                if err.downcast_ref::<CircomCircuitError>().is_some() {
                    MoproError::CircomError(err.to_string())
                } else if err.downcast_ref::<Halo2CircuitError>().is_some() {
                    MoproError::Halo2Error(err.to_string())
                } else {
                    panic!("Unhandled error type: {}", err)
                }
            }
        }

        mopro_ffi::circom_app!(MoproError);

        mopro_ffi::halo2_app!(MoproError);
    };
}
