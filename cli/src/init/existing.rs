use anyhow::{bail, Context, Result};
use std::fs;
use std::io::{ErrorKind, Write};
use std::path::Path;
use tempfile::NamedTempFile;
use toml_edit::{Array, DocumentMut, InlineTable, Item, Table, TableLike, Value};

const MOPRO_VERSION: &str = concat!("=", env!("CARGO_PKG_VERSION"));

pub(crate) fn try_init() -> Result<bool> {
    let project_dir = std::env::current_dir()?;
    let manifest = project_dir.join("Cargo.toml");
    let contents = match fs::read_to_string(&manifest) {
        Ok(contents) => contents,
        Err(error) if error.kind() == ErrorKind::NotFound => return Ok(false),
        Err(error) => return Err(error).context("Failed to read Cargo.toml"),
    };
    let mut doc: DocumentMut = contents.parse().context("Invalid Cargo.toml")?;
    if !doc.contains_key("package") {
        return Ok(false);
    }

    init_existing_project(&project_dir, &contents, &mut doc)?;
    println!("Initialized Mopro in the current Rust project. Run `mopro build --platforms android` or `mopro build --platforms ios` to generate bindings.");
    Ok(true)
}

fn init_existing_project(
    project_dir: &Path,
    original_manifest: &str,
    doc: &mut DocumentMut,
) -> Result<()> {
    let lib_source = doc
        .get("lib")
        .and_then(|lib| lib.get("path"))
        .and_then(Item::as_str)
        .unwrap_or("src/lib.rs");
    let lib_path = project_dir.join(lib_source);
    let original_lib = match fs::read_to_string(&lib_path) {
        Ok(contents) => Some(contents),
        Err(error) if error.kind() == ErrorKind::NotFound && lib_source == "src/lib.rs" => None,
        Err(error) => {
            return Err(error).with_context(|| format!("Failed to read {}", lib_path.display()))
        }
    };

    let lib = manifest_table(doc, "lib")?;
    let crate_types = array(
        lib.entry("crate-type").or_insert(Item::None),
        "lib.crate-type",
    )?;
    if crate_types.is_empty() {
        crate_types.push("lib");
    }
    for crate_type in ["cdylib", "staticlib"] {
        add_to_array(crate_types, crate_type);
    }

    let dependencies = manifest_table(doc, "dependencies")?;
    let has_uniffi = dependencies.contains_key("uniffi");
    let dependency = dependencies.entry("mopro-ffi").or_insert(Item::None);
    if dependency.is_none() || dependency.is_str() {
        let mut table = InlineTable::new();
        table.insert(
            "version",
            Value::from(dependency.as_str().unwrap_or(MOPRO_VERSION)),
        );
        *dependency = Item::Value(Value::InlineTable(table));
    }
    let table = dependency
        .as_table_like_mut()
        .context("Invalid mopro-ffi dependency")?;
    if table.get("optional").and_then(Item::as_bool) == Some(true)
        || table
            .get("package")
            .and_then(Item::as_str)
            .is_some_and(|name| name != "mopro-ffi")
    {
        bail!(
            "Existing `mopro-ffi` dependency must refer to the Mopro FFI crate and be non-optional"
        );
    }
    let features = table
        .entry("features")
        .or_insert_with(|| toml_edit::value(Array::new()));
    let features = array(features, "mopro-ffi.features")?;
    for feature in ["uniffi", "build"] {
        add_to_array(features, feature);
    }

    let source = original_lib.as_deref().unwrap_or_default();
    let manifest = doc.to_string();
    let new_source = if source.lines().map(str::trim).any(|line| {
        [
            "mopro_ffi::app!",
            "mopro_ffi::uniffi_setup!",
            "uniffi::setup_scaffolding!",
        ]
        .iter()
        .any(|macro_name| line.starts_with(macro_name))
    }) {
        None
    } else {
        let scaffolding = if has_uniffi {
            "uniffi::setup_scaffolding!();"
        } else {
            "mopro_ffi::uniffi_setup!();"
        };
        let newline = if source.contains("\r\n") {
            "\r\n"
        } else {
            "\n"
        };
        let separator = if source.is_empty() || source.ends_with('\n') {
            ""
        } else {
            newline
        };
        Some(format!("{source}{separator}{scaffolding}{newline}"))
    };

    if let Some(source) = &new_source {
        if let Some(parent) = lib_path.parent() {
            fs::create_dir_all(parent)?;
        }
        write_atomic(&lib_path, source)?;
    }
    if manifest != original_manifest {
        if let Err(error) = write_atomic(&project_dir.join("Cargo.toml"), &manifest) {
            if new_source.is_some() {
                match original_lib {
                    Some(original) => write_atomic(&lib_path, &original)?,
                    None => fs::remove_file(&lib_path)?,
                }
            }
            return Err(error).context("Failed to update Cargo.toml");
        }
    }
    Ok(())
}

fn manifest_table<'a>(doc: &'a mut DocumentMut, name: &str) -> Result<&'a mut dyn TableLike> {
    doc.entry(name)
        .or_insert_with(|| Item::Table(Table::new()))
        .as_table_like_mut()
        .with_context(|| format!("{name} must be a table"))
}

fn write_atomic(path: &Path, contents: &str) -> Result<()> {
    // Cargo accepts symlinked manifests and library source files.
    let resolved_path = if path.is_symlink() {
        path.canonicalize()?
    } else {
        path.to_path_buf()
    };
    let mut file =
        NamedTempFile::new_in(resolved_path.parent().context("Missing file directory")?)?;
    file.write_all(contents.as_bytes())?;
    if let Ok(metadata) = fs::metadata(&resolved_path) {
        file.as_file().set_permissions(metadata.permissions())?;
    }
    file.persist(&resolved_path)?;
    Ok(())
}

fn array<'a>(item: &'a mut Item, name: &str) -> Result<&'a mut Array> {
    if item.is_none() {
        *item = toml_edit::value(Array::new());
    }
    item.as_array_mut()
        .with_context(|| format!("{name} must be an array"))
}

fn add_to_array(array: &mut Array, value: &str) {
    if !array.iter().any(|item| item.as_str() == Some(value)) {
        array.push(value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initializes_existing_library_in_place_and_is_repeatable() -> Result<()> {
        let temp = tempfile::tempdir()?;
        let root = temp.path();
        let manifest = root.join("Cargo.toml");
        let lib = root.join("custom.rs");
        let original_manifest = r#"# Keep the existing settings.
[package]
name = "my-project"
version = "0.1.0"
edition = "2024"

[lib]
name = "custom_name"
path = "custom.rs"
crate-type = ["rlib"] # Keep the existing crate type.

[dependencies]
mopro-ffi = { version = "0.3", features = ["wasm"] }

[features]
default = ["custom"]
custom = []
"#;
        let original_source = "#![allow(dead_code)]\r\n#[uniffi::export]\r\npub fn custom() {}\r\n";
        fs::write(&manifest, original_manifest)?;
        fs::write(&lib, original_source)?;
        fs::write(root.join("build.rs"), "fn main() {}\n")?;

        init_existing_project(root, original_manifest, &mut original_manifest.parse()?)?;

        let initialized_manifest = fs::read_to_string(&manifest)?;
        assert!(initialized_manifest.contains("# Keep the existing settings."));
        assert!(initialized_manifest.contains("# Keep the existing crate type."));
        assert!(initialized_manifest.contains("name = \"custom_name\""));
        assert!(initialized_manifest.contains("default = [\"custom\"]"));
        assert!(initialized_manifest.contains("[\"rlib\", \"cdylib\", \"staticlib\"]"));
        assert!(initialized_manifest
            .contains("version = \"0.3\", features = [\"wasm\", \"uniffi\", \"build\"]"));
        let expected_lib = format!("{original_source}mopro_ffi::uniffi_setup!();\r\n");
        assert_eq!(fs::read_to_string(&lib)?, expected_lib);
        assert_eq!(fs::read_to_string(root.join("build.rs"))?, "fn main() {}\n");

        init_existing_project(
            root,
            &initialized_manifest,
            &mut initialized_manifest.parse()?,
        )?;
        assert_eq!(fs::read_to_string(manifest)?, initialized_manifest);
        assert_eq!(fs::read_to_string(lib)?, expected_lib);
        Ok(())
    }

    #[test]
    fn initializes_existing_binary_without_rewriting_main() -> Result<()> {
        let temp = tempfile::tempdir()?;
        let root = temp.path();
        let manifest = root.join("Cargo.toml");
        let original_manifest = "[package]\nname = \"my-project\"\nversion = \"0.1.0\"\n";
        let main = root.join("src/main.rs");
        fs::create_dir(root.join("src"))?;
        fs::write(&manifest, original_manifest)?;
        fs::write(&main, "fn main() {}\n")?;

        init_existing_project(root, original_manifest, &mut original_manifest.parse()?)?;

        let initialized: DocumentMut = fs::read_to_string(manifest)?.parse()?;
        let dependency = &initialized["dependencies"]["mopro-ffi"];
        assert_eq!(dependency["version"].as_str(), Some(MOPRO_VERSION));
        assert_eq!(
            fs::read_to_string(root.join("src/lib.rs"))?,
            "mopro_ffi::uniffi_setup!();\n"
        );
        assert_eq!(fs::read_to_string(main)?, "fn main() {}\n");
        Ok(())
    }

    #[test]
    fn preserves_existing_uniffi_scaffolding() -> Result<()> {
        let temp = tempfile::tempdir()?;
        let root = temp.path();
        let manifest = root.join("Cargo.toml");
        let original_manifest = "[package]\nname = \"my-project\"\nversion = \"0.1.0\"\n\n[dependencies]\nuniffi = \"0.32.0\"\n";
        let lib = root.join("src/lib.rs");
        let original_source = "uniffi::setup_scaffolding!();\n";
        fs::create_dir(root.join("src"))?;
        fs::write(&manifest, original_manifest)?;
        fs::write(&lib, original_source)?;

        init_existing_project(root, original_manifest, &mut original_manifest.parse()?)?;

        assert_eq!(fs::read_to_string(lib)?, original_source);
        Ok(())
    }
}
