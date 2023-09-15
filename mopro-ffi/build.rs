fn main() {
    uniffi::generate_scaffolding("./src/mopro_uniffi.udl").expect("Building the UDL file failed");
}
