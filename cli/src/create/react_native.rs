use super::Create;
use crate::config::read_config;
use crate::constants::Platform;
use crate::create::utils::{check_bindings, copy_dir, copy_keys, download_and_extract_template};
use crate::print::print_footer_message;
use crate::style::print_green_bold;

use anyhow::{Context, Error, Result};
use mopro_ffi::app_config::constants::{REACT_NATIVE_APP_DIR, REACT_NATIVE_BINDINGS_DIR};
use mopro_ffi::app_config::react_native::patch_gradle_properties_architectures;
use std::{fs, path::PathBuf};

pub struct ReactNative;

impl Create for ReactNative {
    const NAME: &'static str = REACT_NATIVE_APP_DIR;

    fn create(project_dir: PathBuf) -> Result<()> {
        let react_native_bindings_dir = check_bindings(&project_dir, Platform::ReactNative)?;

        let target_dir = project_dir.join(Self::NAME);
        if target_dir.exists() {
            return Err(Error::msg(format!(
                "The directory {} already exists. Please remove it and try again.",
                target_dir.display()
            )));
        }
        download_and_extract_template(
            "https://github.com/zkmopro/react-native-app/archive/refs/heads/ubrn.zip",
            &project_dir,
            Self::NAME,
        )?;

        let react_native_dir = project_dir.join("react-native-app-ubrn");
        fs::rename(react_native_dir, &target_dir)?;

        let mopro_module_dir = target_dir.join(REACT_NATIVE_BINDINGS_DIR);
        copy_dir(
            react_native_bindings_dir.as_ref().unwrap(),
            &mopro_module_dir,
        )?;
        remove_stale_web_entrypoint(&mopro_module_dir)?;

        let assets_dir = target_dir.join("assets/keys");
        fs::remove_dir_all(&assets_dir)?;
        fs::create_dir(&assets_dir)?;

        copy_keys(assets_dir)?;

        // The downloaded scaffold's `android/gradle.properties` defaults
        // `reactNativeArchitectures` to all four ABIs, but the bindings dir
        // we just copied in only has `.so`s for the architectures this
        // project was built for. Narrow it now so a fresh `create` doesn't
        // reintroduce the "missing .so, no known rule to make it" ninja
        // failure for ABIs that were never built.
        let config_path = project_dir.join("Config.toml");
        match read_config(&config_path) {
            Ok(config) => {
                if let Some(react_native_archs) = config.react_native {
                    let android_archs: Vec<String> = react_native_archs
                        .into_iter()
                        .filter(|a| a.contains("android"))
                        .collect();
                    if !android_archs.is_empty() {
                        patch_gradle_properties_architectures(&project_dir, &android_archs)?;
                    }
                }
            }
            Err(e)
                if e.downcast_ref::<std::io::Error>()
                    .is_some_and(|io_err| io_err.kind() == std::io::ErrorKind::NotFound) => {}
            Err(e) => return Err(e).with_context(|| format!("Failed to read {:?}", config_path)),
        }

        Self::print_message();
        Ok(())
    }

    fn print_message() {
        print_green_bold("React Native template created successfully!".to_string());
        println!();
        print_green_bold("Next steps:".to_string());
        println!();
        print_green_bold(
            "  Refer to the README.md in the `react-native` folder for instructions on running the app.".to_string(),
        );
        print_footer_message();
    }
}

/// The downloaded `zkmopro/react-native-app` scaffold ships a static
/// `src/index.web.ts` (for optional web/wasm support) that imports from
/// `./generated/wasm-bindgen/index.js` and `index_bg.wasm`. `mopro build` never
/// generates that `generated/wasm-bindgen` directory — React Native builds only
/// target iOS/Android — so the file is always a dangling reference. Since
/// `copy_dir` only overwrites files present in the built bindings dir, it can't
/// remove this pre-existing one; left in place, it breaks `npm install`'s
/// `prepare: bob build` step (`tsc` fails to resolve the missing module).
fn remove_stale_web_entrypoint(mopro_module_dir: &std::path::Path) -> Result<()> {
    let index_web_ts = mopro_module_dir.join("src").join("index.web.ts");
    if index_web_ts.exists() {
        fs::remove_file(&index_web_ts)?;
    }
    Ok(())
}
