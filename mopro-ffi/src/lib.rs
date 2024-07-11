pub mod app_config;
#[cfg(feature = "circom")]
pub mod circom;
#[cfg(feature = "halo2")]
pub mod halo2;

use std::collections::HashMap;
use thiserror::Error;

pub type WtnsFn = fn(HashMap<String, Vec<num_bigint::BigInt>>) -> Vec<num_bigint::BigInt>;

#[derive(Debug, Error)]
pub enum MoproError {
    #[error("CircomError: {0}")]
    CircomError(String),
    #[error("Halo2Error: {0}")]
    Halo2Error(String),
}

#[cfg(feature = "circom")]
pub fn generate_circom_proof_wtns(
    in0: String,
    in1: HashMap<String, Vec<String>>,
    in2: WtnsFn,
) -> Result<GenerateProofResult, MoproError> {
    circom::generate_circom_proof_wtns(in0, in1, in2)
}

#[cfg(not(feature = "circom"))]
pub fn generate_circom_proof_wtns(
    _: String,
    _: HashMap<String, Vec<String>>,
    _: WtnsFn,
) -> Result<GenerateProofResult, MoproError> {
    Err(MoproError::CircomError("Project is compiled for Halo2 proving system. This function is currently not supported in Halo2.".to_string()))
}

#[cfg(feature = "circom")]
pub fn verify_circom_proof(in0: String, in1: Vec<u8>, in2: Vec<u8>) -> Result<bool, MoproError> {
    circom::verify_circom_proof(in0, in1, in2)
}

#[cfg(not(feature = "circom"))]
pub fn verify_circom_proof(_: String, _: Vec<u8>, _: Vec<u8>) -> Result<bool, MoproError> {
    Err(MoproError::CircomError("Project is compiled for Halo2 proving system. This function is currently not supported in Halo2.".to_string()))
}

#[cfg(feature = "circom")]
pub fn to_ethereum_proof(in0: Vec<u8>) -> ProofCalldata {
    circom::serialization::to_ethereum_proof(in0)
}

#[cfg(not(feature = "circom"))]
pub fn to_ethereum_proof(_: Vec<u8>) -> ProofCalldata {
    panic!("not built with circom");
}

#[cfg(feature = "circom")]
pub fn to_ethereum_inputs(in0: Vec<u8>) -> Vec<String> {
    circom::serialization::to_ethereum_inputs(in0)
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

        static CIRCOM_CIRCUITS: Lazy<HashMap<String, WtnsFn>> = Lazy::new(|| set_circom_circuits());

        fn generate_circom_proof(
            in0: String,
            in1: HashMap<String, Vec<String>>,
        ) -> Result<GenerateProofResult, MoproError> {
            let name = std::path::Path::new(in0.as_str()).file_name().unwrap();
            if let Ok(witness_fn) =
                mopro_ffi::circom::zkey_witness_map(&CIRCOM_CIRCUITS, &name.to_str().unwrap())
            {
                mopro_ffi::generate_circom_proof_wtns(in0, in1, witness_fn)
            } else {
                Err(MoproError::CircomError("Unknown ZKEY".to_string()))
            }
        }

        fn verify_circom_proof(
            in0: String,
            in1: Vec<u8>,
            in2: Vec<u8>,
        ) -> Result<bool, MoproError> {
            mopro_ffi::verify_circom_proof(in0, in1, in2)
        }

        fn to_ethereum_proof(in0: Vec<u8>) -> ProofCalldata {
            mopro_ffi::to_ethereum_proof(in0)
        }

        fn to_ethereum_inputs(in0: Vec<u8>) -> Vec<String> {
            mopro_ffi::to_ethereum_inputs(in0)
        }

        uniffi::include_scaffolding!("mopro");
    };
}

#[macro_export]
macro_rules! set_circom_circuits {
    // Generates a function `set_circom_circuits` that takes no arguments and updates CIRCOM_CIRCUITS
    ($($key:expr, $func:expr),+ $(,)?) => {
        fn set_circom_circuits() -> HashMap<String, WtnsFn> {
            let mut m: HashMap<String, WtnsFn> = HashMap::new();

            $(
                    m.insert($key.to_string(), $func);
            )+

            m
        }
    };
}
