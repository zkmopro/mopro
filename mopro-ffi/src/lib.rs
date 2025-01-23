pub mod app_config;

#[cfg(feature = "ashlang")]
pub mod ashlang;
#[cfg(feature = "circom")]
mod circom;
#[cfg(feature = "halo2")]
mod halo2;

#[cfg(feature = "circom")]
pub use circom::{generate_circom_proof_wtns, verify_circom_proof};

#[cfg(feature = "circom")]
pub use circom_prover::{
    prover::{self, serialization::to_ethereum_inputs, serialization::to_ethereum_proof},
    witness,
};

#[cfg(feature = "halo2")]
pub use halo2::{Halo2ProveFn, Halo2VerifyFn};

#[cfg(not(feature = "ashlang"))]
#[macro_export]
macro_rules! ashlang_spartan_app {
    () => {
        fn generate_ashlang_spartan_proof(
            ar1cs_path: String, // path to ar1cs file
            secret_inputs: Vec<String>,
        ) -> Result<mopro_ffi::GenerateProofResult, mopro_ffi::MoproError> {
            panic!("Ashlang proving is not enabled in this build. Please pass `ashlang` feature to `mopro-ffi` to enable Ashlang.");
        }

        fn verify_ashlang_spartan_proof(
            ar1cs_path: String,
            proof: Vec<u8>,
        ) -> Result<bool, mopro_ffi::MoproError> {
            panic!("Ashlang proving is not enabled in this build. Please pass `ashlang` feature to `mopro-ffi` to enable Ashlang.");
        }
    };
}

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

use thiserror::Error;

#[derive(Debug, Error)]
pub enum MoproError {
    #[error("CircomError: {0}")]
    CircomError(String),
    #[error("Halo2Error: {0}")]
    Halo2Error(String),
    #[error("AshlangError: {0}")]
    AshlangError(String),
}

#[derive(Debug, Clone)]
pub struct GenerateProofResult {
    pub proof: Vec<u8>,
    pub inputs: Vec<u8>,
}

/// This macro is used to setup the Mopro FFI library
/// It should be included in the `lib.rs` file of the project
///
/// This should be used with the adapter-specific macros, such as `set_circom_circuits!(...)`
/// and `set_halo2_proving_circuits!(...)`, etc.
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
//     multiplier2_witness,
/// )
/// ```
///
/// # Halo2 Example
/// ```ignore
/// // Setup the Mopro FFI library
/// mopro_ffi::app!();
///
/// // Import a prepared Halo2 circuit
/// use crate::halo2::FibonacciMoproCircuit;
///
/// // Add `Fibonacci` circuit to generate proofs
/// mopro_ffi::set_halo2_proving_circuits!("plonk_fibonacci_pk.bin", FibonacciMoproCircuit::prove);
///
/// // Add `Fibonacci` circuit to verify proofs
// mopro_ffi::set_halo2_verifying_circuits!("plonk_fibonacci_vk.bin", FibonacciMoproCircuit::verify);
///
///
#[macro_export]
macro_rules! app {
    () => {
        // These are mandatory imports for the uniffi to pick them up and match with UDL
        use circom_prover::{
            prover::{CircomProof, ProofLib},
            witness::{WitnessFn, WitnessLib},
            ProofCalldata, G1, G2,
        };
        use mopro_ffi::{GenerateProofResult, MoproError};

        mopro_ffi::circom_app!();

        mopro_ffi::halo2_app!();

        mopro_ffi::ashlang_spartan_app!();

        uniffi::include_scaffolding!("mopro");
    };
}
