pub use noir_rs::{
    barretenberg::{
        prove::prove_ultra_honk, srs::setup_srs_from_bytecode, utils::get_honk_verification_key,
        verify::verify_ultra_honk,
    },
    witness::from_vec_str_to_witness_map,
};

pub fn get_bytecode(circuit_path: String) -> String {
    // Read the JSON manifest of the circuit
    let circuit_txt = std::fs::read_to_string(circuit_path).unwrap();
    let circuit: serde_json::Value = serde_json::from_str(&circuit_txt).unwrap();

    circuit["bytecode"].as_str().unwrap().to_string()
}

#[macro_export]
macro_rules! noir_app {
    () => {
        #[cfg_attr(not(feature = "no_uniffi_exports"), uniffi::export)]
        fn generate_noir_proof(
            circuit_path: String,
            srs_path: Option<String>,
            inputs: Vec<String>,
        ) -> Result<Vec<u8>, crate::MoproError> {
            let circuit_bytecode = mopro_ffi::noir::get_bytecode(circuit_path);

            mopro_ffi::noir::setup_srs_from_bytecode(circuit_bytecode.as_str(), srs_path.as_deref(), false)
                .map_err(|e| crate::MoproError::NoirError(format!("Setting up SRS error: {}", e)))?;

            let witness = mopro_ffi::noir::from_vec_str_to_witness_map(inputs.iter().map(|s| s.as_str()).collect())
                .map_err(|e| crate::MoproError::NoirError(format!("Setting up Witness Map error: {}", e)))?;

            mopro_ffi::noir::prove_ultra_honk(circuit_bytecode.as_str(), witness, false)
                .map_err(|e| crate::MoproError::NoirError(format!("Generate Proof error: {}", e)))
        }

        #[cfg_attr(not(feature = "no_uniffi_exports"), uniffi::export)]
        fn verify_noir_proof(
            circuit_path: String,
            proof: Vec<u8>,
        ) -> Result<bool, crate::MoproError> {
            let circuit_bytecode = mopro_ffi::noir::get_bytecode(circuit_path);

            let vk = mopro_ffi::noir::get_honk_verification_key(circuit_bytecode.as_str(), false)
                .map_err(|e| crate::MoproError::NoirError(format!("Setting up Verification Key error: {}", e)))?;

            mopro_ffi::noir::verify_ultra_honk(proof, vk)
                .map_err(|e| crate::MoproError::NoirError(format!("Verifying Proof error: {}", e)))
        }
    };
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;
    use std::{collections::HashMap, fs};

    const MULTIPLIER2_CIRCUIT_FILE: &str = "../test-vectors/noir/noir_multiplier2.json";
    const CIRCUIT_FILE: &str = "../test-vectors/noir/zkemail.json";
    const INPUT_FILE: &str = "../test-vectors/noir/zkemail_input.json";
    const SRS_FILE: &str = "../test-vectors/noir/zkemail_srs.local";

    #[test]
    #[serial_test::serial]
    fn test_proof_multiplier2() {
        let witness = vec!["3".to_string(), "5".to_string()];
        let proof =
            generate_noir_proof(MULTIPLIER2_CIRCUIT_FILE.to_string(), None, witness).unwrap();
        assert!(verify_noir_proof(
            MULTIPLIER2_CIRCUIT_FILE.to_string(),
            proof
        ));
    }

    #[test]
    #[serial_test::serial]
    fn test_proof_zkemail() {
        // Load input data from the JSON file for the test case
        let json_str = fs::read_to_string(INPUT_FILE).unwrap();
        let witness = to_zkemail_witness(json_str.as_str());

        let proof = generate_noir_proof(
            CIRCUIT_FILE.to_string(),
            Some(SRS_FILE.to_string()),
            witness,
        )
        .unwrap();

        assert!(verify_noir_proof(CIRCUIT_FILE.to_string(), proof));
    }

    #[cfg(feature = "no_uniffi_exports")]
    #[test]
    #[serial_test::serial]
    fn test_macro_proof_zkemail() {
        noir_app!(mopro_ffi::MoproError);

        // Load input data from the JSON file for the test case
        let json_str = fs::read_to_string(INPUT_FILE).unwrap();
        let witness = to_zkemail_witness(json_str.as_str());

        let proof = generate_noir_proof(
            CIRCUIT_FILE.to_string(),
            Some(SRS_FILE.to_string()),
            witness,
        )
        .unwrap();

        let result = verify_noir_proof(CIRCUIT_FILE.to_string(), proof).unwrap();
        assert!(result);
    }

    fn to_zkemail_witness(json_str: &str) -> Vec<String> {
        #[derive(Deserialize, Debug)]
        struct ZkEmailInputTest {
            header: HeaderTest,
            pubkey: PubKeyTest,
            signature: Vec<String>,
            date_index: u32,
            subject_sequence: SequenceTest,
            from_header_sequence: SequenceTest,
            from_address_sequence: SequenceTest,
        }
        #[derive(Deserialize, Debug)]
        struct HeaderTest {
            storage: Vec<u8>,
            len: u32,
        }
        #[derive(Deserialize, Debug)]
        struct PubKeyTest {
            modulus: Vec<String>,
            redc: Vec<String>,
        }
        #[derive(Deserialize, Debug)]
        struct SequenceTest {
            index: u32,
            length: u32,
        }

        let input_data: ZkEmailInputTest =
            serde_json::from_str(json_str).expect("Failed to parse zkemail_input.json for test");

        // Convert loaded data into the HashMap format required by prove_zkemail
        let mut inputs: HashMap<String, Vec<String>> = HashMap::new();
        inputs.insert(
            "header_storage".to_string(),
            input_data
                .header
                .storage
                .iter()
                .map(|b| b.to_string())
                .collect(),
        );
        inputs.insert(
            "header_len".to_string(),
            vec![input_data.header.len.to_string()],
        );
        inputs.insert("pubkey_modulus".to_string(), input_data.pubkey.modulus);
        inputs.insert("pubkey_redc".to_string(), input_data.pubkey.redc);
        inputs.insert("signature".to_string(), input_data.signature);
        inputs.insert(
            "date_index".to_string(),
            vec![input_data.date_index.to_string()],
        );
        inputs.insert(
            "subject_index".to_string(),
            vec![input_data.subject_sequence.index.to_string()],
        );
        inputs.insert(
            "subject_length".to_string(),
            vec![input_data.subject_sequence.length.to_string()],
        );
        inputs.insert(
            "from_header_index".to_string(),
            vec![input_data.from_header_sequence.index.to_string()],
        );
        inputs.insert(
            "from_header_length".to_string(),
            vec![input_data.from_header_sequence.length.to_string()],
        );
        inputs.insert(
            "from_address_index".to_string(),
            vec![input_data.from_address_sequence.index.to_string()],
        );
        inputs.insert(
            "from_address_length".to_string(),
            vec![input_data.from_address_sequence.length.to_string()],
        );

        // Define the expected order of witness values based on the ZkEmailInput struct
        let witness_key_order = [
            "header_storage",
            "header_len",
            "pubkey_modulus",
            "pubkey_redc",
            "signature",
            "date_index",
            "subject_index",
            "subject_length",
            "from_header_index",
            "from_header_length",
            "from_address_index",
            "from_address_length",
        ];

        let mut witness_vec_string: Vec<String> = Vec::new();
        for key in witness_key_order {
            match inputs.get(key) {
                Some(values) => witness_vec_string.extend(values.iter().cloned()),
                None => panic!("Missing required input key in HashMap: {}", key),
            }
        }
        witness_vec_string
    }
}
