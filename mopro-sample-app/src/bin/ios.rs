use mopro_ffi::app_config::build;
use mopro_ffi::app_config::Target::Ios;

fn main() {
    // The name of the this crates "cdylib"
    // TODO - get this from the Cargo.toml
    let library_name = "mopro_app";

    build(Ios, library_name).unwrap();
}
