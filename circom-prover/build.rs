fn main() {
    #[cfg(feature = "rustwitness")]
    rust_witness::transpile::transpile_wasm("./test-vectors".to_string());
    #[cfg(feature = "witnesscalc")]
    witnesscalc_adapter::build_and_link("./test-vectors");
}
