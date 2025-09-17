fn main() {
    if std::env::var("CIRCOM_PROVER_TEST_WITNESS").unwrap_or_default() != "" {
        #[cfg(all(test, feature = "rustwitness"))]
        rust_witness::transpile::transpile_wasm("./test-vectors".to_string());
        #[cfg(all(test, feature = "witnesscalc"))]
        witnesscalc_adapter::build_and_link("./test-vectors");
    }
}
