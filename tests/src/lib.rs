#[cfg(feature = "uniffi")]
mopro_ffi::app!();

fn mopro_hello_world() -> String {
    "Hello, World!".to_string()
}

#[cfg(feature = "uniffi")]
#[uniffi::export]
fn mopro_uniffi_hello_world() -> String {
    mopro_hello_world()
}

#[cfg(feature = "flutter")]
pub fn mopro_flutter_hello_world() -> String {
    mopro_hello_world()
}
