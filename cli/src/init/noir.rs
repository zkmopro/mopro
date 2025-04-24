use super::replace_string_in_file;
use super::ProvingSystem;
use anyhow::Result;
pub struct Noir;

impl ProvingSystem for Noir {
    fn lib_template(_file_path: &str) -> Result<()> {
        Ok(())
    }

    fn dep_template(file_path: &str) -> Result<()> {
        let replacement = "
# build for Android
noir_rs = { package = \"noir\", git = \"https://github.com/zkmopro/noir-rs\", features = [\"barretenberg\", \"android-compat\"], optional = true }
# build for iOS
# noir_rs = { package = \"noir\", git = \"https://github.com/zkmopro/noir-rs\", features = [\"barretenberg\"], optional = true }
";
        let target = "# NOIR_DEPENDENCIES";
        replace_string_in_file(file_path, target, replacement)
    }

    fn build_template(_file_path: &str) -> Result<()> {
        Ok(())
    }
}
