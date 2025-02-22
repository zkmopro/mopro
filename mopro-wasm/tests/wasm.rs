#![cfg(test)]
#![cfg(target_family = "wasm")]

use std::collections::HashMap;

use wasm_bindgen::prelude::*;
pub use wasm_bindgen_rayon::init_thread_pool;
use wasm_bindgen_test::*;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
#[cfg(feature = "plonk")]
pub fn prove_and_verify_plonk_proof() {
    // Read all at once
    const SRS_KEY: &[u8] = include_bytes!("../../test-vectors/halo2/plonk_fibonacci_srs.bin");
    const PROVING_KEY: &[u8] = include_bytes!("../../test-vectors/halo2/plonk_fibonacci_pk.bin");
    const VERIFYING_KEY: &[u8] = include_bytes!("../../test-vectors/halo2/plonk_fibonacci_vk.bin");

    // Test input
    let mut input = HashMap::new();
    input.insert("out".to_string(), vec!["55".to_string()]);

    match plonk_fibonacci::prove(SRS_KEY, PROVING_KEY, input) {
        Ok((proof, public_input)) => {
            assert!(
                plonk_fibonacci::verify(SRS_KEY, VERIFYING_KEY, proof, public_input)
                    .expect("Proof verification should not fail")
            );
        }
        Err(e) => panic!("Generating proof failed: {:?}", e),
    }
}

#[wasm_bindgen_test]
#[cfg(feature = "hyperplonk")]
pub fn prove_and_verify_hyperplonk_proof() {
    // Read all at once
    const SRS_KEY: &[u8] = include_bytes!("../../test-vectors/halo2/hyperplonk_fibonacci_srs.bin");
    const PROVING_KEY: &[u8] = include_bytes!("../../test-vectors/halo2/hyperplonk_fibonacci_pk.bin");
    const VERIFYING_KEY: &[u8] = include_bytes!("../../test-vectors/halo2/hyperplonk_fibonacci_vk.bin");

    // Test input
    let mut input = HashMap::new();
    input.insert("out".to_string(), vec!["55".to_string()]);

    match hyperplonk_fibonacci::prove(SRS_KEY, PROVING_KEY, input) {
        Ok((proof, public_input)) => {
            assert!(
                hyperplonk_fibonacci::verify(SRS_KEY, VERIFYING_KEY, proof, public_input)
                    .expect("Proof verification should not fail")
            );
        }
        Err(e) => panic!("Generating proof failed: {:?}", e),
    }
}

#[wasm_bindgen_test]
#[cfg(feature = "gemini")]
pub fn prove_and_verify_gemini_proof() {
    // Read all at once
    const SRS_KEY: &[u8] = include_bytes!("../../test-vectors/halo2/gemini_fibonacci_srs.bin");
    const PROVING_KEY: &[u8] = include_bytes!("../../test-vectors/halo2/gemini_fibonacci_pk.bin");
    const VERIFYING_KEY: &[u8] = include_bytes!("../../test-vectors/halo2/gemini_fibonacci_vk.bin");

    // Test input
    let mut input = HashMap::new();
    input.insert("out".to_string(), vec!["55".to_string()]);

    match gemini_fibonacci::prove(SRS_KEY, PROVING_KEY, input) {
        Ok((proof, public_input)) => {
            assert!(
                gemini_fibonacci::verify(SRS_KEY, VERIFYING_KEY, proof, public_input)
                    .expect("Proof verification should not fail")
            );
        }
        Err(e) => panic!("Generating proof failed: {:?}", e),
    }
}
