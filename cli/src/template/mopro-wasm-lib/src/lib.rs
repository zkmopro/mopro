#[allow(unused_imports)]
#[cfg(target_family = "wasm")]
use mopro_wasm::halo2::{gemini, hyperplonk, plonk};

use wasm_bindgen::prelude::wasm_bindgen;

// Customize your code here
// Reference: https://rustwasm.github.io/docs/wasm-bindgen/reference/types.html
//
#[wasm_bindgen(js_name = "moproWasmHelloWorld")]
pub fn mopro_wasm_hello_world() -> String {
    "Hello, World!".to_string()
}
