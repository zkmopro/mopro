use std::collections::HashMap;
use std::error::Error;

use anyhow::Result;

pub type Halo2ProveFn =
    fn(&str, &str, HashMap<String, Vec<String>>) -> Result<(Vec<u8>, Vec<u8>), Box<dyn Error>>;

pub type Halo2VerifyFn = fn(&str, &str, Vec<u8>, Vec<u8>) -> Result<bool, Box<dyn Error>>;

#[macro_export]
macro_rules! halo2_app {
    ($(($prove_key:expr, $prove_fn:expr, $verify_key:expr, $verify_fn:expr)),+ $(,)?) => {
        $crate::set_halo2_circuits! {
            $(
                ($prove_key, $prove_fn, $verify_key, $verify_fn),
            )+
        }
        $crate::halo2_setup!();
    }
}

/// This macro is provided for backward compatibility.
#[macro_export]
macro_rules! set_halo2_circuits {
    () => {
        #[allow(unused)]
        fn get_halo2_proving_circuit(circuit_pk: &str) -> Result<$crate::halo2::Halo2ProveFn, MoproError> {
            Err(MoproError::Halo2Error("No Halo2 circuits set".to_string()))
        }

        #[allow(unused)]
        fn get_halo2_verifying_circuit(circuit_vk: &str) -> Result<$crate::halo2::Halo2VerifyFn, MoproError> {
            Err(MoproError::Halo2Error("No Halo2 circuits set".to_string()))
        }
    };
    ($(($prove_key:expr, $prove_fn:expr, $verify_key:expr, $verify_fn:expr)),+ $(,)?) => {
        fn get_halo2_proving_circuit(circuit_pk: &str) -> Result<$crate::halo2::Halo2ProveFn, MoproError> {
            match circuit_pk {
                $(
                    $prove_key => Ok($prove_fn),
                )+
                _ => Err(MoproError::Halo2Error(format!("Unknown proving key: {}", circuit_pk)))
            }
        }

        fn get_halo2_verifying_circuit(circuit_vk: &str) -> Result<$crate::halo2::Halo2VerifyFn, MoproError> {
            match circuit_vk {
                $(
                    $verify_key => Ok($verify_fn),
                )+
                _ => Err(MoproError::Halo2Error(format!("Unknown verifying key: {}", circuit_vk)))
            }
        }
    }
}

/// Set the Halo2 circuits that can be used within the mopro library.
/// Provide the circuits you want to be able to generate and verify proofs for
/// as a list of quadruples in the form `(prove_key, prove_fn, verify_key, verify_fn)`.
/// Where `prove_key` is the name of the proving key file, `prove_fn` is the function
/// that generates the proof, `verify_key` is the name of the verifying key file, and
/// `verify_fn` is the function that verifies the proof.
///
/// ## How to use:
/// This macro should only be used once in the same module as the `mopro_ffi::app!()`.
/// Ensure that the `mopro-ffi/halo2` feature is enabled to use this macro.
///
/// #### Example:
///
/// ```ignore
/// mopro_ffi::app!();
///
/// set_halo2_circuits! {
///   (
///     "circuit1_proving_key", circuit1_prove_function,
///     "circuit1_verifying_key", circuit1_verify_function
///   ),
///   (
///     "circuit2_proving_key", circuit2_prove_function,
///     "circuit2_verifying_key", circuit2_verify_function
///   )
/// }
/// ```
///
/// ## For Advanced Users:
/// This macro abstracts away the implementation of:
/// - `get_halo2_proving_circuit(circuit_pk: &str) -> Result<mopro_ffi::halo2::Halo2ProveFn>`
/// - `get_halo2_verifying_circuit(circuit_vk: &str) -> Result<mopro_ffi::halo2::Halo2VerifyFn>`
///
/// You can choose to implement these functions directly with your custom logic:
///
/// #### Example:
/// ```ignore
/// fn get_halo2_proving_circuit(circuit_pk: &str) -> Result<mopro_ffi::halo2::Halo2ProveFn> {
///    match circuit_pk {
///       "circuit1_proving_key" => Ok(circuit1_prove_function),
///       "circuit2_proving_key" => Ok(circuit1_prove_function),
///       _ => Err(mopro_ffi::MoproError::Halo2Error(format!("Unknown proving key: {}", circuit_pk).to_string()))
///    }
/// }
///
/// fn get_halo2_verifying_circuit(circuit_vk: &str) -> Result<mopro_ffi::halo2::Halo2VerifyFn> {
///    match circuit_vk {
///       "circuit1_verifying_key" => Ok(circuit1_verify_function),
///       "circuit2_verifying_key" => Ok(circuit2_verify_function),
///       _ => Err(mopro_ffi::MoproError::Halo2Error(format!("Unknown verifying key: {}", circuit_vk).to_string()))
///    }
/// }
/// ```
#[macro_export]
macro_rules! halo2_setup {
    () => {
        #[derive(Debug, Clone)]
        #[cfg_attr(any(target_os = "ios", target_os = "android"), derive(uniffi::Record))]
        #[cfg_attr(
            target_arch = "wasm32",
            wasm_bindgen::prelude::wasm_bindgen(getter_with_clone)
        )]
        pub struct Halo2ProofResult {
            pub proof: Vec<u8>,
            pub inputs: Vec<u8>,
        }

        #[cfg_attr(any(target_os = "ios", target_os = "android"), uniffi::export)] // TODO - this does not work in the target crate
        #[cfg(not(target_arch = "wasm32"))]
        pub fn generate_halo2_proof(
            srs_path: String,
            pk_path: String,
            circuit_inputs: std::collections::HashMap<String, Vec<String>>,
        ) -> Result<Halo2ProofResult, MoproError> {
            let name = std::path::Path::new(pk_path.as_str()).file_name().unwrap();
            let proving_fn = get_halo2_proving_circuit(name.to_str().unwrap()).map_err(|e| {
                MoproError::Halo2Error(format!("error getting proving circuit: {}", e))
            })?;
            proving_fn(&srs_path, &pk_path, circuit_inputs)
                .map(|(proof, inputs)| Halo2ProofResult { proof, inputs })
                .map_err(|e| MoproError::Halo2Error(format!("halo2 error: {}", e)))
        }

        #[cfg(target_arch = "wasm32")]
        #[wasm_bindgen::prelude::wasm_bindgen]
        pub fn generate_halo2_proof(
            srs_path: String,
            pk_path: String,
            circuit_inputs: wasm_bindgen::JsValue,
        ) -> Result<Halo2ProofResult, MoproError> {
            let circuit_inputs: std::collections::HashMap<String, Vec<String>> =
                serde_wasm_bindgen::from_value(circuit_inputs).map_err(|e| {
                    MoproError::Halo2Error(format!("Failed to parse the circuit_inputs {}", e))
                })?;

            let name = std::path::Path::new(pk_path.as_str()).file_name().unwrap();
            let proving_fn = get_halo2_proving_circuit(name.to_str().unwrap()).map_err(|e| {
                MoproError::Halo2Error(format!("error getting proving circuit: {}", e))
            })?;
            proving_fn(&srs_path, &pk_path, circuit_inputs)
                .map(|(proof, inputs)| Halo2ProofResult { proof, inputs })
                .map_err(|e| MoproError::Halo2Error(format!("halo2 error: {}", e)))
        }

        #[cfg_attr(any(target_os = "ios", target_os = "android"), uniffi::export)]
        #[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
        pub fn verify_halo2_proof(
            srs_path: String,
            vk_path: String,
            proof: Vec<u8>,
            public_input: Vec<u8>,
        ) -> Result<bool, MoproError> {
            let name = std::path::Path::new(vk_path.as_str()).file_name().unwrap();
            let verifying_fn =
                get_halo2_verifying_circuit(name.to_str().unwrap()).map_err(|e| {
                    MoproError::Halo2Error(format!("error getting verification circuit: {}", e))
                })?;
            verifying_fn(&srs_path, &vk_path, proof, public_input)
                .map_err(|e| MoproError::Halo2Error(format!("error verifying proof: {}", e)))
        }
    };
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    // Import the logic generated by `setup!` macros
    use crate::*;

    halo2_app!(
        (
            "plonk_fibonacci_pk.bin",
            plonk_fibonacci::prove,
            "plonk_fibonacci_vk.bin",
            plonk_fibonacci::verify
        ),
        (
            "gemini_fibonacci_pk.bin",
            gemini_fibonacci::prove,
            "gemini_fibonacci_vk.bin",
            gemini_fibonacci::verify
        ),
        (
            "hyperplonk_fibonacci_pk.bin",
            hyperplonk_fibonacci::prove,
            "hyperplonk_fibonacci_vk.bin",
            hyperplonk_fibonacci::verify
        ),
    );

    #[test]
    fn test_generate_and_verify_plonk_proof() {
        const SRS_KEY_PATH: &str = "../test-vectors/halo2/plonk_fibonacci_srs.bin";
        const PROVING_KEY_PATH: &str = "../test-vectors/halo2/plonk_fibonacci_pk.bin";
        const VERIFYING_KEY_PATH: &str = "../test-vectors/halo2/plonk_fibonacci_vk.bin";

        let mut input = HashMap::new();
        input.insert("out".to_string(), vec!["55".to_string()]);

        if let Ok(proof_result) = generate_halo2_proof(
            SRS_KEY_PATH.to_string(),
            PROVING_KEY_PATH.to_string(),
            input,
        ) {
            let result = verify_halo2_proof(
                SRS_KEY_PATH.to_string(),
                VERIFYING_KEY_PATH.to_string(),
                proof_result.proof,
                proof_result.inputs,
            );
            assert!(result.is_ok());
        } else {
            panic!("Failed to generate the proof!")
        }
    }

    #[test]
    fn test_generate_and_verify_hyperplonk_proof() {
        const SRS_KEY_PATH: &str = "../test-vectors/halo2/hyperplonk_fibonacci_srs.bin";
        const PROVING_KEY_PATH: &str = "../test-vectors/halo2/hyperplonk_fibonacci_pk.bin";
        const VERIFYING_KEY_PATH: &str = "../test-vectors/halo2/hyperplonk_fibonacci_vk.bin";

        let mut input = HashMap::new();
        input.insert("out".to_string(), vec!["55".to_string()]);

        if let Ok(proof_result) = generate_halo2_proof(
            SRS_KEY_PATH.to_string(),
            PROVING_KEY_PATH.to_string(),
            input,
        ) {
            let result = verify_halo2_proof(
                SRS_KEY_PATH.to_string(),
                VERIFYING_KEY_PATH.to_string(),
                proof_result.proof,
                proof_result.inputs,
            );
            assert!(result.is_ok());
        } else {
            panic!("Failed to generate the proof!")
        }
    }

    #[test]
    fn test_generate_and_verify_gemini_proof() {
        const SRS_KEY_PATH: &str = "../test-vectors/halo2/gemini_fibonacci_srs.bin";
        const PROVING_KEY_PATH: &str = "../test-vectors/halo2/gemini_fibonacci_pk.bin";
        const VERIFYING_KEY_PATH: &str = "../test-vectors/halo2/gemini_fibonacci_vk.bin";

        let mut input = HashMap::new();
        input.insert("out".to_string(), vec!["55".to_string()]);

        if let Ok(proof_result) = generate_halo2_proof(
            SRS_KEY_PATH.to_string(),
            PROVING_KEY_PATH.to_string(),
            input,
        ) {
            let result = verify_halo2_proof(
                SRS_KEY_PATH.to_string(),
                VERIFYING_KEY_PATH.to_string(),
                proof_result.proof,
                proof_result.inputs,
            );
            assert!(result.is_ok());
        } else {
            panic!("Failed to generate the proof!")
        }
    }
}
