fn main() {
    #[cfg(feature = "circom")]
    {
        if std::env::var("MOPRO_FFI_LINK_TEST_WITNESS").unwrap_or_default() != "" {
            #[cfg(feature = "rustwitness")]
            rust_witness::transpile::transpile_wasm("../test-vectors/circom".to_string());

            #[cfg(feature = "witnesscalc")]
            witnesscalc_adapter::build_and_link("../test-vectors/circom/witnesscalc");
        }
    }
}
