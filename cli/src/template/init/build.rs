fn main() {
    // CIRCOM_TEMPLATE
    // This is writing the UDL file which defines the functions exposed
    // to your app. We have pre-generated this file for you
    // This file must be written to ./src
    std::fs::write("./src/mopro.udl", mopro_ffi::app_config::UDL).expect("Failed to write UDL");
    // Finally initialize uniffi and build the scaffolding into the
    // rust binary
    uniffi::generate_scaffolding("./src/mopro.udl").unwrap();
}
