fn main() {
    let target = std::env::var("TARGET").unwrap_or_default();

    if !target.contains("wasm32") {
        rust_witness::transpile::transpile_wasm("../test-vectors/circom".to_string());
        witnesscalc_adapter::build_and_link("../test-vectors/circom/witnesscalc");
    }
}
