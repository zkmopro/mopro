use super::replace_string_in_file;
use super::ProvingSystem;
use anyhow::Result;
use include_dir::include_dir;
use include_dir::Dir;
pub struct Noir;

impl ProvingSystem for Noir {
    fn lib_template(file_path: &str) -> Result<()> {
        let template_dir: Dir = include_dir!("$CARGO_MANIFEST_DIR/src/template/noir");
        let noir_lib_rs = match template_dir.get_file("lib.rs") {
            Some(file) => file.contents(),
            None => return Err(anyhow::anyhow!("lib.rs not found in template")),
        };
        let target = "// NOIR_TEMPLATE";
        replace_string_in_file(file_path, target, &String::from_utf8_lossy(noir_lib_rs))
    }
}
