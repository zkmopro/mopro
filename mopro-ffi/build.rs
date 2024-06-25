use rust_witness::transpile::transpile_wasm;

fn main() {
    transpile_wasm("../mopro-core/examples/circom".to_string());
    uniffi::generate_scaffolding("src/mopro.udl").expect("Building the UDL file failed");
}
