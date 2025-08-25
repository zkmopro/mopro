fn main() {
    #[cfg(feature = "circom")]
    rust_witness::transpile::transpile_wasm("../test-vectors/circom".to_string());
    #[cfg(feature = "circom")]
    witnesscalc_adapter::build_and_link("../test-vectors/circom/witnesscalc");
}
