//! UniFFI re-export
//!
//! Uniffi macros use fully qualified paths (`::uniffi::*`) internally.
//! To allow downstream crates to transparently resolve these macros to `mopro_ffi`,
//! users must alias it (`extern crate mopro_ffi as uniffi;`, automated via `app!` macro).
//!
//! However, for this alias to work correctly, `mopro_ffi` must provide the exact same
//! exported items as the original `uniffi`. Hence, we re-export all individual items.
#[cfg(feature = "uniffi")]
#[allow(unused_imports)]
pub use ::uniffi::*;

#[cfg(feature = "uniffi")]
#[macro_export]
macro_rules! uniffi_setup {
    () => {
        // `::uniffi` must be available in the caller’s extern-prelude.
        extern crate mopro_ffi as uniffi;

        uniffi::setup_scaffolding!("mopro");
    };
}

#[cfg(not(feature = "uniffi"))]
#[macro_export]
macro_rules! uniffi_setup {
    () => {
        uniffi::setup_scaffolding!("mopro");
    };
}
