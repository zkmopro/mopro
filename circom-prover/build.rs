fn main() {
    let crate_name = std::env::var("CARGO_PKG_NAME").unwrap();
    if crate_name == "circom-prover" {
        #[cfg(feature = "rustwitness")]
        rust_witness::transpile::transpile_wasm("./test-vectors".to_string());
        #[cfg(feature = "witnesscalc")]
        witnesscalc_adapter::build_and_link("./test-vectors");
    }
}
