// --- Noir Example of using Ultra Honk proving and verifying circuits ---

// Module containing the Noir circuit logic (Multiplier2)
#[macro_use]
mod noir;

#[cfg(test)]
mod noir_tests {
    // Import the generated functions from the uniffi bindings
    use serial_test::serial;
    use super::*;

    #[test]
    #[serial]
    fn test_noir_multiplier2() {
        let srs_path = "./test-vectors/noir/noir_multiplier2.srs".to_string();
        let circuit_path = "./test-vectors/noir/noir_multiplier2.json".to_string();
        let circuit_inputs = vec!["3".to_string(), "5".to_string()];
        let vk = get_noir_verification_key(
            circuit_path.clone(),
            Some(srs_path.clone()),
            true,   // on_chain (uses Keccak for Solidity compatibility)
            false,  // low_memory_mode
        ).unwrap();

        let proof = generate_noir_proof(
            circuit_path.clone(),
            Some(srs_path.clone()),
            circuit_inputs.clone(),
            true,   // on_chain (uses Keccak for Solidity compatibility)
            vk.clone(),
            false,  // low_memory_mode
        ).unwrap();

        let valid = verify_noir_proof(
            circuit_path,
            proof,
            true,   // on_chain (uses Keccak for Solidity compatibility)
            vk,
            false,  // low_memory_mode
        ).unwrap();
        assert!(valid);
    }

    #[test]
    #[serial]
    fn test_noir_multiplier2_with_existing_vk() {
        let srs_path = "./test-vectors/noir/noir_multiplier2.srs".to_string();
        let circuit_path = "./test-vectors/noir/noir_multiplier2.json".to_string();
        let vk_path = "./test-vectors/noir/noir_multiplier2.vk".to_string();

        // read vk from file as Vec<u8>
        let vk = std::fs::read(vk_path).unwrap();

        let circuit_inputs = vec!["3".to_string(), "5".to_string()];

        let proof = generate_noir_proof(
            circuit_path.clone(),
            Some(srs_path),
            circuit_inputs,
            true,   // on_chain (uses Keccak for Solidity compatibility)
            vk.clone(),
            false,  // low_memory_mode
        ).unwrap();

        let valid = verify_noir_proof(
            circuit_path,
            proof,
            true,   // on_chain (uses Keccak for Solidity compatibility)
            vk,
            false,  // low_memory_mode
        ).unwrap();
        assert!(valid);
    }
}
