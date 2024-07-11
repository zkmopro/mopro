fn main() {
    #[cfg(feature = "circom")]
    rust_witness::transpile::transpile_wasm("../test-vectors/circom".to_string());

    std::fs::write("./src/mopro.udl", mopro_ffi::app_config::UDL).expect("Failed to write UDL");
    uniffi::generate_scaffolding("./src/mopro.udl").unwrap();
}
