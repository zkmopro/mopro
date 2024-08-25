fn main() {
    // We're going to transpile the wasm witness generators to C
    // Change this to where you put your zkeys and wasm files
    rust_witness::transpile::transpile_wasm("./test-vectors/circom".to_string());
    // This is writing the UDL file which defines the functions exposed
    // to your app. We have pre-generated this file for you
    // This file must be written to ./src
    std::fs::write("./src/mopro.udl", mopro_ffi::app_config::UDL).expect("Failed to write UDL");
    // Finally initialize uniffi and build the scaffolding into the
    // rust binary
    uniffi::generate_scaffolding("./src/mopro.udl").unwrap();
}
