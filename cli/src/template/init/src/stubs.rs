#[macro_export]
macro_rules! circom_stub {
    () => {
        mod circom_stub {
            use crate::error::MoproError;

            #[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
            pub struct CircomProofResult {
                pub proof: CircomProof,
                pub inputs: Vec<String>,
            }

            #[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
            pub struct G1 {
                pub x: String,
                pub y: String,
                pub z: String,
            }

            #[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
            pub struct G2 {
                pub x: Vec<String>,
                pub y: Vec<String>,
                pub z: Vec<String>,
            }

            #[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
            pub struct CircomProof {
                pub a: G1,
                pub b: G2,
                pub c: G1,
                pub protocol: String,
                pub curve: String,
            }

            #[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
            pub enum ProofLib {
                Arkworks,
                Rapidsnark,
            }

            #[cfg_attr(feature = "uniffi", uniffi::export)]
            pub fn generate_circom_proof(
                _zkey_path: String,
                _circuit_inputs: String,
                _proof_lib: ProofLib,
            ) -> Result<CircomProofResult, MoproError> {
                panic!("Circom is not enabled in this build. Please select \"circom\" adapter when initializing the project.");
            }

            #[cfg_attr(feature = "uniffi", uniffi::export)]
            pub fn verify_circom_proof(
                _zkey_path: String,
                _proof_result: CircomProofResult,
                _proof_lib: ProofLib,
            ) -> Result<bool, MoproError> {
                panic!("Circom is not enabled in this build. Please select \"circom\" adapter when initializing the project.");
            }
        }
        pub use circom_stub::{
            generate_circom_proof, verify_circom_proof, CircomProof, CircomProofResult, ProofLib,
            G1, G2,
        };
    };
}

#[macro_export]
macro_rules! halo2_stub {
    () => {
        mod halo2_stub {
            use crate::error::MoproError;

            #[derive(Debug, Clone, Default)]
            #[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
            pub struct Halo2ProofResult {
                pub proof: Vec<u8>,
                pub inputs: Vec<u8>,
            }

            #[cfg_attr(feature = "uniffi", uniffi::export)]
            pub fn generate_halo2_proof(
                _srs_path: String,
                _pk_path: String,
                _circuit_inputs: std::collections::HashMap<String, Vec<String>>,
            ) -> Result<Halo2ProofResult, MoproError> {
                panic!("Halo2 is not enabled in this build. Please select \"halo2\" adapter when initializing the project.");
            }

            #[cfg_attr(feature = "uniffi", uniffi::export)]
            pub fn verify_halo2_proof(
                _srs_path: String,
                _vk_path: String,
                _proof: Vec<u8>,
                _public_input: Vec<u8>,
            ) -> Result<bool, MoproError> {
                panic!("Halo2 is not enabled in this build. Please select \"halo2\" adapter when initializing the project.");
            }
        }
        pub use halo2_stub::{generate_halo2_proof, verify_halo2_proof, Halo2ProofResult};
    };
}

#[macro_export]
macro_rules! noir_stub {
    () => {
        mod noir_stub {
            use crate::error::MoproError;

            #[cfg_attr(feature = "uniffi", uniffi::export)]
            pub fn generate_noir_proof(
                _circuit_path: String,
                _srs_path: Option<String>,
                _inputs: Vec<String>,
                _on_chain: bool,
                _vk: Vec<u8>,
                _low_memory_mode: bool,
            ) -> Result<Vec<u8>, MoproError> {
                panic!("Noir is not enabled in this build. Please select \"noir\" adapter when initializing the project.");
            }

            #[cfg_attr(feature = "uniffi", uniffi::export)]
            pub fn verify_noir_proof(
                _circuit_path: String,
                _proof: Vec<u8>,
                _on_chain: bool,
                _vk: Vec<u8>,
                _low_memory_mode: bool,
            ) -> Result<bool, MoproError> {
                panic!("Noir is not enabled in this build. Please select \"noir\" adapter when initializing the project.");

            }


            #[cfg_attr(feature = "uniffi", uniffi::export)]
            pub fn get_noir_verification_key(
                _circuit_path: String,
                _srs_path: Option<String>,
                _on_chain: bool,
                _low_memory_mode: bool,
            ) -> Result<Vec<u8>, MoproError> {
                panic!("Noir is not enabled in this build. Please select \"noir\" adapter when initializing the project.");

            }


        }
        pub use noir_stub::{
            generate_noir_proof, get_noir_verification_key, verify_noir_proof,
        };
    };
}
