use anyhow::Context;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::app_config::constants::{
    Arch, Mode, ReactNativeArch, ReactNativePlatform, ARCH_ARM_64_V8, ARCH_ARM_V7_ABI, ARCH_I686,
    ARCH_X86_64, REACT_NATIVE_APP_DIR, REACT_NATIVE_BINDINGS_DIR,
};

use super::PlatformBuilder;

// Maintained for backwards compatibility
#[inline]
pub fn build() {
    super::build_from_env::<ReactNativePlatform>()
}

impl PlatformBuilder for ReactNativePlatform {
    type Arch = ReactNativeArch;
    type Params = ();

    fn build(
        mode: Mode,
        project_dir: &Path,
        target_archs: Vec<Self::Arch>,
        _params: Self::Params,
    ) -> anyhow::Result<PathBuf> {
        let bindings_dir = setup_bindings_dir(project_dir)?;
        generate_react_native_bindings(project_dir, target_archs, mode, &bindings_dir)?;
        Ok(PathBuf::from(REACT_NATIVE_BINDINGS_DIR))
    }
}

/// Install `uniffi-bindgen-react-native` if it isn't already on `PATH`.
pub fn install_uniffi_bindgen_react_native() -> anyhow::Result<()> {
    let output = Command::new("uniffi-bindgen-react-native").output();
    match output {
        Ok(_) => {
            // Command exists, no need to install
            println!("uniffi-bindgen-react-native already installed.");
            return Ok(());
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            // Command not found, proceed with installation
            println!("uniffi-bindgen-react-native not found, installing...");
            let current_path: PathBuf = std::env::current_dir()?;
            let status = Command::new("git")
                .args([
                    "clone",
                    "https://github.com/zkmopro/uniffi-bindgen-react-native.git",
                ])
                .current_dir(current_path.clone())
                .status()
                .expect("failed to download uniffi-bindgen-react-native");
            if !status.success() {
                return Err(anyhow::anyhow!(
                    "Failed to download uniffi-bindgen-react-native"
                ));
            }

            let status = Command::new("cargo")
                .args(["install", "--path", "."])
                .current_dir(current_path.join("uniffi-bindgen-react-native/crates/ubrn_cli"))
                .status()
                .expect("failed to install uniffi-bindgen-react-native");
            if !status.success() {
                return Err(anyhow::anyhow!(
                    "Failed to install uniffi-bindgen-react-native"
                ));
            }
            fs::remove_dir_all(current_path.join("uniffi-bindgen-react-native"))
                .expect("failed to remove uniffi-bindgen-react-native");
        }
        Err(e) => {
            // Other error, propagate it
            return Err(anyhow::anyhow!(
                "Failed to check for uniffi-bindgen-react-native: {}",
                e
            ));
        }
    }

    Ok(())
}

/// Install `uniffi-bindgen-react-native`, and set up the `MoproReactNativeBindings`
/// directory (copy the template, point `ubrn.config.yaml` at `project_dir`).
/// Returns the bindings directory. Idempotent: safe to call before every build.
pub fn setup_bindings_dir(project_dir: &Path) -> anyhow::Result<PathBuf> {
    install_uniffi_bindgen_react_native()?;

    let bindings_dir = project_dir.join(REACT_NATIVE_BINDINGS_DIR);
    fs::create_dir_all(&bindings_dir).expect("failed to create bindings directory");

    // Copy the react_native template to the project directory
    // Get the path to the template directory relative to this source file
    let template_dir =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("src/app_config/template/react_native");
    let mut copy_options = fs_extra::dir::CopyOptions::new();
    copy_options.overwrite = true;
    copy_options.content_only = true;
    fs_extra::dir::copy(&template_dir, &bindings_dir, &copy_options)
        .with_context(|| format!("Failed to copy react_native folder from {:?}", template_dir))?;

    // Replace the <%PATH_TO_PROJECT%> in the ubrn.config.yaml template with the project directory
    let target_file = bindings_dir.join("ubrn.config.yaml");

    let contents = fs::read_to_string(&target_file)
        .with_context(|| format!("Failed to read ubrn.config.yaml from {:?}", target_file))?
        .replace("<%PATH_TO_PROJECT%>", &project_dir.to_string_lossy());

    fs::write(&target_file, contents)
        .with_context(|| format!("Failed to write ubrn.config.yaml to {:?}", target_file))?;

    Ok(bindings_dir)
}

/// Run `uniffi-bindgen-react-native generate jsi turbo-module` in `bindings_dir`.
pub fn generate_jsi_bindings(bindings_dir: &Path) -> anyhow::Result<()> {
    let status = Command::new("uniffi-bindgen-react-native")
        .args(["generate", "jsi", "turbo-module"])
        .current_dir(bindings_dir)
        .status()
        .expect("failed to generate react native bindings");
    if !status.success() {
        return Err(anyhow::anyhow!("Failed to generate react native bindings"));
    }
    Ok(())
}

fn generate_react_native_bindings(
    _project_dir: &Path,
    target_archs: Vec<ReactNativeArch>,
    mode: Mode,
    bindings_dir: &Path,
) -> anyhow::Result<()> {
    generate_jsi_bindings(bindings_dir)?;

    let ios_target_string = target_archs
        .iter()
        .filter(|arch| arch.as_str().contains("ios"))
        .map(|arch| arch.as_str())
        .collect::<Vec<&str>>()
        .join(",");
    let android_target_string = target_archs
        .iter()
        .filter(|arch| arch.as_str().contains("android"))
        .map(|arch| arch.as_str())
        .collect::<Vec<&str>>()
        .join(",");

    if !ios_target_string.is_empty() {
        build_for_arch("ios", mode, &ios_target_string, bindings_dir)?;
    }
    if !android_target_string.is_empty() {
        build_for_arch("android", mode, &android_target_string, bindings_dir)?;
        patch_android_cmake_lists_uniffi_bindgen_resolve(bindings_dir)?;
    }

    set_xcframework_package_files(bindings_dir)?;

    Ok(())
}

/// Include the xcframework in `package.json` for `mopro-react-native-package`.
pub fn set_xcframework_package_files(bindings_dir: &Path) -> anyhow::Result<()> {
    let npm_status = Command::new("npm")
        .args(["pkg", "set", "files[]=*.xcframework/**"])
        .current_dir(bindings_dir)
        .status()
        .expect("failed to set files in package.json");
    if !npm_status.success() {
        return Err(anyhow::anyhow!("Failed to set files in package.json"));
    }
    Ok(())
}

/// Run `uniffi-bindgen-react-native build <platform> --and-generate --targets <target_string>`
/// in `bindings_dir`. `platform` is `"ios"` or `"android"`.
pub fn build_for_arch(
    platform: &str,
    mode: Mode,
    target_string: &str,
    bindings_dir: &Path,
) -> anyhow::Result<()> {
    let mut args = vec![
        "build".to_string(),
        platform.to_string(),
        "--and-generate".to_string(),
    ];

    if mode == Mode::Release {
        args.push("--release".to_string());
    }

    args.push("--targets".to_string());
    args.push(target_string.to_string());

    let status = Command::new("uniffi-bindgen-react-native")
        .args(&args)
        .current_dir(bindings_dir)
        .status()
        .expect("failed to build react native bindings");
    if !status.success() {
        return Err(anyhow::anyhow!("Failed to build react native bindings"));
    }
    Ok(())
}

// uniffi-bindgen-react-native's generated android/CMakeLists.txt resolves its
// own package root via `require.resolve('uniffi-bindgen-react-native/package.json')`,
// which throws ERR_PACKAGE_PATH_NOT_EXPORTED on package versions whose
// "exports" field omits that subpath (0.31.0-3+, see
// zkmopro/uniffi-bindgen-react-native#399), breaking the Android C++ build
// (missing headers like UniffiCallInvoker.h). Rewrite the generated file to
// resolve the package's public "." export instead and walk up to the
// package root, which works regardless of the "exports" field. This is a
// no-op once the upstream fix ships in a published release.
pub fn patch_android_cmake_lists_uniffi_bindgen_resolve(bindings_dir: &Path) -> anyhow::Result<()> {
    let cmake_lists_path = bindings_dir.join("android").join("CMakeLists.txt");
    let contents = match fs::read_to_string(&cmake_lists_path) {
        Ok(contents) => contents,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(()),
        Err(e) => return Err(e).with_context(|| format!("Failed to read {:?}", cmake_lists_path)),
    };

    const BROKEN: &str = "require.resolve('uniffi-bindgen-react-native/package.json')";
    if !contents.contains(BROKEN) {
        return Ok(());
    }

    const FIXED: &str = "require('path').dirname(require('path').dirname(require('path').dirname(require('path').dirname(require.resolve('uniffi-bindgen-react-native')))))";
    let patched = contents.replace(BROKEN, FIXED).replace(
        "\n# Get the directory; get_filename_component and cmake_path will normalize\n\
# paths with Windows path separators.\n\
get_filename_component(UNIFFI_BINDGEN_PATH \"${UNIFFI_BINDGEN_PATH}\" DIRECTORY)\n",
        "\n",
    );

    fs::write(&cmake_lists_path, patched)
        .with_context(|| format!("Failed to patch {:?}", cmake_lists_path))?;
    Ok(())
}

fn android_abi_for_triple(triple: &str) -> Option<&'static str> {
    match triple {
        "aarch64-linux-android" => Some(ARCH_ARM_64_V8),
        "armv7-linux-androideabi" => Some(ARCH_ARM_V7_ABI),
        "i686-linux-android" => Some(ARCH_I686),
        "x86_64-linux-android" => Some(ARCH_X86_64),
        _ => None,
    }
}

/// The `react-native/android/gradle.properties` template (from the
/// `zkmopro/react-native-app` scaffold) defaults `reactNativeArchitectures` to
/// all four Android ABIs, but the native library is only built for the
/// architectures configured for this project. Gradle's CMake build fails with
/// a missing `.so` for any listed ABI that wasn't actually built, so narrow
/// the property to match what was built (appending the line if it's missing
/// entirely). `project_dir` is the mopro project root (the parent of the
/// `react-native/` app directory); a no-op if that app directory hasn't been
/// created yet (i.e. before `mopro create react-native` has run).
pub fn patch_gradle_properties_architectures(
    project_dir: &Path,
    android_target_strings: &[String],
) -> anyhow::Result<()> {
    let abis: Vec<&str> = android_target_strings
        .iter()
        .filter_map(|triple| android_abi_for_triple(triple))
        .collect();
    if abis.is_empty() {
        return Ok(());
    }

    let gradle_properties_path = project_dir
        .join(REACT_NATIVE_APP_DIR)
        .join("android")
        .join("gradle.properties");
    let contents = match fs::read_to_string(&gradle_properties_path) {
        Ok(contents) => contents,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(()),
        Err(e) => {
            return Err(e).with_context(|| format!("Failed to read {:?}", gradle_properties_path))
        }
    };

    const PREFIX: &str = "reactNativeArchitectures=";
    let new_line = format!("{PREFIX}{}", abis.join(","));

    // Match only a real assignment line (trimmed line starting with the
    // property name), not any occurrence of the substring — the template's
    // own comment block (`# ./gradlew <task> -PreactNativeArchitectures=x86_64`)
    // also contains "reactNativeArchitectures=" and would otherwise be
    // matched first, silently leaving the real property untouched.
    let existing_line = contents
        .lines()
        .find(|line| line.trim_start().starts_with(PREFIX));

    let patched = match existing_line {
        Some(line) => contents.replacen(line, &new_line, 1),
        // Property was removed from the template (e.g. by hand); append it
        // rather than silently leaving Gradle to fall back to building all
        // four ABIs again.
        None => {
            let mut patched = contents.clone();
            if !patched.ends_with('\n') && !patched.is_empty() {
                patched.push('\n');
            }
            patched.push_str(&new_line);
            patched.push('\n');
            patched
        }
    };

    fs::write(&gradle_properties_path, patched)
        .with_context(|| format!("Failed to patch {:?}", gradle_properties_path))?;
    Ok(())
}
