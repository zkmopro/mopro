use std::collections::HashMap;

use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;

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
