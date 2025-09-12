use super::replace_string_in_file;
use super::ProvingSystem;
use anyhow::Result;
use include_dir::include_dir;
use include_dir::Dir;
use std::fs;
use std::path::Path;

pub struct Halo2;

impl ProvingSystem for Halo2 {
    const TEMPLATE_DIR: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/src/template/halo2");

    fn dep_template(file_path: &str) -> Result<()> {
        let replacement = r#"
# HALO2_DEPENDENCIES
plonk-fibonacci   = { package = "plonk-fibonacci",   git = "https://github.com/sifnoc/plonkish-fibonacci-sample.git" }
hyperplonk-fibonacci = { package = "hyperplonk-fibonacci", git = "https://github.com/sifnoc/plonkish-fibonacci-sample.git" }
gemini-fibonacci  = { package = "gemini-fibonacci",  git = "https://github.com/sifnoc/plonkish-fibonacci-sample.git" }
"#;

        let target = "# HALO2_DEPENDENCIES";
        replace_string_in_file(file_path, target, replacement)
    }

    fn lib_template(file_path: &str) -> Result<()> {
        let halo2_lib_rs = match Self::TEMPLATE_DIR.get_file("lib.rs") {
            Some(file) => file.contents(),
            None => return Err(anyhow::anyhow!("lib.rs not found in template")),
        };
        let target = "// HALO2_TEMPLATE";
        replace_string_in_file(file_path, target, &String::from_utf8_lossy(halo2_lib_rs))
    }

    fn mod_template(lib_file_path: &str) -> Result<()> {
        let mod_file = "halo2.rs";
        let circom_rs = match Self::TEMPLATE_DIR.get_file(mod_file) {
            Some(file) => file.contents(),
            None => return Err(anyhow::anyhow!("halo2.rs not found in template")),
        };

        // Place the circom.rs in the same directory as lib.rs
        let dest_path = Path::new(lib_file_path)
            .parent()
            .ok_or_else(|| anyhow::anyhow!("Invalid file_path: no parent directory"))?
            .join(mod_file);

        fs::write(&dest_path, circom_rs).map_err(|e| anyhow::anyhow!("{}", e))
    }
}
