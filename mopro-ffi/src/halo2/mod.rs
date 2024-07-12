use std::collections::HashMap;
use std::error::Error;

#[macro_export]
macro_rules! halo2_app {
    () => {
        fn generate_halo2_proof(
            in0: String,
            in1: String,
            in2: std::collections::HashMap<String, Vec<String>>,
        ) -> Result<mopro_ffi::GenerateProofResult, mopro_ffi::MoproError> {
            let name = std::path::Path::new(in1.as_str()).file_name().unwrap();
            let proving_fn = get_halo2_proving_circuit(name.to_str().unwrap())?;
            proving_fn(&in0, &in1, in2)
                .map_err(|e| mopro_ffi::MoproError::Halo2Error(e.to_string()))
                .map(|(proof, inputs)| mopro_ffi::GenerateProofResult { proof, inputs })
        }

        fn verify_halo2_proof(
            in0: String,
            in1: String,
            in2: Vec<u8>,
            in3: Vec<u8>,
        ) -> Result<bool, mopro_ffi::MoproError> {
            let name = std::path::Path::new(in1.as_str()).file_name().unwrap();
            let verifying_fn = get_halo2_verifying_circuit(name.to_str().unwrap())?;
            verifying_fn(&in0, &in1, in2, in3)
                .map_err(|e| mopro_ffi::MoproError::Halo2Error(e.to_string()))
        }
    };
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
/// - `get_halo2_proving_circuit(circuit_pk: &str) -> Result<mopro_ffi::Halo2ProveFn, mopro_ffi::MoproError>`
/// - `get_halo2_verifying_circuit(circuit_vk: &str) -> Result<mopro_ffi::Halo2VerifyFn, mopro_ffi::MoproError>`
///
/// You can choose to implement these functions directly with your custom logic:
///
/// #### Example:
/// ```ignore
/// fn get_halo2_proving_circuit(circuit_pk: &str) -> Result<mopro_ffi::Halo2ProveFn, mopro_ffi::MoproError> {
///    match circuit_pk {
///       "circuit1_proving_key" => Ok(circuit1_prove_function),
///       _ => Err(mopro_ffi::MoproError::Halo2Error(format!("Unknown proving key: {}", circuit_pk).to_string()))
///    }
/// }
///
/// fn get_halo2_verifying_circuit(circuit_vk: &str) -> Result<mopro_ffi::Halo2VerifyFn, mopro_ffi::MoproError> {
///    match circuit_vk {
///       "circuit1_verifying_key" => Ok(circuit1_verify_function),
///       _ => Err(mopro_ffi::MoproError::Halo2Error(format!("Unknown verifying key: {}", circuit_vk).to_string()))
///    }
/// }
/// ```
#[macro_export]
macro_rules! set_halo2_circuits {
    ($(($prove_key:expr, $prove_fn:expr, $verify_key:expr, $verify_fn:expr)),+ $(,)?) => {
        fn get_halo2_proving_circuit(circuit_pk: &str) -> Result<mopro_ffi::Halo2ProveFn, mopro_ffi::MoproError> {
            match circuit_pk {
                $(
                    $prove_key => Ok($prove_fn),
                )+
                _ => Err(mopro_ffi::MoproError::Halo2Error(format!("Unknown proving key: {}", circuit_pk))),
            }
        }

        fn get_halo2_verifying_circuit(circuit_vk: &str) -> Result<mopro_ffi::Halo2VerifyFn, mopro_ffi::MoproError> {
            match circuit_vk {
                $(
                    $verify_key => Ok($verify_fn),
                )+
                _ => Err(mopro_ffi::MoproError::Halo2Error(format!("Unknown verifying key: {}", circuit_vk))),
            }
        }
    };
}

type GenerateProofResult = (Vec<u8>, Vec<u8>);

pub type Halo2ProveFn =
    fn(&str, &str, HashMap<String, Vec<String>>) -> Result<GenerateProofResult, Box<dyn Error>>;

pub type Halo2VerifyFn = fn(&str, &str, Vec<u8>, Vec<u8>) -> Result<bool, Box<dyn Error>>;

#[cfg(test)]
mod test {
    use crate as mopro_ffi;
    use std::collections::HashMap;
    use std::error::Error;
    use std::fmt::Display;
    use thiserror::Error;

    #[derive(Debug, Error)]
    pub struct CustomError(String);

    impl Display for CustomError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    fn dummy_prove_fn(
        _srs_key_path: &str,
        _proving_key_path: &str,
        _input: HashMap<String, Vec<String>>,
    ) -> Result<(Vec<u8>, Vec<u8>), Box<dyn Error>> {
        Ok((vec![], vec![]))
    }

    fn dummy_fail_prove_fn(
        _srs_key_path: &str,
        _proving_key_path: &str,
        _input: HashMap<String, Vec<String>>,
    ) -> Result<(Vec<u8>, Vec<u8>), Box<dyn Error>> {
        Err(CustomError("Failed to generate proof".to_string()).into())
    }

    fn dummy_verify_fn(
        _srs_key_path: &str,
        _verifying_key_path: &str,
        _proof: Vec<u8>,
        _public_inputs: Vec<u8>,
    ) -> Result<bool, Box<dyn Error>> {
        Ok(true)
    }

    fn dummy_fail_verify_fn(
        _srs_key_path: &str,
        _verifying_key_path: &str,
        _proof: Vec<u8>,
        _public_inputs: Vec<u8>,
    ) -> Result<bool, Box<dyn Error>> {
        Err(CustomError("Failed to verify proof".to_string()).into())
    }

    halo2_app!();

    set_halo2_circuits! {
        ("fibonacci_pk", dummy_prove_fn, "fibonacci_vk", dummy_verify_fn),
    }

    const SRS_KEY_PATH: &str = "../test-vectors/halo2/fibonacci_srs";
    const PROVING_KEY_PATH: &str = "../test-vectors/halo2/fibonacci_pk";
    const VERIFYING_KEY_PATH: &str = "../test-vectors/halo2/fibonacci_vk";

    #[test]
    fn test_generate_halo2_proof() {
        let result = generate_halo2_proof(
            SRS_KEY_PATH.to_string(),
            PROVING_KEY_PATH.to_string(),
            HashMap::new(),
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_verify_halo2_proof() {
        let result = verify_halo2_proof(
            SRS_KEY_PATH.to_string(),
            VERIFYING_KEY_PATH.to_string(),
            vec![],
            vec![],
        );
        println!("{:?}", result);
        assert!(result.is_ok());
    }
}
