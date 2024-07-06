fn main() {
    let link_test_witness = std::env::var("MOPRO_FFI_LINK_TEST_WITNESS").unwrap_or("".to_string());
    if link_test_witness != "" {
        rust_witness::transpile::transpile_wasm("../test-vectors/circom".to_string());
    }
}
