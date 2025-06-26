mod uniffi;

#[allow(unused_imports)]
pub use crate::bindings::uniffi::*;

#[macro_export]
macro_rules! setup_bindings {
    () => {
        mopro_ffi::uniffi_setup!();
    };
}