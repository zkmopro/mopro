use std::error::Error;

type GkrProofResult = (String, Vec<u8>);

pub type GkrProveFn = fn(Vec<u8>) -> Result<GkrProofResult, Box<dyn Error>>;

pub type GkrVerifyFn = fn(Vec<u8>, &str, Vec<u8>) -> Result<bool, Box<dyn Error>>;

#[macro_export]
macro_rules! gkr_app {
    () => {
        fn generate_gkr_proof(
            in0: String,
            in1: Vec<u8>,
        ) -> Result<mopro_ffi::GkrProofResult, mopro_ffi::MoproError> {
            let name = in0;
            let proving_fn = get_gkr_proving_circuit(&name).map_err(|e| {
                mopro_ffi::MoproError::GkrError(format!("error getting proving circuit: {}", e))
            })?;
            proving_fn(in1)
                .map(|(output_claims, proof)| mopro_ffi::GkrProofResult {
                    output_claims,
                    proof,
                })
                .map_err(|e| mopro_ffi::MoproError::GkrError(format!("gkr error: {}", e)))
        }

        fn verify_gkr_proof(
            in0: String,
            in1: Vec<u8>,
            in2: String,
            in3: Vec<u8>,
        ) -> Result<bool, mopro_ffi::MoproError> {
            let name = in0;
            let verifying_fn = get_gkr_verifying_circuit(&name).map_err(|e| {
                mopro_ffi::MoproError::GkrError(format!(
                    "error getting verification circuit: {}",
                    e
                ))
            })?;
            verifying_fn(in1, &in2, in3).map_err(|e| {
                mopro_ffi::MoproError::GkrError(format!("error verifying proof: {}", e))
            })
        }
    };
}

#[macro_export]
macro_rules! set_gkr_circuits {
    ($(($key:expr, $prove_fn:expr, $verify_fn:expr)),+ $(,)?) => {
        fn get_gkr_proving_circuit(circuit: &str) -> Result<mopro_ffi::GkrProveFn, mopro_ffi::MoproError> {
            match circuit {
                $(
                    $key => Ok($prove_fn),
                )+
                _ => Err(mopro_ffi::MoproError::GkrError(format!("Unknown proving key: {}", circuit)))
            }
        }

        fn get_gkr_verifying_circuit(circuit: &str) -> Result<mopro_ffi::GkrVerifyFn, mopro_ffi::MoproError> {
            match circuit {
                $(
                    $key => Ok($verify_fn),
                )+
                _ => Err(mopro_ffi::MoproError::GkrError(format!("Unknown verifying key: {}", circuit)))
            }
        }
    };
}
#[cfg(test)]
mod test {
    use crate as mopro_ffi;

    gkr_app!();

    set_gkr_circuits! {
        ("keccak256", gkr_keccak256::prove, gkr_keccak256::verify),
    }

    #[test]
    fn test_generate_and_verify_gkr_proof() {
        let input: Vec<u8> = vec![
            116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0,
        ];

        if let Ok(proof_result) = generate_gkr_proof("keccak256".to_string(), input.clone()) {
            let result = verify_gkr_proof(
                "keccak256".to_string(),
                input,
                proof_result.output_claims,
                proof_result.proof,
            );

            assert!(result.is_ok());
        } else {
            panic!("Failed to generate the proof!")
        }
    }
}
