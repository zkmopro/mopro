pub mod app_config;

#[cfg(feature = "circom")]
mod circom;
#[cfg(feature = "halo2")]
mod halo2;
#[cfg(feature = "nova_scotia")]
mod nova_scotia;

#[cfg(feature = "circom")]
pub use circom::{
    generate_circom_proof_wtns, serialization::to_ethereum_inputs,
    serialization::to_ethereum_proof, verify_circom_proof, WtnsFn,
};

#[cfg(feature = "halo2")]
pub use halo2::{Halo2ProveFn, Halo2VerifyFn};

#[cfg(feature = "nova_scotia")]
//pub use nova_scotia::{};

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

#[cfg(not(feature = "nova_scotia"))]
#[macro_export]
macro_rules! nova_scotia_app {
    () => {
        fn generate_nova_scotia_proof(
            r1cs_path: String,
            cpp_bin_or_wasm_path: String,
            private_inputs: Vec<HashMap<String, Value>>,
            start_public_input: Vec<F<G1>>,
        ) -> Result<GenerateProofResult, MoproError> {
            panic!("Nova Scotia is not enabled in this build. Please pass `nova-scotia` feature to `mopro-ffi` to enable Nova Scotia.")
        }

        fn verify_nova_scotia_proof(
            recursive_snark: RecursiveSNARK<G1, G2, C1<G1>, C2<G2>>,
            pp: &PublicParams<G1, G2, C1<G1>, C2<G2>>,
            iteration_count: usize,
            start_public_input: &[E1::Scalar],
        ) -> Result<bool, MoproError> {
            panic!("Nova Scotia is not enabled in this build. Please pass `nova-scotia` feature to `mopro-ffi` to enable Nova Scotia.")
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
    #[error("NovaScotiaError: {0}")]
    NovaScotiaError(String),
}

#[derive(Debug, Clone)]
pub struct GenerateProofResult {
    pub proof: Vec<u8>,
    pub inputs: Vec<u8>,
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
/// mopro_ffi::set_halo2_proving_circuits!("fibonacci_pk.bin", FibonacciMoproCircuit::prove);
///
/// // Add `Fibonacci` circuit to verify proofs
// mopro_ffi::set_halo2_verifying_circuits!("fibonacci_vk.bin", FibonacciMoproCircuit::verify);
///
///
#[macro_export]
macro_rules! app {
    () => {
        // These are mandatory imports for the uniffi to pick them up and match with UDL
        use mopro_ffi::{GenerateProofResult, MoproError, ProofCalldata, G1, G2};

        mopro_ffi::circom_app!();

        mopro_ffi::halo2_app!();

        mopro_ffi::nova_scotia_app!();

        uniffi::include_scaffolding!("mopro");
    };
}
