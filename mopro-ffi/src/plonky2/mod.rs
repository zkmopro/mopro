use std::collections::HashMap;
use std::error::Error;

#[macro_export]
macro_rules! plonky2_app {
    () => {
        fn generate_plonky2_proof(
            in0: String,
            in1: std::collections::HashMap<String, Vec<String>>,
        ) -> Result<Vec<u8>, mopro_ffi::MoproError> {
            let name = std::path::Path::new(in0.as_str()).file_name().unwrap();
            let proving_fn = get_plonky2_proving_circuit(name.to_str().unwrap()).map_err(|e| {
                mopro_ffi::MoproError::Plonky2Error(format!("error getting proving circuit: {}", e))
            })?;
            proving_fn(&in0, in1)
                .map_err(|e| mopro_ffi::MoproError::Plonky2Error(format!("plonky2 error: {}", e)))
        }

        fn verify_plonky2_proof(in0: String, in1: Vec<u8>) -> Result<bool, mopro_ffi::MoproError> {
            let name = std::path::Path::new(in0.as_str()).file_name().unwrap();
            let verifying_fn =
                get_plonky2_verifying_circuit(name.to_str().unwrap()).map_err(|e| {
                    mopro_ffi::MoproError::Plonky2Error(format!(
                        "error getting verification circuit: {}",
                        e
                    ))
                })?;
            verifying_fn(&in0, in1).map_err(|e| {
                mopro_ffi::MoproError::Plonky2Error(format!("error verifying proof: {}", e))
            })
        }
    };
}

/// Set the plonky2 circuits that can be used within the mopro library.
/// Provide the circuits you want to be able to generate and verify proofs for
/// as a list of quadruples in the form `(prover_data, prove_fn, verifier_data, verify_fn)`.
/// Where `prover_data` is the name of the prover data file, `prove_fn` is the function
/// that generates the proof, `verifier_data` is the name of the verifier data file, and
/// `verify_fn` is the function that verifies the proof.
///
/// ## How to use:
/// This macro should only be used once in the same module as the `mopro_ffi::app!()`.
/// Ensure that the `mopro-ffi/plonky2` feature is enabled to use this macro.
///
/// #### Example:
///
/// ```ignore
/// mopro_ffi::app!();
///
/// set_plonky2_circuits! {
///   (
///     "circuit1_prover_data", circuit1_prove_function,
///     "circuit1_verifier_data", circuit1_verify_function
///   ),
///   (
///     "circuit2_prover_data", circuit2_prove_function,
///     "circuit2_verifier_data", circuit2_verify_function
///   )
/// }
/// ```
///
/// ## For Advanced Users:
/// This macro abstracts away the implementation of:
/// - `get_plonky2_proving_circuit(circuit_pk: &str) -> Result<mopro_ffi::plonky2ProveFn, mopro_ffi::MoproError>`
/// - `get_plonky2_verifying_circuit(circuit_vk: &str) -> Result<mopro_ffi::plonky2VerifyFn, mopro_ffi::MoproError>`
///
/// You can choose to implement these functions directly with your custom logic:
///
/// #### Example:
/// ```ignore
/// fn get_plonky2_proving_circuit(circuit_pk: &str) -> Result<mopro_ffi::plonky2ProveFn, mopro_ffi::MoproError> {
///    match circuit_pk {
///       "circuit1_prover_data" => Ok(circuit1_prove_function),
///       "circuit2_prover_data" => Ok(circuit2_prove_function),
///       _ => Err(mopro_ffi::MoproError::Plonky2Error(format!("Unknown proving key: {}", circuit_pk).to_string()))
///    }
/// }
///
/// fn get_plonky2_verifying_circuit(circuit_vk: &str) -> Result<mopro_ffi::plonky2VerifyFn, mopro_ffi::MoproError> {
///    match circuit_vk {
///       "circuit1_verifier_data" => Ok(circuit1_verify_function),
///       "circuit2_verifier_data" => Ok(circuit2_verify_function),
///       _ => Err(mopro_ffi::MoproError::Plonky2Error(format!("Unknown verifying key: {}", circuit_vk).to_string()))
///    }
/// }
/// ```
#[macro_export]
macro_rules! set_plonky2_circuits {
    ($(($prove_key:expr, $prove_fn:expr, $verify_key:expr, $verify_fn:expr)),+ $(,)?) => {
        fn get_plonky2_proving_circuit(circuit_pk: &str) -> Result<mopro_ffi::Plonky2ProveFn, mopro_ffi::MoproError> {
            match circuit_pk {
                $(
                    $prove_key => Ok($prove_fn),
                )+
                _ => Err(mopro_ffi::MoproError::Plonky2Error(format!("Unknown proving key: {}", circuit_pk)))
            }
        }

        fn get_plonky2_verifying_circuit(circuit_vk: &str) -> Result<mopro_ffi::Plonky2VerifyFn, mopro_ffi::MoproError> {
            match circuit_vk {
                $(
                    $verify_key => Ok($verify_fn),
                )+
                _ => Err(mopro_ffi::MoproError::Plonky2Error(format!("Unknown verifying key: {}", circuit_vk)))
            }
        }
    };
}

pub type Plonky2ProveFn = fn(&str, HashMap<String, Vec<String>>) -> Result<Vec<u8>, Box<dyn Error>>;

pub type Plonky2VerifyFn = fn(&str, Vec<u8>) -> Result<bool, Box<dyn Error>>;

#[cfg(test)]
mod test {
    use plonky2_fibonacci::{plonky2_prove, plonky2_verify};

    use crate as mopro_ffi;
    use std::collections::HashMap;

    #[test]
    fn test_generate_and_verify_proof() {
        plonky2_app!();

        set_plonky2_circuits! {
            ("plonky2_fibonacci_pk.bin", plonky2_prove, "plonky2_fibonacci_vk.bin", plonky2_verify),
        }

        const PROVER_DATA_PATH: &str = "../test-vectors/plonky2/plonky2_fibonacci_pk.bin";
        const VERIFIER_DATA_PATH: &str = "../test-vectors/plonky2/plonky2_fibonacci_vk.bin";

        let mut input = HashMap::new();
        input.insert("a".to_string(), vec!["0".to_string()]);
        input.insert("b".to_string(), vec!["1".to_string()]);

        if let Ok(proof_result) = generate_plonky2_proof(PROVER_DATA_PATH.to_string(), input) {
            let result = verify_plonky2_proof(VERIFIER_DATA_PATH.to_string(), proof_result);
            assert!(result.is_ok());
        } else {
            panic!("Failed to generate the proof!")
        }
    }
}
