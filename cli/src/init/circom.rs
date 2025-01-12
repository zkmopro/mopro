use super::replace_string_in_file;
use super::ProvingSystem;
use anyhow::Result;
use include_dir::include_dir;
use include_dir::Dir;

pub struct Circom;

impl ProvingSystem for Circom {
    fn lib_template(file_path: &str) -> Result<()> {
        let template_dir: Dir = include_dir!("$CARGO_MANIFEST_DIR/src/template/circom");
        let circom_lib_rs = match template_dir.get_file("lib.rs") {
            Some(file) => file.contents(),
            None => return Err(anyhow::anyhow!("lib.rs not found in template")),
        };
        let target = "// CIRCOM_TEMPLATE";
        replace_string_in_file(file_path, target, &String::from_utf8_lossy(circom_lib_rs))
    }

    fn dep_template(file_path: &str) -> Result<()> {
        let replacement =
        "# TODO: fix this
[patch.crates-io]
ark-circom = { git = \"https://github.com/zkmopro/circom-compat.git\", version = \"0.1.0\", branch = \"wasm-delete\" }";
        let target = "# CIRCOM_DEPENDENCIES";
        replace_string_in_file(file_path, target, replacement)
    }

    fn build_template(file_path: &str) -> Result<()> {
        let replacement =
            "rust_witness::transpile::transpile_wasm(\"./test-vectors/circom\".to_string());";
        let target = "// CIRCOM_TEMPLATE";
        replace_string_in_file(file_path, target, replacement)
    }
}
