fn main() {
    if std::env::var("MOPRO_FFI_LINK_TEST_WITNESS").unwrap_or_default() != "" {
        rust_witness::transpile::transpile_wasm("../test-vectors/circom".to_string());
    }
}
