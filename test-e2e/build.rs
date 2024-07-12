fn main() {
    compile_circom_circuits();

    std::fs::write("./src/mopro.udl", mopro_ffi::app_config::UDL).expect("Failed to write UDL");
    uniffi::generate_scaffolding("./src/mopro.udl").unwrap();
}

fn compile_circom_circuits() {
    rust_witness::transpile::transpile_wasm("../test-vectors/circom".to_string());
}
