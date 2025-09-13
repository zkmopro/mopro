use noir_rs::{
    barretenberg::{
        prove::prove_ultra_honk, srs::setup_srs_from_bytecode, utils::get_honk_verification_key,
        verify::verify_ultra_honk,
    },
    witness::from_vec_str_to_witness_map,
};

use crate::MoproError;

#[uniffi::export]
pub(crate) fn generate_noir_proof(
    circuit_path: String,
    srs_path: Option<String>,
    inputs: Vec<String>,
) -> Result<Vec<u8>, MoproError> {
    let circuit_bytecode = get_bytecode(circuit_path);

    // Setup the SRS
    setup_srs_from_bytecode(circuit_bytecode.as_str(), srs_path.as_deref(), false).unwrap();

    // Set up the witness
    let witness = from_vec_str_to_witness_map(inputs.iter().map(|s| s.as_str()).collect()).unwrap();

    prove_ultra_honk(circuit_bytecode.as_str(), witness, false)
        .map_err(|e| MoproError::NoirError(format!("Generate Proof error: {}", e)))
}

#[uniffi::export]
pub(crate) fn verify_noir_proof(circuit_path: String, proof: Vec<u8>) -> Result<bool, MoproError> {
    let circuit_bytecode = get_bytecode(circuit_path);
    let vk = get_honk_verification_key(circuit_bytecode.as_str(), false).unwrap();
    verify_ultra_honk(proof, vk).map_err(|e| MoproError::NoirError(format!("Verify Error: {}", e)))
}

fn get_bytecode(circuit_path: String) -> String {
    // Read the JSON manifest of the circuit
    let circuit_txt = std::fs::read_to_string(circuit_path).unwrap();
    let circuit: serde_json::Value = serde_json::from_str(&circuit_txt).unwrap();

    circuit["bytecode"].as_str().unwrap().to_string()
}

#[cfg(test)]
mod tests {
    const MULTIPLIER2_CIRCUIT_FILE: &str = "./test-vectors/noir/noir_multiplier2.json";

    #[test]
    fn test_proof_multiplier2() {
        let witness = vec!["3".to_string(), "5".to_string()];
        let proof = super::generate_noir_proof(MULTIPLIER2_CIRCUIT_FILE.to_string(), None, witness)
            .unwrap();
        assert!(super::verify_noir_proof(MULTIPLIER2_CIRCUIT_FILE.to_string(), proof).is_ok());
    }
}
