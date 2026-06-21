use anyhow::Context;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use toml::Value;

use mopro_build_common::{
    build_from_env, raw_project_name_from_toml,
    PlatformBuilder, FLUTTER_BINDINGS_DIR,
};
pub use mopro_build_common::{FlutterArch, Mode};

pub struct FlutterPlatform;

pub fn build() {
    build_from_env::<FlutterPlatform>()
}

impl PlatformBuilder for FlutterPlatform {
    type Arch = FlutterArch;
    type Params = ();

    fn identifier() -> &'static str { "flutter" }

    fn build(
        _mode: Mode,
        project_dir: &Path,
        _target_archs: Vec<Self::Arch>,
        _params: Self::Params,
    ) -> anyhow::Result<PathBuf> {
        init_flutter_bindings(project_dir)?;

        let cargo_toml_path = project_dir.join(FLUTTER_BINDINGS_DIR).join("rust/Cargo.toml");
        ensure_workspace_toml(cargo_toml_path.to_string_lossy().as_ref());

        let third_party_crate_name = raw_project_name_from_toml(project_dir)?;
        let cargo_add_status = Command::new("cargo")
            .args([
                "add", &third_party_crate_name,
                "--path", project_dir.to_string_lossy().as_ref(),
                "--no-default-features", "--features", "flutter",
            ])
            .current_dir(project_dir.join(FLUTTER_BINDINGS_DIR).join("rust"))
            .status().expect("failed to run cargo add");
        if !cargo_add_status.success() {
            return Err(anyhow::anyhow!("Failed to add third party crate"));
        }

        replace_relative_path_with_absolute(&cargo_toml_path, &third_party_crate_name, project_dir)?;
        patch_cargokit_build_script(project_dir)?;
        add_cpp_flag_to_ios_podspec(project_dir)?;
        disable_android_architecture_support(project_dir)?;
        copy_libcxx_shared_so_to_jni_libs(project_dir)?;

        let rust_root = project_dir.join(FLUTTER_BINDINGS_DIR).join("rust");
        let dart_output = project_dir.join(FLUTTER_BINDINGS_DIR).join("lib/src/rust");
        let generate_status = Command::new("flutter_rust_bridge_codegen")
            .args([
                "generate",
                "--rust-root", &rust_root.to_string_lossy(),
                "--rust-input", &third_party_crate_name,
                "--dart-output", &dart_output.to_string_lossy(),
            ])
            .current_dir(project_dir)
            .status().expect("failed to run flutter_rust_bridge_codegen");
        if !generate_status.success() {
            return Err(anyhow::anyhow!("Failed to generate FRB bindings"));
        }

        Ok(PathBuf::from(FLUTTER_BINDINGS_DIR))
    }
}

fn install_flutter_rust_bridge_codegen() -> anyhow::Result<()> {
    match Command::new("flutter_rust_bridge_codegen").output() {
        Ok(_) => Ok(()),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            let status = Command::new("cargo")
                .args(["install", "flutter_rust_bridge_codegen@=2.11.1"])
                .status().expect("failed to install flutter_rust_bridge_codegen");
            if !status.success() {
                Err(anyhow::anyhow!("Failed to install flutter_rust_bridge_codegen"))
            } else { Ok(()) }
        }
        Err(e) => Err(anyhow::anyhow!("Failed to check for flutter_rust_bridge_codegen: {}", e)),
    }
}

fn init_flutter_bindings(project_dir: &Path) -> anyhow::Result<()> {
    let flutter_bindings_dir = project_dir.join(FLUTTER_BINDINGS_DIR);
    install_flutter_rust_bridge_codegen()?;
    if !flutter_bindings_dir.exists() {
        let status = Command::new("flutter_rust_bridge_codegen")
            .args(["create", FLUTTER_BINDINGS_DIR, "--template", "plugin"])
            .status().expect("failed to run flutter_rust_bridge_codegen");
        if !status.success() {
            return Err(anyhow::anyhow!("flutter_rust_bridge_codegen failed"));
        }
    }
    Ok(())
}

fn ensure_workspace_toml(cargo_toml_path: &str) {
    let content = fs::read_to_string(cargo_toml_path).expect("Failed to read Cargo.toml");
    if !content.contains("[workspace]") {
        let new_content = format!("{content}\n\n[workspace]\n");
        fs::write(cargo_toml_path, new_content).expect("Failed to write updated Cargo.toml");
    }
}

fn replace_relative_path_with_absolute(
    cargo_toml_path: &Path,
    crate_name: &str,
    abs_path: &Path,
) -> anyhow::Result<()> {
    let content = fs::read_to_string(cargo_toml_path).context("Failed to read Cargo.toml")?;
    let mut cargo_toml: Value = content.parse::<Value>().context("Failed to parse Cargo.toml")?;

    if let Some(Value::Table(table)) = cargo_toml.get_mut("dependencies").and_then(|d| d.get_mut(crate_name)) {
        table.insert("path".to_string(), Value::String(abs_path.to_string_lossy().to_string()));
    }

    let updated = toml::to_string_pretty(&cargo_toml).context("Failed to serialize Cargo.toml")?;
    fs::write(cargo_toml_path, updated).context("Failed to write updated Cargo.toml")
}

fn patch_cargokit_build_script(project_dir: &Path) -> anyhow::Result<()> {
    let path = project_dir.join(FLUTTER_BINDINGS_DIR).join("cargokit/gradle/plugin.gradle");
    let content = fs::read_to_string(path.clone()).context("Failed to read plugin.gradle")?;
    if !content.contains("if (plugin.class.name == \"com.flutter.gradle.FlutterPlugin\" || plugin.class.name == \"FlutterPlugin\")") {
        let updated = content
            .replace(
                "if (plugin.class.name == \"com.flutter.gradle.FlutterPlugin\")",
                "if (plugin.class.name == \"com.flutter.gradle.FlutterPlugin\" || plugin.class.name == \"FlutterPlugin\")"
            )
            .replace(
                "        def platforms = com.flutter.gradle.FlutterPluginUtils.getTargetPlatforms(project).collect()",
                "        def List<String> platforms\n            try {\n                platforms = com.flutter.gradle.FlutterPluginUtils.getTargetPlatforms(project).collect()\n            } catch (Exception ignored) {\n                platforms = plugin.getTargetPlatforms().collect()\n            }"
            );
        fs::write(&path, updated).context("Failed to write updated plugin.gradle")?;
    }
    Ok(())
}

fn add_cpp_flag_to_ios_podspec(project_dir: &Path) -> anyhow::Result<()> {
    let path = project_dir.join(FLUTTER_BINDINGS_DIR).join("ios")
        .join(format!("{FLUTTER_BINDINGS_DIR}.podspec"));
    let content = fs::read_to_string(path.clone()).context("Failed to read podspec")?;
    if !content.contains("-lc++") {
        let updated = content.replace(
            "'OTHER_LDFLAGS' => '-force_load ${BUILT_PRODUCTS_DIR}/libmopro_flutter_bindings.a'",
            "'OTHER_LDFLAGS' => '-force_load ${BUILT_PRODUCTS_DIR}/libmopro_flutter_bindings.a -lc++'",
        );
        fs::write(&path, updated).context("Failed to write updated podspec")?;
    }
    Ok(())
}

fn disable_android_architecture_support(project_dir: &Path) -> anyhow::Result<()> {
    let path = project_dir.join(FLUTTER_BINDINGS_DIR).join("cargokit/gradle/plugin.gradle");
    let content = fs::read_to_string(path.clone()).context("Failed to read plugin.gradle")?;
    let updated = content
        .replace("        platforms.add(\"android-x86\")", "")
        .replace("        platforms.add(\"android-x64\")", "");
    fs::write(&path, updated).context("Failed to write updated plugin.gradle")
}

fn copy_libcxx_shared_so_to_jni_libs(project_dir: &Path) -> anyhow::Result<()> {
    let path = project_dir.join(FLUTTER_BINDINGS_DIR).join("cargokit/gradle/plugin.gradle");
    let content = fs::read_to_string(path.clone()).context("Failed to read plugin.gradle")?;
    if !content.contains("// After cargo build in CargoKitBuildTask.build()") {
        let updated = content.replace(
            "project.tasks.whenTaskAdded onTask",
            "project.tasks.whenTaskAdded onTask\n
                // After cargo build in CargoKitBuildTask.build()
                def outputDir = new File(cargoOutputDir)
                def ndkDir = plugin.project.android.ndkDirectory
                def abiMap = [
                    \"arm64-v8a\" : \"aarch64-linux-android\",
                    \"armeabi-v7a\" : \"arm-linux-androideabi\",
                    \"x86\"        : \"i686-linux-android\",
                    \"x86_64\"     : \"x86_64-linux-android\"
                ]
                abiMap.each { abi, triple ->
                    def srcLibcxx = new File(\"${ndkDir}/toolchains/llvm/prebuilt/${Os.isFamily(Os.FAMILY_MAC) ? \"darwin-x86_64\" : \"linux-x86_64\"}/sysroot/usr/lib/${triple}/libc++_shared.so\")
                    def destDir = new File(\"${outputDir}/${abi}\")
                    destDir.mkdirs()
                    project.copy { from srcLibcxx; into destDir }
                }"
        );
        fs::write(&path, updated).context("Failed to write updated plugin.gradle")?;
    }
    Ok(())
}
