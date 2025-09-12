use super::replace_string_in_file;
use super::ProvingSystem;
use anyhow::Result;
use include_dir::include_dir;
use include_dir::Dir;
use std::fs;
use std::path::Path;
pub struct Noir;

impl ProvingSystem for Noir {
    const TEMPLATE_DIR: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/src/template/noir");

    fn dep_template(file_path: &str) -> Result<()> {
        let replacement = r#"
# NOIR_DEPENDENCIES
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0.94"

noir_rs = { package = "noir", git = "https://github.com/zkmopro/noir-rs", features = [
    "barretenberg",
    "android-compat",
], branch = "v1.0.0-beta.3-2" }
"#;

        let target = "# NOIR_DEPENDENCIES";
        replace_string_in_file(file_path, target, replacement)
    }

    fn dev_dep_template(file_path: &str) -> Result<()> {
        let replacement = r#"
# NOIR_DEV_DEPENDENCIES
serial_test = "3.0.0"
"#;
        let target = "# NOIR_DEV_DEPENDENCIES";
        replace_string_in_file(file_path, target, replacement)
    }

    fn lib_template(file_path: &str) -> Result<()> {
        let noir_lib_rs = match Self::TEMPLATE_DIR.get_file("lib.rs") {
            Some(file) => file.contents(),
            None => return Err(anyhow::anyhow!("lib.rs not found in template")),
        };
        let target = "// NOIR_TEMPLATE";
        replace_string_in_file(file_path, target, &String::from_utf8_lossy(noir_lib_rs))
    }

    fn test_template(_lib_file_path: &str) -> Result<()> {
        Ok(())
    }

    fn mod_template(lib_file_path: &str) -> Result<()> {
        let mod_file = "noir.rs";
        let circom_rs = match Self::TEMPLATE_DIR.get_file(mod_file) {
            Some(file) => file.contents(),
            None => return Err(anyhow::anyhow!("noir.rs not found in template")),
        };

        // Place the circom.rs in the same directory as lib.rs
        let dest_path = Path::new(lib_file_path)
            .parent()
            .ok_or_else(|| anyhow::anyhow!("Invalid file_path: no parent directory"))?
            .join(mod_file);

        fs::write(&dest_path, circom_rs).map_err(|e| anyhow::anyhow!("{}", e))
    }
}
