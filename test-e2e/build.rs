fn main() {
    println!("cargo::rustc-check-cfg=cfg(disable_uniffi_export)");
    
    rust_witness::transpile::transpile_wasm("../test-vectors/circom".to_string());
}
