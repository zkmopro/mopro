fn main() {
    if cfg!(test) {
        rust_witness::transpile::transpile_wasm("../test-vectors/circom".to_string());
    }
}
