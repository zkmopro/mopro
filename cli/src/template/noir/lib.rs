#[cfg(test)]
mod noir_tests {
    // Import the generated functions from the uniffi bindings

    use super::*;

    #[test]
    fn test_noir_multiplier2() {
        let srs_path = "./test-vectors/noir/noir_multiplier2.srs".to_string();
        let circuit_path = "./test-vectors/noir/noir_multiplier2.json".to_string();
        let circuit_inputs = vec!["3".to_string(), "5".to_string()];
        let vk = get_noir_verification_keccak_key(
            circuit_path.clone(),
            Some(srs_path.clone()),
            false,  // disable_zk
            false,  // low_memory_mode
        ).unwrap();
        let result = generate_noir_proof_with_keccak(
            circuit_path.clone(),
            Some(srs_path.clone()),
            circuit_inputs.clone(),
            false,  // disable_zk
            vk.clone(),
            false,  // low_memory_mode
        );
        assert!(result.is_ok());
        let proof = result.unwrap();
        let result = verify_noir_proof_with_keccak(
            circuit_path.clone(),
            proof,
            false,  // disable_zk
            vk,
            false,  // low_memory_mode
        );
        assert!(result.is_ok());
        let valid = result.unwrap();
        assert!(valid);
    }

    #[test]
    fn test_noir_multiplier2_with_existing_vk() {
        let srs_path = "./test-vectors/noir/noir_multiplier2.srs".to_string();
        let circuit_path = "./test-vectors/noir/noir_multiplier2.json".to_string();
        let vk_path = "./test-vectors/noir/noir_multiplier2.vk".to_string();

        // read vk from file as Vec<u8>
        let vk: Vec<u8> = std::fs::read(vk_path.clone()).unwrap();

        let circuit_inputs = vec!["3".to_string(), "5".to_string()];
        let result = generate_noir_proof_with_keccak(
            circuit_path.clone(),
            Some(srs_path.clone()),
            circuit_inputs.clone(),
            false,  // disable_zk
            vk.clone(),
            false,  // low_memory_mode
        );
        assert!(result.is_ok());
        let proof = result.unwrap();
        let result = verify_noir_proof_with_keccak(
            circuit_path.clone(),
            proof,
            false,  // disable_zk
            vk.clone(),
            false,  // low_memory_mode
        );
        assert!(result.is_ok());
        let valid = result.unwrap();
        assert!(valid);
    }
}
