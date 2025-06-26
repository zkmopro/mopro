#[cfg(feature = "circom")]
pub mod circom;
#[cfg(feature = "halo2")]
pub mod halo2;
#[cfg(feature = "noir")]
pub mod noir;

#[macro_export]
macro_rules! setup_adapters_common {
    () => {
        // Shared Error message for `mopro_ffi`
        // This should be declared into this macro due to Uniffi's limitation
        // Please refer this issue: https://github.com/mozilla/uniffi-rs/issues/2257
        #[derive(Debug, thiserror::Error)]
        #[allow(unused)]
        #[cfg_attr(not(feature = "no_uniffi_exports"), derive(uniffi::Error))]
        pub enum MoproError {
            #[error("CircomError: {0}")]
            CircomError(String),
            #[error("Halo2Error: {0}")]
            Halo2Error(String),
            #[error("NoirError: {0}")]
            NoirError(String),
        }
    };
}

// Placeholder macros for circom, halo2, and noir setups
// We need these placeholders for use in `app!`
// when the respective features are not enabled.
// Once we move to a per-adapter setup, we can remove these
#[cfg(not(feature = "circom"))]
#[macro_export]
macro_rules! circom_setup {
    () => {};
}

#[cfg(not(feature = "halo2"))]
#[macro_export]
macro_rules! halo2_setup {
    () => {};
}

#[cfg(not(feature = "noir"))]
#[macro_export]
macro_rules! noir_setup {
    () => {};
}