#[cfg(feature = "uniffi")]
mod uniffi;
#[cfg(feature = "wasm-bindgen")]
mod wasm_bindgen;

#[cfg(all(not(feature = "uniffi"), any(target_os = "ios", target_os = "android")))]
compile_error!(
    "To compile mopro_ffi for iOS and Android targets you must enable the `uniffi` feature!"
);

#[cfg(all(not(feature = "wasm-bindgen"), target_arch = "wasm32"))]
compile_error!(
    "To compile mopro_ffi for iOS and Android targets you must enable the `uniffi` feature!"
);

#[cfg(feature = "uniffi-reexport")]
#[allow(unused_imports)]
pub use crate::bindings::uniffi::*;

#[cfg(feature = "wasm-bindgen-reexport")]
#[allow(unused_imports)]
pub use crate::bindings::wasm_bindgen::*;

#[macro_export]
macro_rules! setup_bindings {
    () => {
        $crate::uniffi_setup!();
        $crate::wasm_bindgen_setup!();
    };
}

// Placeholder macros for uniffi and other binding setups
// We need these placeholders for use in `setup!`
// when the respective features are not enabled.
#[cfg(not(feature = "uniffi"))]
#[macro_export]
macro_rules! uniffi_setup {
    () => {};
}

#[cfg(not(feature = "wasm-bindgen"))]
#[macro_export]
macro_rules! wasm_bindgen_setup {
    () => {};
}
