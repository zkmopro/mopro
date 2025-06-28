//! UniFFI re-export
//!
//! Uniffi macros use fully qualified paths (`::wasm-bindgen::*`) internally.
//! To allow downstream crates to transparently resolve these macros to `mopro_ffi`,
//! users must alias it (`extern crate mopro_ffi as wasm_bindgen;`, automated via `app!` macro).
//!
//! However, for this alias to work correctly, `mopro_ffi` must provide the exact same
//! exported items as the original `wasm-bindgen`. Hence, we re-export all individual items.
#[cfg(feature = "wasm-bindgen-reexport")]
#[allow(unused_imports)]
pub use ::wasm_bindgen::*;

#[cfg(feature = "wasm-bindgen-reexport")]
#[allow(unused_imports)]
pub use ::serde_wasm_bindgen::*;

// We configure this out for `mopro-ffi` tests as we can not do `extern crate` then
#[cfg(all(feature = "wasm-bindgen-reexport", not(test)))]
#[macro_export]
macro_rules! wasm_bindgen_setup {
    () => {
        // `::uniffi` must be available in the caller’s extern-prelude.
        extern crate mopro_ffi as wasm_bindgen;
        extern crate mopro_ffi as serde_wasm_bindgen;
    };
}

#[cfg(not(all(feature = "wasm-bindgen-reexport", not(test))))]
#[macro_export]
macro_rules! wasm_bindgen_setup {
    () => {
    };
}