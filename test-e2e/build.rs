fn main() {
    rust_witness::transpile::transpile_wasm("../test-vectors/circom".to_string());
    witnesscalc_adapter::build_and_link("../test-vectors/circom/witnesscalc");
}
