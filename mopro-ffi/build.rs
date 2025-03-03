fn main() {
    #[cfg(feature = "circom")]
    if std::env::var("MOPRO_FFI_LINK_TEST_WITNESS").unwrap_or_default() != "" {
        rust_witness::transpile::transpile_wasm("../test-vectors/circom".to_string());
    }

    if std::env::var("DISABLE_UNIFFI_EXPORT").is_ok() {
        println!("cargo:rustc-cfg=disable_uniffi_export");
    }
}
