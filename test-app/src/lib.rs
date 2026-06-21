#[cfg(any(feature = "uniffi", feature = "wasm"))]
mopro_ffi::app!();

#[cfg(all(feature = "wasm", target_arch = "wasm32"))]
use mopro_ffi::prelude::wasm_bindgen;

fn mopro_hello_world() -> String {
    "Hello, World!".to_string()
}

#[cfg_attr(
    all(feature = "wasm", target_arch = "wasm32"),
    wasm_bindgen(js_name = "moproWasmHelloWorld")
)]
pub fn mopro_wasm_hello_world() -> String {
    mopro_hello_world()
}
