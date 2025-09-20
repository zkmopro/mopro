#[macro_export]
macro_rules! noir_stub {
    () => {
        mod noir_stub {
            use crate::MoproError;

            #[uniffi::export]
            pub(crate) fn generate_noir_proof(
                circuit_path: String,
                srs_path: Option<String>,
                inputs: Vec<String>,
            ) -> Result<Vec<u8>, MoproError> {
                panic!("Noir is not enabled in this build. Please select \"noir\" adapter when initializing the project.");
            }

            #[uniffi::export]
            pub(crate) fn verify_noir_proof(circuit_path: String, proof: Vec<u8>) -> Result<bool, MoproError> {
                panic!("Noir is not enabled in this build. Please select \"noir\" adapter when initializing the project.");

            }
        }
    };
}
