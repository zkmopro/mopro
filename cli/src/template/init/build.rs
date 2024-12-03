use std::path::Path;

fn main() {
    // CIRCOM_TEMPLATE
    // This is writing the UDL file which defines the functions exposed
    // to your app. We have pre-generated this file for you.
    // Feel free to modify it to suit your needs.
    let udl_path = Path::new("src/mopro.udl");
    if !udl_path.exists() {
        std::fs::write(udl_path, mopro_ffi::app_config::UDL).expect("Failed to write UDL");
    }
    // Finally initialize uniffi and build the scaffolding into the
    // rust binary
    uniffi::generate_scaffolding(udl_path.to_str().unwrap()).unwrap();
}
