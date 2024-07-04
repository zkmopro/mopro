fn main() {
    rust_witness::transpile::transpile_wasm("../test-vectors/circom".to_string());
    // uniffi::uniffi_bindgen_main();
}
