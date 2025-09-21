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
                on_chain: bool,
                vk: Vec<u8>,
                low_memory_mode: bool,
            ) -> Result<Vec<u8>, MoproError> {
                panic!("Noir is not enabled in this build. Please select \"noir\" adapter when initializing the project.");
            }

            #[uniffi::export]
            pub(crate) fn verify_noir_proof(
                circuit_path: String,
                proof: Vec<u8>,
                on_chain: bool,
                vk: Vec<u8>,
                low_memory_mode: bool,
            ) -> Result<bool, MoproError> {
                panic!("Noir is not enabled in this build. Please select \"noir\" adapter when initializing the project.");

            }


            #[uniffi::export]
            pub(crate) fn get_noir_verification_key(
                circuit_path: String,
                srs_path: Option<String>,
                on_chain: bool,
                low_memory_mode: bool,
            ) -> Result<Vec<u8>, MoproError> {
                panic!("Noir is not enabled in this build. Please select \"noir\" adapter when initializing the project.");

            }


        }
    };
}
