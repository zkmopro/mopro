fn main() {
    if std::env::var("CIRCOM_PROVER_TEST_WITNESS").unwrap_or_default() != "" {
        #[cfg(feature = "rustwitness")]
        rust_witness::transpile::transpile_wasm("./test-vectors".to_string());
        #[cfg(feature = "witnesscalc")]
        witnesscalc_adapter::build_and_link("./test-vectors");
    }

    println!("cargo:rerun-if-env-changed=CIRCOM_PROVER_TEST_WITNESS");
}
