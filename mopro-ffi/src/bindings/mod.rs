#[cfg(feature = "uniffi")]
mod uniffi;

#[cfg(all(not(feature = "uniffi"), any(target_os = "ios", target_os = "android")))]
compile_error!("To compile mopro_ffi for iOS and Android targets you must enable the `uniffi` feature!");

#[cfg(feature = "uniffi-reexport")]
#[allow(unused_imports)]
pub use crate::bindings::uniffi::*;

#[macro_export]
macro_rules! setup_bindings {
    () => {
        $crate::uniffi_setup!();
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
