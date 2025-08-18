use noir_rs::{
    barretenberg::{
        prove::{prove_ultra_honk, prove_ultra_honk_keccak},
        srs::setup_srs_from_bytecode,
        verify::{
            get_ultra_honk_keccak_verification_key, get_ultra_honk_verification_key,
            verify_ultra_honk, verify_ultra_honk_keccak,
        },
    },
    witness::from_vec_str_to_witness_map,
};

#[macro_export]
macro_rules! noir_app {
    ($err:ty) => {
        #[cfg_attr(not(feature = "no_uniffi_exports"), uniffi::export)]
        fn generate_noir_proof(
            circuit_path: String,
            srs_path: Option<String>,
            inputs: Vec<String>,
            low_memory_mode: bool,
        ) -> Result<Vec<u8>, $err> {
            mopro_ffi::generate_noir_proof(circuit_path, srs_path, inputs, low_memory_mode)
                .map_err(|e| <$err>::NoirError(format!("Generate Proof error: {}", e)))
        }

        #[cfg_attr(not(feature = "no_uniffi_exports"), uniffi::export)]
        fn verify_noir_proof(
            circuit_path: String,
            proof: Vec<u8>,
            low_memory_mode: bool,
        ) -> Result<bool, $err> {
            Ok(mopro_ffi::verify_noir_proof(
                circuit_path,
                proof,
                low_memory_mode,
            ))
        }

        #[cfg_attr(not(feature = "no_uniffi_exports"), uniffi::export)]
        fn generate_noir_keccak_proof(
            circuit_path: String,
            srs_path: Option<String>,
            inputs: Vec<String>,
            disable_zk: bool,
            low_memory_mode: bool,
        ) -> Result<Vec<u8>, $err> {
            mopro_ffi::generate_noir_keccak_proof(
                circuit_path,
                srs_path,
                inputs,
                disable_zk,
                low_memory_mode,
            )
            .map_err(|e| <$err>::NoirError(format!("Generate Proof error: {}", e)))
        }

        #[cfg_attr(not(feature = "no_uniffi_exports"), uniffi::export)]
        fn verify_noir_keccak_proof(
            circuit_path: String,
            proof: Vec<u8>,
            disable_zk: bool,
            low_memory_mode: bool,
        ) -> Result<bool, $err> {
            Ok(mopro_ffi::verify_noir_keccak_proof(
                circuit_path,
                proof,
                disable_zk,
                low_memory_mode,
            ))
        }
    };
}

pub fn generate_noir_proof(
    circuit_path: String,
    srs_path: Option<String>,
    inputs: Vec<String>,
    low_memory_mode: bool,
) -> Result<Vec<u8>, String> {
    let circuit_bytecode = get_bytecode(circuit_path);

    // Setup the SRS
    setup_srs_from_bytecode(circuit_bytecode.as_str(), srs_path.as_deref(), false).unwrap();

    // Set up the witness
    let witness = from_vec_str_to_witness_map(inputs.iter().map(|s| s.as_str()).collect()).unwrap();

    // Get the verification key
    let vk = get_ultra_honk_verification_key(circuit_bytecode.as_str(), low_memory_mode).unwrap();

    prove_ultra_honk(circuit_bytecode.as_str(), witness, vk, low_memory_mode)
}

pub fn verify_noir_proof(circuit_path: String, proof: Vec<u8>, low_memory_mode: bool) -> bool {
    let circuit_bytecode = get_bytecode(circuit_path);
    let vk = get_ultra_honk_verification_key(circuit_bytecode.as_str(), low_memory_mode).unwrap();
    verify_ultra_honk(proof, vk).unwrap()
}

pub fn generate_noir_keccak_proof(
    circuit_path: String,
    srs_path: Option<String>,
    inputs: Vec<String>,
    disable_zk: bool,
    low_memory_mode: bool,
) -> Result<Vec<u8>, String> {
    let circuit_bytecode = get_bytecode(circuit_path);

    // Setup the SRS
    setup_srs_from_bytecode(circuit_bytecode.as_str(), srs_path.as_deref(), false).unwrap();

    // Set up the witness
    let witness = from_vec_str_to_witness_map(inputs.iter().map(|s| s.as_str()).collect()).unwrap();

    // Get the verification key
    let vk = get_ultra_honk_keccak_verification_key(
        circuit_bytecode.as_str(),
        disable_zk,
        low_memory_mode,
    )
    .unwrap();

    prove_ultra_honk_keccak(
        circuit_bytecode.as_str(),
        witness,
        vk,
        disable_zk,
        low_memory_mode,
    )
}

pub fn verify_noir_keccak_proof(
    circuit_path: String,
    proof: Vec<u8>,
    disable_zk: bool,
    low_memory_mode: bool,
) -> bool {
    let circuit_bytecode = get_bytecode(circuit_path);
    let vk = get_ultra_honk_keccak_verification_key(
        circuit_bytecode.as_str(),
        disable_zk,
        low_memory_mode,
    )
    .unwrap();
    verify_ultra_honk_keccak(proof, vk, disable_zk).unwrap()
}

fn get_bytecode(circuit_path: String) -> String {
    // Read the JSON manifest of the circuit
    let circuit_txt = std::fs::read_to_string(circuit_path).unwrap();
    let circuit: serde_json::Value = serde_json::from_str(&circuit_txt).unwrap();

    circuit["bytecode"].as_str().unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate as mopro_ffi;

    const MULTIPLIER2_CIRCUIT_FILE: &str = "../test-vectors/noir/noir_multiplier2.json";
    const SRS_FILE: &str = "../test-vectors/noir/noir_multiplier2.srs";

    #[test]
    #[serial_test::serial]
    fn test_proof_multiplier2() {
        let witness = vec!["3".to_string(), "5".to_string()];
        let proof = generate_noir_proof(
            MULTIPLIER2_CIRCUIT_FILE.to_string(),
            Some(SRS_FILE.to_string()),
            witness,
            false,
        )
        .unwrap();
        assert!(verify_noir_proof(
            MULTIPLIER2_CIRCUIT_FILE.to_string(),
            proof,
            false,
        ));
    }

    #[test]
    #[serial_test::serial]
    fn test_proof_multiplier2_low_memory() {
        let witness = vec!["3".to_string(), "5".to_string()];
        let proof = generate_noir_proof(
            MULTIPLIER2_CIRCUIT_FILE.to_string(),
            Some(SRS_FILE.to_string()),
            witness,
            true,
        )
        .unwrap();
        assert!(verify_noir_proof(
            MULTIPLIER2_CIRCUIT_FILE.to_string(),
            proof,
            true,
        ));
    }

    #[test]
    #[serial_test::serial]
    fn test_proof_multiplier2_without_srs_path() {
        let witness = vec!["3".to_string(), "5".to_string()];
        let proof = generate_noir_proof(MULTIPLIER2_CIRCUIT_FILE.to_string(), None, witness, false)
            .unwrap();
        assert!(verify_noir_proof(
            MULTIPLIER2_CIRCUIT_FILE.to_string(),
            proof,
            false,
        ));
    }

    #[test]
    #[serial_test::serial]
    fn test_keccak_proof_multiplier2() {
        let witness = vec!["3".to_string(), "5".to_string()];
        let proof = generate_noir_keccak_proof(
            MULTIPLIER2_CIRCUIT_FILE.to_string(),
            Some(SRS_FILE.to_string()),
            witness,
            false,
            false,
        )
        .unwrap();
        assert!(verify_noir_keccak_proof(
            MULTIPLIER2_CIRCUIT_FILE.to_string(),
            proof,
            false,
            false,
        ));
    }

    #[test]
    #[serial_test::serial]
    fn test_keccak_proof_multiplier2_disable_zk() {
        let witness = vec!["3".to_string(), "5".to_string()];
        let proof = generate_noir_keccak_proof(
            MULTIPLIER2_CIRCUIT_FILE.to_string(),
            Some(SRS_FILE.to_string()),
            witness,
            true,
            false,
        )
        .unwrap();
        assert!(verify_noir_keccak_proof(
            MULTIPLIER2_CIRCUIT_FILE.to_string(),
            proof,
            true,
            false,
        ));
    }

    #[test]
    #[serial_test::serial]
    fn test_keccak_proof_multiplier2_low_memory() {
        let witness = vec!["3".to_string(), "5".to_string()];
        let proof = generate_noir_keccak_proof(
            MULTIPLIER2_CIRCUIT_FILE.to_string(),
            Some(SRS_FILE.to_string()),
            witness,
            false,
            true,
        )
        .unwrap();
        assert!(verify_noir_keccak_proof(
            MULTIPLIER2_CIRCUIT_FILE.to_string(),
            proof,
            false,
            true,
        ));
    }

    #[cfg(feature = "no_uniffi_exports")]
    #[test]
    #[serial_test::serial]
    fn test_noir_app_macro() {
        noir_app!(mopro_ffi::MoproError);

        let witness = vec!["3".to_string(), "5".to_string()];
        let proof_result = generate_noir_proof(
            MULTIPLIER2_CIRCUIT_FILE.to_string(),
            Some(SRS_FILE.to_string()),
            witness,
            false,
        );

        assert!(proof_result.is_ok());
        let proof = proof_result.unwrap();

        let verify_result = verify_noir_proof(MULTIPLIER2_CIRCUIT_FILE.to_string(), proof, false);

        assert!(verify_result.is_ok());
        assert!(verify_result.unwrap());
    }
}
