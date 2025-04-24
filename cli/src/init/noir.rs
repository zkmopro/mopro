use super::ProvingSystem;
use anyhow::Result;
pub struct Noir;

impl ProvingSystem for Noir {
    fn lib_template(_file_path: &str) -> Result<()> {
        Ok(())
    }

    fn dep_template(_file_path: &str) -> Result<()> {
        Ok(())
    }

    fn build_template(_file_path: &str) -> Result<()> {
        Ok(())
    }
}
