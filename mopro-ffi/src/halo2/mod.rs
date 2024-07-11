use crate::{GenerateProofResult, MoproError};
use std::collections::HashMap;

#[macro_export]
macro_rules! halo2_app {
    () => {
        static HALO2_PROVING_CIRCUITS: once_cell::sync::Lazy<
            HashMap<String, mopro_ffi::Halo2ProveFn>,
        > = once_cell::sync::Lazy::new(|| set_halo2_proving_circuits());

        static HALO2_VERIFYING_CIRCUITS: once_cell::sync::Lazy<
            HashMap<String, mopro_ffi::Halo2VerifyFn>,
        > = once_cell::sync::Lazy::new(|| set_halo2_verifying_circuits());

        fn generate_halo2_proof(
            in0: String,
            in1: String,
            in2: HashMap<String, Vec<String>>,
        ) -> Result<GenerateProofResult, MoproError> {
            let name = std::path::Path::new(in1.as_str()).file_name().unwrap();
            if let Some(prove_fn) = HALO2_PROVING_CIRCUITS.get(name.to_str().unwrap()) {
                prove_fn(&in0, &in1, in2)
            } else {
                Err(MoproError::Halo2Error(
                    format!(
                        "Unknown Prove Circuit: {}. Have Prove Circuits: {:?}",
                        in0,
                        HALO2_PROVING_CIRCUITS.keys()
                    )
                    .to_string(),
                ))
            }
        }

        fn verify_halo2_proof(
            in0: String,
            in1: String,
            in2: Vec<u8>,
            in3: Vec<u8>,
        ) -> Result<bool, MoproError> {
            let name = std::path::Path::new(in1.as_str()).file_name().unwrap();
            if let Some(verify_fn) = HALO2_VERIFYING_CIRCUITS.get(name.to_str().unwrap()) {
                verify_fn(&in0, &in1, in2, in3)
            } else {
                Err(MoproError::Halo2Error(
                    format!(
                        "Unknown Verify Circuit: {}. Have Verify Circuits: {:?}",
                        in0,
                        HALO2_VERIFYING_CIRCUITS.keys()
                    )
                    .to_string(),
                ))
            }
        }
    };
}

/// Proving Circuits are Halo2 Circuits that can generate proofs
/// Provide the circuits that you want to be able to generate proofs for
/// as a list of pairs of the form `circuit_proving_key`, `prove_fn`
/// Where `circuit_proving_key` is the name of the proving key file
/// and `prove_fn` is the function that generates the proof available locally
/// NOTE: YOU CAN ONLY USE THIS MACROS ONCE IN YOUR CODEBASE, IN THE SAME MODULE AS `app!()`
/// NOTE: TO USE THIS MACRO, YOU MUST HAVE THE `mopro-ffi/halo2` FEATURE ENABLED
#[macro_export]
macro_rules! set_halo2_proving_circuits {
    // Generates a function `set_circom_circuits` that takes no arguments and updates CIRCOM_CIRCUITS
    ($($key:expr, $func:expr),+ $(,)?) => {
        fn set_halo2_proving_circuits() -> HashMap<String, mopro_ffi::Halo2ProveFn> {
            let mut circuits: HashMap<String, mopro_ffi::Halo2ProveFn> = HashMap::new();

            $(
                    circuits.insert($key.to_string(), $func);
            )+

            circuits
        }
    };
}

/// Verifying Circuits are Halo2 Circuits that can verify proofs
/// Provide the circuits that you want to be able to verify proofs for
/// as a list of pairs of the form `circuit_verifying_key`, `verify_fn`
/// Where `circuit_verifying_key` is the name of the verifying key file
/// and `verify_fn` is the function that verifies the proof available locally
/// NOTE: YOU CAN ONLY USE THIS MACROS ONCE IN YOUR CODEBASE, IN THE SAME MODULE AS `app!()`
/// NOTE: TO USE THIS MACRO, YOU MUST HAVE THE `mopro-ffi/halo2` FEATURE ENABLED
#[macro_export]
macro_rules! set_halo2_verifying_circuits {
    // Generates a function `set_circom_circuits` that takes no arguments and updates CIRCOM_CIRCUITS
    ($($key:expr, $func:expr),+ $(,)?) => {
        fn set_halo2_verifying_circuits() -> HashMap<String, mopro_ffi::Halo2VerifyFn> {
            let mut circuits: HashMap<String, mopro_ffi::Halo2VerifyFn> = HashMap::new();

            $(
                    circuits.insert($key.to_string(), $func);
            )+

            circuits
        }
    };
}

pub type Halo2ProveFn =
    fn(&str, &str, HashMap<String, Vec<String>>) -> Result<GenerateProofResult, MoproError>;

pub type Halo2VerifyFn = fn(&str, &str, Vec<u8>, Vec<u8>) -> Result<bool, MoproError>;
