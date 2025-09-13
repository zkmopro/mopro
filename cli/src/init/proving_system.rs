use crate::init::adapter::Adapter;
use include_dir::Dir;
use std::fs;
use std::io::Write;
use std::path::Path;

pub(super) trait ProvingSystem {
    const TEMPLATE_DIR: Dir<'static>;
    const ADAPTER: Adapter;

    const DEPENDENCIES: &'static str = "";
    const BUILD_DEPENDENCIES: &'static str = "";
    const DEV_DEPENDENCIES: &'static str = "";

    const BUILD_TEMPLATE: &'static str = "";

    fn dep_template(file_path: &str) -> anyhow::Result<()> {
        let target = format!("{}_DEPENDENCIES", Self::ADAPTER.as_str().to_uppercase());
        append_below_string_in_file(file_path, &target, Self::DEPENDENCIES)
    }

    fn build_dep_template(_file_path: &str) -> anyhow::Result<()> {
        let target = format!(
            "{}_BUILD_DEPENDENCIES",
            Self::ADAPTER.as_str().to_uppercase()
        );
        append_below_string_in_file(_file_path, &target, Self::BUILD_DEPENDENCIES)
    }

    fn dev_dep_template(_file_path: &str) -> anyhow::Result<()> {
        let target = format!("{}_DEV_DEPENDENCIES", Self::ADAPTER.as_str().to_uppercase());
        append_below_string_in_file(_file_path, &target, Self::DEV_DEPENDENCIES)
    }

    fn lib_template(file_path: &str) -> anyhow::Result<()> {
        let circom_lib_rs = match Self::TEMPLATE_DIR.get_file("lib.rs") {
            Some(file) => file.contents(),
            None => return Err(anyhow::anyhow!("lib.rs not found in template")),
        };
        let target = format!("// {}_TEMPLATE", Self::ADAPTER.as_str().to_uppercase());
        append_below_string_in_file(file_path, &target, &String::from_utf8_lossy(circom_lib_rs))
    }

    fn mod_template(lib_file_path: &str) -> anyhow::Result<()> {
        let mod_file = format!("{}.rs", Self::ADAPTER.as_str().to_lowercase());

        let content = match Self::TEMPLATE_DIR.get_file(&mod_file) {
            Some(file) => file.contents(),
            None => return Err(anyhow::anyhow!("{mod_file} not found in template")),
        };

        // Place the circom.rs in the same directory as lib.rs
        let dest_path = Path::new(lib_file_path)
            .parent()
            .ok_or_else(|| anyhow::anyhow!("Invalid file_path: no parent directory"))?
            .join(mod_file);

        fs::write(&dest_path, content).map_err(|e| anyhow::anyhow!("{}", e))
    }

    fn test_template(lib_file_path: &str) -> anyhow::Result<()> {
        let tmpl_tests: &Dir = Self::TEMPLATE_DIR
            .get_dir("tests")
            .ok_or_else(|| anyhow::anyhow!("tests dir not found in template"))?;

        let dest_tests_dir: &Path = Path::new(lib_file_path)
            .parent()
            .ok_or_else(|| anyhow::anyhow!("Invalid file_path: no parent directory"))?
            .parent()
            .ok_or_else(|| anyhow::anyhow!("Invalid file_path: no grandparent directory"))?;

        fs::create_dir_all(dest_tests_dir)
            .map_err(|e| anyhow::anyhow!("failed to create tests dir: {}", e))?;

        tmpl_tests.extract(dest_tests_dir)?;

        Ok(())
    }

    fn build_template(_file_path: &str) -> anyhow::Result<()> {
        let target = format!("// {}_TEMPLATE", Self::ADAPTER.as_str().to_uppercase());
        append_below_string_in_file(_file_path, &target, Self::BUILD_TEMPLATE)
    }
}

fn append_below_string_in_file(
    file_path: &str,
    target: &str,
    replacement: &str,
) -> anyhow::Result<()> {
    // Read the entire content of the file
    let content = fs::read_to_string(file_path)?;

    // Replace the target string with the replacement string
    let modified_content = content.replace(target, &format!("{target}\n{replacement}"));

    // Open the file in write mode, which truncates the file content
    let mut file = fs::File::create(file_path)?;

    // Write the modified content back to the file
    file.write_all(modified_content.as_bytes())?;

    Ok(())
}
