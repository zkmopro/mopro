use rust_witness::transpile::transpile_wasm;

#[cfg(test)]
fn main() {
    transpile_wasm("./test-vectors/circom".to_string());
    uniffi::generate_scaffolding("src/mopro.udl").expect("Building the UDL file failed");
}

#[cfg(not(test))]
fn main() {
    uniffi::generate_scaffolding("src/mopro.udl").expect("Building the UDL file failed");
}
