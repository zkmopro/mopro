use super::replace_string_in_file;
use super::ProvingSystem;
use anyhow::Ok;
use anyhow::Result;
use include_dir::include_dir;
use include_dir::Dir;

pub struct Halo2;

impl ProvingSystem for Halo2 {
    fn lib_template(file_path: &str) -> Result<()> {
        let template_dir: Dir = include_dir!("$CARGO_MANIFEST_DIR/src/template/halo2");
        let halo2_lib_rs = match template_dir.get_file("lib.rs") {
            Some(file) => file.contents(),
            None => return Err(anyhow::anyhow!("lib.rs not found in template")),
        };
        let target = "// HALO2_TEMPLATE";
        replace_string_in_file(file_path, target, &String::from_utf8_lossy(halo2_lib_rs))
    }

    fn dep_template(file_path: &str) -> Result<()> {
        let replacement =
	         "plonk-fibonacci = { package = \"plonk-fibonacci\", git = \"https://github.com/sifnoc/plonkish-fibonacci-sample.git\" } 
	 hyperplonk-fibonacci = { package = \"hyperplonk-fibonacci\", git = \"https://github.com/sifnoc/plonkish-fibonacci-sample.git\" } 
	 gemini-fibonacci = { package = \"gemini-fibonacci\", git = \"https://github.com/sifnoc/plonkish-fibonacci-sample.git\" }";
        let target = "# HALO2_DEPENDENCIES";
        replace_string_in_file(file_path, target, replacement)
    }

    fn build_template(_: &str) -> Result<()> {
        Ok(())
    }
}
