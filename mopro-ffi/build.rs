fn main() {
    #[cfg(feature = "circom")]
    {
        #[cfg(feature = "circom-wit-rustwitness")]
        if std::env::var("MOPRO_FFI_LINK_TEST_WITNESS").unwrap_or_default() != "" {
            rust_witness::transpile::transpile_wasm("../test-vectors/circom".to_string());
        }

        #[cfg(feature = "circom-wit-witnesscalc")]
        {
            witnesscalc_adapter::build_and_link("../test-vectors/circom/witnesscalc");
        }
    }
}
