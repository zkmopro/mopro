fn main() {
    uniffi::generate_scaffolding("./src/rust_uniffi_example.udl")
        .expect("Building the UDL file failed");
}
