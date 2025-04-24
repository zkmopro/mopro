use super::replace_string_in_file;
use super::ProvingSystem;
use anyhow::Result;
use include_dir::include_dir;
use include_dir::Dir;

pub struct Noir;

impl ProvingSystem for Noir {
    fn lib_template(file_path: &str) -> Result<()> {
        Ok(())
    }

    fn dep_template(file_path: &str) -> Result<()> {
        Ok(())
    }

    fn build_template(file_path: &str) -> Result<()> {
        Ok(())
    }
}
