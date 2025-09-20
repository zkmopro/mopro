#[macro_export]
macro_rules! circom_stub {
    () => {
        mod circom_stub {
            use crate::MoproError;

            #[derive(uniffi::Record)]
            pub struct CircomProofResult {
                pub proof: CircomProof,
                pub inputs: Vec<String>,
            }

            #[derive(uniffi::Record)]
            pub struct G1 {
                pub x: String,
                pub y: String,
                pub z: String,
            }

            #[derive(uniffi::Record)]
            pub struct G2 {
                pub x: Vec<String>,
                pub y: Vec<String>,
                pub z: Vec<String>,
            }

            #[derive(uniffi::Record)]
            pub struct CircomProof {
                pub a: G1,
                pub b: G2,
                pub c: G1,
                pub protocol: String,
                pub curve: String,
            }

            #[derive(uniffi::Enum)]
            pub enum ProofLib {
                Arkworks,
                Rapidsnark,
            }

            #[uniffi::export]
            pub(crate) fn generate_circom_proof(
                zkey_path: String,
                circuit_inputs: String,
                proof_lib: ProofLib,
            ) -> Result<CircomProofResult, MoproError> {
                panic!("Circom is not enabled in this build. Please select \"circom\" adapter when initializing the project.");
            }

            #[uniffi::export]
            pub(crate) fn verify_circom_proof(
                zkey_path: String,
                proof_result: CircomProofResult,
                proof_lib: ProofLib,
            ) -> Result<bool, MoproError> {
                panic!("Circom is not enabled in this build. Please select \"circom\" adapter when initializing the project.");
            }
        }
    };
}
