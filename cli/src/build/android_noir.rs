//! Noir/barretenberg-specific Android build setup.
//!
//! `mopro-ffi`'s Android builder is adapter-free: it only applies generic
//! `AndroidBindingsParams` (per-arch env/RUSTFLAGS, min SDK, an optional separate
//! NDK-linked bindgen build). This module computes those params for Noir, because
//! barretenberg-rs's `build.rs` matches `linux` before `android` and would fetch a
//! glibc prebuilt Android can't `dlopen`. We pre-fetch the right prebuilt, point
//! `BB_LIB_DIR` at it, and link with Zig so the prebuilt's libc++ (`std::__1`)
//! resolves (the NDK's `std::__ndk1` is ABI-incompatible).

use anyhow::Context;
use mopro_ffi::app_config::android::{AndroidBindingsParams, ArchBuildConfig};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Build the [`AndroidBindingsParams`] a Noir project needs for `arch_strs`.
/// Returns the default (plain) params when the project doesn't use barretenberg-rs
/// or none of the requested archs has a published Android prebuilt.
pub fn android_bindings_params(
    project_dir: &Path,
    arch_strs: &[&String],
) -> anyhow::Result<AndroidBindingsParams> {
    let mut params = AndroidBindingsParams::default();

    // Aztec only publishes static Android prebuilts for arm64 and x86_64.
    let bb_targets: Vec<(String, &'static str)> = arch_strs
        .iter()
        .filter_map(|triple| bb_arch_for_triple(triple).map(|bb| (triple.to_string(), bb)))
        .collect();
    if bb_targets.is_empty() {
        return Ok(params);
    }

    let Some(version) = barretenberg_rs_version(project_dir)? else {
        return Ok(params);
    };

    let build_dir = project_dir.join("build");
    ensure_zig_available()?;

    for (triple, bb_arch) in &bb_targets {
        let bb_lib_dir = download_barretenberg_android_lib(bb_arch, &version, &build_dir)?;
        let linker_flag = zig_linker_flag(triple, &build_dir)?;
        params.arch_overrides.insert(
            triple.clone(),
            ArchBuildConfig {
                extra_env: vec![(
                    "BB_LIB_DIR".to_string(),
                    bb_lib_dir.to_string_lossy().into_owned(),
                )],
                extra_rustflags: vec![linker_flag],
            },
        );
    }

    // The barretenberg Android prebuilt imports symbols (e.g. __tls_get_addr, API
    // 29 on x86_64) above cargo-ndk's default of 21.
    params.min_sdk_version = Some(30);
    // Zig strips the `.symtab` uniffi-bindgen reads, so bind from a separate
    // NDK-linked build.
    params.relink_with_ndk_for_bindgen = true;
    Ok(params)
}

/// Map a Rust target triple to the barretenberg Android prebuilt arch, or `None`
/// when Aztec ships no static prebuilt for it.
fn bb_arch_for_triple(triple: &str) -> Option<&'static str> {
    match triple {
        "x86_64-linux-android" => Some("x86_64-android"),
        "aarch64-linux-android" => Some("arm64-android"),
        _ => None,
    }
}

/// Fetch the barretenberg Android static library for `bb_arch` at `version` and
/// return the dir to expose via `BB_LIB_DIR`. Cached per version under `build/`.
fn download_barretenberg_android_lib(
    bb_arch: &str,
    version: &str,
    build_dir: &Path,
) -> anyhow::Result<PathBuf> {
    // Cache per version: a noir-rs bump changes the prebuilt, and `build/` isn't
    // wiped between builds, so an arch-only key would relink a stale .a.
    let dest = build_dir
        .join("bb-android-prebuilt")
        .join(format!("{bb_arch}-{version}"));
    let lib = dest.join("libbb-external.a");
    if lib.exists() {
        return Ok(dest);
    }
    fs::create_dir_all(&dest).context("Failed to create barretenberg prebuilt dir")?;

    let url = format!(
        "https://github.com/AztecProtocol/aztec-packages/releases/download/v{version}/barretenberg-static-{bb_arch}.tar.gz"
    );
    let tarball = dest.join("barretenberg-static.tar.gz");

    let status = Command::new("curl")
        .args(["-L", "-f", "-o"])
        .arg(&tarball)
        .arg(&url)
        .status()
        .context("Failed to run curl for barretenberg Android prebuilt")?;
    if !status.success() {
        anyhow::bail!("Failed to download barretenberg Android prebuilt from {url}");
    }

    let status = Command::new("tar")
        .arg("-xzf")
        .arg(&tarball)
        .arg("-C")
        .arg(&dest)
        .status()
        .context("Failed to run tar for barretenberg Android prebuilt")?;
    if !status.success() {
        anyhow::bail!("Failed to extract barretenberg Android prebuilt");
    }
    let _ = fs::remove_file(&tarball);

    if !lib.exists() {
        anyhow::bail!(
            "barretenberg Android prebuilt extracted but libbb-external.a is missing in {}",
            dest.display()
        );
    }
    Ok(dest)
}

/// Error early if Zig isn't on `PATH`; the prebuilt is built with Zig's libc++.
fn ensure_zig_available() -> anyhow::Result<()> {
    let zig_ok = Command::new("zig")
        .arg("version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);
    if !zig_ok {
        anyhow::bail!(
            "Building the Noir (barretenberg) adapter for Android requires Zig on PATH \
             (https://ziglang.org/download/). The prebuilt barretenberg static library is \
             built with Zig's libc++ (std::__1) and must be linked with Zig so those symbols \
             resolve at runtime; the NDK's libc++ (std::__ndk1) is ABI-incompatible."
        );
    }
    Ok(())
}

/// Write a Zig-cc linker wrapper for `triple` and return the `-Clinker=<wrapper>`
/// flag. Zig provides the barretenberg prebuilt's `std::__1` libc++ symbols; only
/// the linker is overridden, so the NDK still compiles everything else.
fn zig_linker_flag(triple: &str, build_dir: &Path) -> anyhow::Result<String> {
    let ndk = android_ndk_home()?;
    let host = ndk_host_tag();
    let sysroot = ndk
        .join("toolchains")
        .join("llvm")
        .join("prebuilt")
        .join(host)
        .join("sysroot");
    let api = "30";

    let triple_lib = sysroot.join("usr").join("lib").join(triple);
    let crt_dir = triple_lib.join(api);
    let arch_include = sysroot.join("usr").join("include").join(triple);
    let generic_include = sysroot.join("usr").join("include");

    fs::create_dir_all(build_dir).context("Failed to create build dir for Zig linker config")?;
    let build_dir_abs = fs::canonicalize(build_dir).context("Failed to resolve build dir")?;

    // Zig ships no Android libc, so point it at the NDK's bionic + arch headers
    // (sys_include_dir is the arch dir, where the kernel `asm/` headers live).
    let libc_conf = build_dir_abs.join(format!("zig-android-libc-{triple}.txt"));
    fs::write(
        &libc_conf,
        format!(
            "include_dir={}\nsys_include_dir={}\ncrt_dir={}\nmsvc_lib_dir=\nkernel32_lib_dir=\ngcc_dir=\n",
            generic_include.display(),
            arch_include.display(),
            crt_dir.display(),
        ),
    )
    .context("Failed to write Zig libc config")?;

    // `zig cc` bakes in Zig's static `__1` libc++ (resolving barretenberg's `-lc++`);
    // the trailing `-lc++_shared` keeps the NDK's `__ndk1` libc++ for other C++.
    let wrapper = build_dir_abs.join(format!("zig-android-cc-{triple}.sh"));
    fs::write(
        &wrapper,
        format!(
            "#!/bin/sh\nset -e\nexport ZIG_LIBC=\"{libc}\"\nexec zig cc -target {triple} \
             -L \"{crt}\" -L \"{tl}\" \"$@\" -lc++_shared\n",
            libc = libc_conf.display(),
            triple = triple,
            crt = crt_dir.display(),
            tl = triple_lib.display(),
        ),
    )
    .context("Failed to write Zig linker wrapper")?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&wrapper)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&wrapper, perms)?;
    }

    Ok(format!("-Clinker={}", wrapper.display()))
}

/// Locate the Android NDK from the usual environment variables.
fn android_ndk_home() -> anyhow::Result<PathBuf> {
    for var in [
        "ANDROID_NDK_HOME",
        "ANDROID_NDK_ROOT",
        "ANDROID_NDK",
        "NDK_HOME",
        "NDK_PATH",
    ] {
        if let Ok(value) = std::env::var(var) {
            if !value.is_empty() {
                let path = PathBuf::from(value);
                if path.exists() {
                    return Ok(path);
                }
            }
        }
    }
    anyhow::bail!(
        "Could not locate the Android NDK. Set ANDROID_NDK_HOME to your NDK install; it is \
         required to link the Noir adapter for Android with Zig."
    );
}

/// NDK prebuilt host directory tag. Apple Silicon uses the x86_64 tools.
fn ndk_host_tag() -> &'static str {
    match std::env::consts::OS {
        "macos" => "darwin-x86_64",
        "windows" => "windows-x86_64",
        _ => "linux-x86_64",
    }
}

/// Resolve the pinned barretenberg-rs version from `project_dir`'s `Cargo.lock`
/// (generating one if absent). `None` when the crate is not a dependency.
fn barretenberg_rs_version(project_dir: &Path) -> anyhow::Result<Option<String>> {
    let lock_path = project_dir.join("Cargo.lock");
    if !lock_path.exists() {
        // We need the resolved version before cargo ndk would create the lock.
        let status = Command::new("cargo")
            .arg("generate-lockfile")
            .arg("--manifest-path")
            .arg(project_dir.join("Cargo.toml"))
            .status()
            .context("Failed to run cargo generate-lockfile")?;
        if !status.success() {
            anyhow::bail!(
                "cargo generate-lockfile failed; cannot resolve the barretenberg-rs version \
                 needed to fetch the correct Android prebuilt"
            );
        }
    }
    // A missing/unreadable lock is a real error, not "crate absent": returning
    // None would silently relink the wrong (glibc) prebuilt and break dlopen.
    let lock = fs::read_to_string(&lock_path)
        .with_context(|| format!("Failed to read {}", lock_path.display()))?;
    Ok(parse_barretenberg_rs_version(&lock))
}

/// Extract the `barretenberg-rs` package version from a `Cargo.lock` body.
fn parse_barretenberg_rs_version(lock: &str) -> Option<String> {
    let mut lines = lock.lines();
    while let Some(line) = lines.next() {
        if line.trim() == r#"name = "barretenberg-rs""# {
            for next in lines.by_ref() {
                let trimmed = next.trim();
                if let Some(rest) = trimmed.strip_prefix(r#"version = ""#) {
                    return rest.strip_suffix('"').map(str::to_string);
                }
                if trimmed == "[[package]]" {
                    break;
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::parse_barretenberg_rs_version;

    #[test]
    fn parses_barretenberg_rs_version_from_lockfile() {
        let lock = r#"
[[package]]
name = "anyhow"
version = "1.0.86"

[[package]]
name = "barretenberg-rs"
version = "4.2.0-aztecnr-rc.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
dependencies = [
 "serde",
]

[[package]]
name = "camino"
version = "1.1.9"
"#;
        assert_eq!(
            parse_barretenberg_rs_version(lock).as_deref(),
            Some("4.2.0-aztecnr-rc.2")
        );
    }

    #[test]
    fn returns_none_when_barretenberg_rs_absent() {
        let lock = r#"
[[package]]
name = "anyhow"
version = "1.0.86"

[[package]]
name = "camino"
version = "1.1.9"
"#;
        assert_eq!(parse_barretenberg_rs_version(lock), None);
    }

    #[test]
    fn does_not_confuse_a_dependency_mention_with_the_package_entry() {
        let lock = r#"
[[package]]
name = "noir"
version = "1.0.0-beta.19"
dependencies = [
 "barretenberg-rs",
]

[[package]]
name = "barretenberg-rs"
version = "4.3.0"
"#;
        assert_eq!(
            parse_barretenberg_rs_version(lock).as_deref(),
            Some("4.3.0")
        );
    }
}
