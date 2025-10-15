use crate::init::adapter::Adapter;
use crate::init::replace_string_in_file;
use include_dir::Dir;
use std::fs;
use std::io::Write;

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

    fn lib_stub_template(file_path: &str) -> anyhow::Result<()> {
        let stub = format!("{}_stub!();", Self::ADAPTER.as_str().to_lowercase());
        let target = format!("// {}_TEMPLATE", Self::ADAPTER.as_str().to_uppercase());
        append_below_string_in_file(file_path, &target, stub.as_str())
    }

    fn build_template(file_path: &str) -> anyhow::Result<()> {
        let target = format!("// {}_TEMPLATE", Self::ADAPTER.as_str().to_uppercase());
        append_below_string_in_file(file_path, &target, Self::BUILD_TEMPLATE)
    }

    fn build_bindings_lib(bindings_lib_path: &str, project_name: &str) -> anyhow::Result<()> {
        let test_bindings_dir = format!("{}/{}", bindings_lib_path, Self::ADAPTER.as_str());
        replace_test_bindings_lib_import(&test_bindings_dir, project_name)
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

/// Replace the placeholder import lines in the test bindings files with the actual import statements
/// based on the project name and file type (Kotlin or Swift).
pub fn replace_test_bindings_lib_import(
    test_bindings_dir: &str,
    project_name: &str,
) -> anyhow::Result<()> {
    let project_lib_name = project_name.replace('-', "_");

    // We expect a structure of `tests/bindings/<adapter>/*.(kts|swift)`
    for file in fs::read_dir(test_bindings_dir)? {
        let file = file?;
        let file_path = file.path();

        if !file_path.is_file() {
            continue;
        }

        let target = "// GENERATED LIB IMPORT PLACEHOLDER";
        let replacement = match file_path.extension().and_then(|ext| ext.to_str()) {
            Some("kts") => Some(format!("import uniffi.{}.*", project_lib_name)),
            Some("swift") => Some(format!("import {}", project_lib_name)),
            _ => None,
        };

        if let Some(line) = replacement {
            replace_string_in_file(
                file_path
                    .to_str()
                    .ok_or(anyhow::anyhow!("Invalid test binding file path"))?,
                target,
                &line,
            )?;
        }
    }

    Ok(())
}
