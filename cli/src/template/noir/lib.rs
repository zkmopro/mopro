#[cfg(test)]
mod noir_tests {
    use super::*;

    #[test]
    fn test_noir_multiplier2() {
        let srs_path = "./test-vectors/noir/noir_multiplier2.srs".to_string();
        let circuit_path = "./test-vectors/noir/noir_multiplier2.json".to_string();
        let circuit_inputs = vec!["3".to_string(), "5".to_string()];
        let result = generate_noir_proof(
            circuit_path.clone(),
            Some(srs_path.clone()),
            circuit_inputs.clone(),
            false  // low_memory_mode
        );
        assert!(result.is_ok());
        let proof = result.unwrap();
        let result = verify_noir_proof(
            circuit_path.clone(),
            proof,
            false  // disable_zk
        );
        assert!(result.is_ok());
        let valid = result.unwrap();
        assert!(valid);
    }
}
