use std::fs;
use std::path::Path;
use super::replace_string_in_file;
use super::ProvingSystem;
use anyhow::Result;
use include_dir::include_dir;
use include_dir::Dir;

pub struct Circom;

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
        let template_dir: Dir = include_dir!("$CARGO_MANIFEST_DIR/src/template/circom");
        let circom_lib_rs = match template_dir.get_file("lib.rs") {
            Some(file) => file.contents(),
            None => return Err(anyhow::anyhow!("lib.rs not found in template")),
        };
        let target = "// CIRCOM_TEMPLATE";
        replace_string_in_file(file_path, target, &String::from_utf8_lossy(circom_lib_rs))?;

        // Copy `circom.rs` from the template dir next to the file_path file
        let circom_rs = match template_dir.get_file("circom.rs") {
            Some(file) => file.contents(),
            None => return Err(anyhow::anyhow!("circom.rs not found in template")),
        };

        let dest_path = Path::new(file_path)
            .parent()
            .ok_or_else(|| anyhow::anyhow!("Invalid file_path: no parent directory"))?
            .join("circom.rs");

        fs::write(&dest_path, circom_rs).map_err(|e| anyhow::anyhow!("{}", e))
    }

    fn build_template(file_path: &str) -> Result<()> {
        let replacement =
            "rust_witness::transpile::transpile_wasm(\"./test-vectors/circom\".to_string());";
        let target = "// CIRCOM_TEMPLATE";
        replace_string_in_file(file_path, target, replacement)
    }
}
