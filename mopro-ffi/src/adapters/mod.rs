#[cfg(feature = "circom")]
pub mod circom;
#[cfg(feature = "halo2")]
pub mod halo2;
#[cfg(feature = "noir")]
pub mod noir;

#[macro_export]
macro_rules! setup_adapters {
    () => {
        // Shared Error message for `mopro_ffi`
        // This should be declared into this macro due to Uniffi's limitation
        // Please refer this issue: https://github.com/mozilla/uniffi-rs/issues/2257
        #[derive(Debug, thiserror::Error, uniffi::Error)]
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
