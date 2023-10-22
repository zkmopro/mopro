use mopro_core::middleware::circom;
use std::path::Path;

// TODO: More general name?
pub fn initialize_witness_calculator(path: &str) {
    println!(
        "cargo:warning=Initializing witness calculator with path: {}",
        path
    );
    circom::initialize(Path::new(path));
}

fn main() {
    uniffi::generate_scaffolding("src/mopro.udl").expect("Building the UDL file failed");

    // XXX: We probably want this to be read from environment variable
    // Also this should work from iOS too
    //    let dylib_path = "../mopro-core/target/debug/keccak256.dylib";
    let dylib_path = "../mopro-core/target/debug/keccak256.dylib";

    initialize_witness_calculator(dylib_path);
}
