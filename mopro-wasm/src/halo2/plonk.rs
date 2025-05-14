use std::collections::HashMap;

use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;

use halo2_keccak_256;
use mopro_halo2_rsa;
use plonk_fibonacci;

#[wasm_bindgen]
pub fn generate_plonk_proof(
    srs_key: &[u8],
    proving_key: &[u8],
    input: JsValue,
) -> Result<JsValue, JsValue> {
    let input: HashMap<String, Vec<String>> = from_value(input)
        .map_err(|e| JsValue::from_str(&format!("Failed to parse input: {}", e)))?;

    // Generate proof
    let (proof, public_input) = plonk_fibonacci::prove(srs_key, proving_key, input)
        .map_err(|e| JsValue::from_str(&format!("Proof generation failed: {}", e)))?;

    // Serialize the output back into JsValue
    to_value(&(proof, public_input))
        .map_err(|e| JsValue::from_str(&format!("Serialization failed: {}", e)))
}

#[wasm_bindgen]
pub fn verify_plonk_proof(
    srs_key: &[u8],
    verifying_key: &[u8],
    proof: JsValue,
    public_inputs: JsValue,
) -> Result<JsValue, JsValue> {
    let proof: Vec<u8> = from_value(proof)
        .map_err(|e| JsValue::from_str(&format!("Failed to parse proof: {}", e)))?;
    let public_inputs: Vec<u8> = from_value(public_inputs)
        .map_err(|e| JsValue::from_str(&format!("Failed to parse public_inputs: {}", e)))?;

    // Verify proof
    let is_valid = plonk_fibonacci::verify(srs_key, verifying_key, proof, public_inputs)
        .map_err(|e| JsValue::from_str(&format!("Proof verification failed: {}", e)))?;

    // Convert result to JsValue
    to_value(&is_valid).map_err(|e| JsValue::from_str(&format!("Serialization failed: {}", e)))
}

#[wasm_bindgen]
pub fn generate_plonk_keccak256_proof(
    srs_key: &[u8],
    proving_key: &[u8],
    input: JsValue,
) -> Result<JsValue, JsValue> {
    let input: HashMap<String, Vec<String>> = from_value(input)
        .map_err(|e| JsValue::from_str(&format!("Failed to parse input: {}", e)))?;

    // Generate proof
    let (proof, public_input) = halo2_keccak_256::prove(srs_key, proving_key, input)
        .map_err(|e| JsValue::from_str(&format!("Proof generation failed: {}", e)))?;

    // Serialize the output back into JsValue
    to_value(&(proof, public_input))
        .map_err(|e| JsValue::from_str(&format!("Serialization failed: {}", e)))
}

#[wasm_bindgen]
pub fn verify_plonk_keccak256_proof(
    srs_key: &[u8],
    verifying_key: &[u8],
    proof: JsValue,
    public_inputs: JsValue,
) -> Result<JsValue, JsValue> {
    let proof: Vec<u8> = from_value(proof)
        .map_err(|e| JsValue::from_str(&format!("Failed to parse proof: {}", e)))?;
    let public_inputs: Vec<u8> = from_value(public_inputs)
        .map_err(|e| JsValue::from_str(&format!("Failed to parse public_inputs: {}", e)))?;

    // Verify proof
    let is_valid = halo2_keccak_256::verify(srs_key, verifying_key, proof, public_inputs)
        .map_err(|e| JsValue::from_str(&format!("Proof verification failed: {}", e)))?;

    // Convert result to JsValue
    to_value(&is_valid).map_err(|e| JsValue::from_str(&format!("Serialization failed: {}", e)))
}

#[wasm_bindgen]
pub fn generate_plonk_rsa_proof(
    srs_key: &[u8],
    proving_key: &[u8],
    input: JsValue,
) -> Result<JsValue, JsValue> {
    let input: HashMap<String, Vec<String>> = from_value(input)
        .map_err(|e| JsValue::from_str(&format!("Failed to parse input: {}", e)))?;

    // Generate proof
    // The `prove`` function here contains key-gen, and this version of the RSA circuit
    // doesn't have functions to read from and write to a file for proving/verifcation key.
    // So we hacked here to add `elapsed` to record real proving time
    let (proof, public_input, _elapsed) = mopro_halo2_rsa::prove(srs_key, proving_key, input)
        .map_err(|e| JsValue::from_str(&format!("Proof generation failed: {}", e)))?;

    // Serialize the output back into JsValue
    to_value(&(proof, public_input))
        .map_err(|e| JsValue::from_str(&format!("Serialization failed: {}", e)))
}

#[wasm_bindgen]
pub fn verify_plonk_rsa_proof(
    srs_key: &[u8],
    verifying_key: &[u8],
    proof: JsValue,
    public_inputs: JsValue,
) -> Result<JsValue, JsValue> {
    let proof: Vec<u8> = from_value(proof)
        .map_err(|e| JsValue::from_str(&format!("Failed to parse proof: {}", e)))?;
    let public_inputs: Vec<u8> = from_value(public_inputs)
        .map_err(|e| JsValue::from_str(&format!("Failed to parse public_inputs: {}", e)))?;

    // Verify proof
    let is_valid = mopro_halo2_rsa::verify(srs_key, verifying_key, proof, public_inputs)
        .map_err(|e| JsValue::from_str(&format!("Proof verification failed: {}", e)))?;

    // Convert result to JsValue
    to_value(&is_valid).map_err(|e| JsValue::from_str(&format!("Serialization failed: {}", e)))
}
