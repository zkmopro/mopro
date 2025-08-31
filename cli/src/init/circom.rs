use std::fs;
use std::path::Path;
use super::replace_string_in_file;
use super::ProvingSystem;
use anyhow::Result;
use include_dir::include_dir;
use include_dir::Dir;

pub struct Circom;

const TEMPLATE_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/src/template/circom");

impl ProvingSystem for Circom {
    fn dep_template(file_path: &str) -> Result<()> {
        let replacement = r#"
# CIRCOM_DEPENDENCIES
circom-prover = { git = "https://github.com/zkmopro/mopro.git" }
rust-witness  = "0.1"
num-bigint    = "0.4.0"
    "#;

        let target = "# CIRCOM_DEPENDENCIES";
        replace_string_in_file(file_path, target, replacement)
    }

    fn build_dep_template(file_path: &str) -> Result<()> {
        let replacement = r#"
# CIRCOM_BUILD_DEPENDENCIES
rust-witness = "0.1"
    "#;
        let target = "# CIRCOM_BUILD_DEPENDENCIES";
        replace_string_in_file(file_path, target, replacement)
    }

    fn lib_template(file_path: &str) -> Result<()> {
        let circom_lib_rs = match TEMPLATE_DIR.get_file("lib.rs") {
            Some(file) => file.contents(),
            None => return Err(anyhow::anyhow!("lib.rs not found in template")),
        };
        let target = "// CIRCOM_TEMPLATE";
        replace_string_in_file(file_path, target, &String::from_utf8_lossy(circom_lib_rs))
    }

    fn mod_template(lib_file_path: &str) -> Result<()> {
        let mod_file = "circom.rs";
        let circom_rs = match TEMPLATE_DIR.get_file(mod_file) {
            Some(file) => file.contents(),
            None => return Err(anyhow::anyhow!("circom.rs not found in template")),
        };

        // Place the circom.rs in the same directory as lib.rs
        let dest_path = Path::new(lib_file_path)
            .parent()
            .ok_or_else(|| anyhow::anyhow!("Invalid file_path: no parent directory"))?
            .join(mod_file);

        fs::write(&dest_path, circom_rs).map_err(|e| anyhow::anyhow!("{}", e))
    }

    fn build_template(file_path: &str) -> Result<()> {
        let replacement =
            "rust_witness::transpile::transpile_wasm(\"./test-vectors/circom\".to_string());";
        let target = "// CIRCOM_TEMPLATE";
        replace_string_in_file(file_path, target, replacement)
    }
}
