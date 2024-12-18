use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn hello() -> Result<JsValue, JsValue> {
    Ok(JsValue::from_str("Hello, world!"))
}
// #[cfg(target_family = "wasm")]
use mopro_wasm::halo2::gemini;
use mopro_wasm::halo2::hyperplonk;
use mopro_wasm::halo2::plonk;

