use super::replace_string_in_file;
use super::ProvingSystem;
use anyhow::Result;
use include_dir::include_dir;
use include_dir::Dir;
use std::fs;
use std::path::Path;

pub struct Circom;

impl ProvingSystem for Circom {
    const TEMPLATE_DIR: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/src/template/circom");

    fn dep_template(file_path: &str) -> Result<()> {
        let replacement = r#"
# CIRCOM_DEPENDENCIES
circom-prover = { git = "https://github.com/zkmopro/mopro.git", features = ["rapidsnark"] }
rust-witness  = "0.1"
num-bigint    = "0.4.0"
    "#;

        let target = "# CIRCOM_DEPENDENCIES";
        replace_string_in_file(file_path, target, replacement)
    }

    fn build_dep_template(file_path: &str) -> Result<()> {
        let replacement = r#"
# CIRCOM_BUILD_DEPENDENCIES
witnesscalc-adapter = "0.1"
rust-witness = "0.1"
    "#;
        let target = "# CIRCOM_BUILD_DEPENDENCIES";
        replace_string_in_file(file_path, target, replacement)
    }

    fn dev_dep_template(file_path: &str) -> Result<()> {
        let replacement = r#"
# CIRCOM_DEV_DEPENDENCIES
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0.94"

circom-prover = { git = "https://github.com/zkmopro/mopro.git", features = ["rapidsnark", "witnesscalc"] }
witnesscalc-adapter = "0.1"
    "#;
        let target = "# CIRCOM_DEV_DEPENDENCIES";
        replace_string_in_file(file_path, target, replacement)
    }

    fn lib_template(file_path: &str) -> Result<()> {
        let circom_lib_rs = match Self::TEMPLATE_DIR.get_file("lib.rs") {
            Some(file) => file.contents(),
            None => return Err(anyhow::anyhow!("lib.rs not found in template")),
        };
        let target = "// CIRCOM_TEMPLATE";
        replace_string_in_file(file_path, target, &String::from_utf8_lossy(circom_lib_rs))
    }

    fn mod_template(lib_file_path: &str) -> Result<()> {
        let mod_file = "circom.rs";
        let circom_rs = match Self::TEMPLATE_DIR.get_file(mod_file) {
            Some(file) => file.contents(),
            None => return Err(anyhow::anyhow!("circom.rs not found in template")),
        };

        // Place the circom.rs in the same directory as lib.rs
        let dest_path = Path::new(lib_file_path)
            .parent()
            .ok_or_else(|| anyhow::anyhow!("Invalid file_path: no parent directory"))?
            .join(mod_file);

        fs::write(&dest_path, circom_rs).map_err(|e| anyhow::anyhow!("{}", e))
    }

    fn build_template(file_path: &str) -> Result<()> {
        let replacement = r#"
rust_witness::transpile::transpile_wasm("./test-vectors/circom".to_string());
witnesscalc_adapter::build_and_link("../test-vectors/circom/witnesscalc");

// For running the uniffi tests on macOS, we need to set the rpath to find the
// witnesscalc dynamic library at runtime.
#[cfg(target_os = "macos")]
{
    let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let wc_dir = out_dir.join("witnesscalc/package/lib");

    if wc_dir.exists() {
        println!("cargo:rustc-link-arg=-Wl,-rpath,{}", wc_dir.display());
    } else {
        panic!(
            "Expected witnesscalc lib path does not exist: {}",
            wc_dir.display()
        );
    }

    // TODO: Make this universal for all witnesscalc circuit libraries
    let src = wc_dir.join("libwitnesscalc_multiplier2_witnesscalc.dylib");
    let dst = wc_dir.join("libwitnesscalc_multiplier2.dylib");

    if src.exists() && !dst.exists() {
        let _ = std::fs::copy(&src, &dst);
    }
}
"#;

        let target = "// CIRCOM_TEMPLATE";
        replace_string_in_file(file_path, target, replacement)
    }
}
