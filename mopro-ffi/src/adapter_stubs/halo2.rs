#[macro_export]
macro_rules! halo2_stub {
    () => {
        mod halo2_stub {
            use crate::MoproError;

            #[derive(uniffi::Record)]
            pub struct Halo2ProofResult {
                pub proof: Vec<u8>,
                pub inputs: Vec<u8>,
            }

            #[uniffi::export]
            pub(crate) fn generate_halo2_proof(
                srs_path: String,
                pk_path: String,
                circuit_inputs: std::collections::HashMap<String, Vec<String>>,
            ) -> Result<Halo2ProofResult, MoproError> {
                panic!("Halo2 is not enabled in this build. Please select \"halo2\" adapter when initializing the project.");
            }

            #[uniffi::export]
            pub(crate) fn verify_halo2_proof(
                srs_path: String,
                vk_path: String,
                proof: Vec<u8>,
                public_input: Vec<u8>,
            ) -> Result<bool, MoproError> {
                panic!("Halo2 is not enabled in this build. Please select \"halo2\" adapter when initializing the project.");
            }
        }
    };
}
