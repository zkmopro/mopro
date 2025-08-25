use color_eyre::eyre::ContextCompat;

pub const BUILD_MODE_ENV: &str = "CONFIGURATION";
pub const IOS_ARCHS_ENV: &str = "IOS_ARCHS";
pub const ANDROID_ARCHS_ENV: &str = "ANDROID_ARCHS";

pub const IOS_BINDINGS_DIR: &str = "MoproiOSBindings";
pub const IOS_SWIFT_FILE: &str = "mopro.swift";
pub const IOS_XCFRAMEWORKS_DIR: &str = "MoproBindings.xcframework";

pub const ANDROID_BINDINGS_DIR: &str = "MoproAndroidBindings";
pub const ANDROID_JNILIBS_DIR: &str = "jniLibs";
pub const ANDROID_UNIFFI_DIR: &str = "uniffi";
pub const ANDROID_PACKAGE_NAME: &str = "mopro";
pub const ANDROID_KT_FILE: &str = "mopro.kt";

pub const WEB_BINDINGS_DIR: &str = "MoproWasmBindings";

pub const ARCH_X86_64: &str = "x86_64";
pub const ARCH_ARM_64: &str = "aarch64";
pub const ARCH_I686: &str = "x86";
pub const ARCH_ARM_V7_ABI: &str = "armeabi-v7a";
pub const ARCH_ARM_64_V8: &str = "arm64-v8a";

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Mode {
    Debug,
    Release,
}

struct ModeInfo {
    mode: Mode,
    str: &'static str,
}

const MODES: [ModeInfo; 2] = [
    ModeInfo {
        mode: Mode::Debug,
        str: "debug",
    },
    ModeInfo {
        mode: Mode::Release,
        str: "release",
    },
];

impl Mode {
    pub fn as_str(&self) -> &'static str {
        MODES
            .iter()
            .find(|info| info.mode == *self)
            .map(|info| info.str)
            .expect("Unsupported Mode, only support 'release' and 'debug'")
    }

    pub fn parse_from_str(s: &str) -> Self {
        MODES
            .iter()
            .find(|info| info.str.to_lowercase() == s.to_lowercase())
            .map(|info| info.mode)
            .expect("Unsupported Mode String, only support 'release' and 'debug'")
    }

    pub fn from_idx(idx: usize) -> Self {
        MODES[idx].mode
    }

    pub fn idx(s: &str) -> Option<usize> {
        MODES
            .iter()
            .enumerate()
            .find(|(_, m)| m.str == s)
            .map(|(i, _)| i)
    }

    pub fn all_strings() -> Vec<&'static str> {
        MODES.iter().map(|info| info.str).collect()
    }
}

//
// Architecture Section
//

pub trait Arch {
    fn platform() -> Box<dyn Platform>;
    fn as_str(&self) -> &'static str;
    fn parse_from_str<S: AsRef<str>>(s: S) -> Self;
    fn all_strings() -> Vec<&'static str>;
    fn all_display_strings() -> Vec<(String, String)>;
    fn env_var_name() -> &'static str;
}

// https://developer.apple.com/documentation/xcode/build-settings-reference#Architectures
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum IosArch {
    Aarch64Apple,
    Aarch64AppleSim,
    X8664Apple,
}

struct IosArchInfo {
    #[allow(dead_code)] // currently not used
    arch: IosArch,
    str: &'static str,
    description: &'static str,
}

const IOS_ARCHS: [IosArchInfo; 3] = [
    IosArchInfo {
        arch: IosArch::Aarch64Apple,
        str: "aarch64-apple-ios",
        description: "64-bit iOS devices (iPhone/iPad)",
    },
    IosArchInfo {
        arch: IosArch::Aarch64AppleSim,
        str: "aarch64-apple-ios-sim",
        description: "ARM64 iOS simulator on Apple Silicon Macs",
    },
    IosArchInfo {
        arch: IosArch::X8664Apple,
        str: "x86_64-apple-ios",
        description: "x86_64 iOS simulator on Intel Macs",
    },
];

impl Arch for IosArch {
    fn platform() -> Box<dyn Platform> {
        Box::new(IosPlatform)
    }

    fn as_str(&self) -> &'static str {
        IOS_ARCHS
            .iter()
            .find(|info| info.arch == *self)
            .map(|info| info.str)
            .expect("Unsupported iOS Arch")
    }

    fn parse_from_str<S: AsRef<str>>(s: S) -> Self {
        IOS_ARCHS
            .iter()
            .find(|info| info.str.to_lowercase() == s.as_ref().to_lowercase())
            .map(|info| info.arch)
            .context(format!("Unsupported iOS Arch '{}'", s.as_ref()))
            .unwrap()
    }

    fn all_strings() -> Vec<&'static str> {
        IOS_ARCHS.iter().map(|info| info.str).collect()
    }

    fn all_display_strings() -> Vec<(String, String)> {
        IOS_ARCHS
            .iter()
            .map(|info| (info.str.to_string(), info.description.to_string()))
            .collect()
    }

    fn env_var_name() -> &'static str {
        IOS_ARCHS_ENV
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AndroidArch {
    X8664Linux,
    I686Linux,
    Armv7LinuxAbi,
    Aarch64Linux,
}

struct AndroidArchInfo {
    arch: AndroidArch,
    str: &'static str,
    description: &'static str,
}

const ANDROID_ARCHS: [AndroidArchInfo; 4] = [
    AndroidArchInfo {
        arch: AndroidArch::X8664Linux,
        str: "x86_64-linux-android",
        description: "64-bit Android emulators (x86_64 architecture)",
    },
    AndroidArchInfo {
        arch: AndroidArch::I686Linux,
        str: "i686-linux-android",
        description: "32-bit Android emulators (x86 architecture, legacy)",
    },
    AndroidArchInfo {
        arch: AndroidArch::Armv7LinuxAbi,
        str: "armv7-linux-androideabi",
        description: "32-bit ARM devices (older Android smartphones/tablets)",
    },
    AndroidArchInfo {
        arch: AndroidArch::Aarch64Linux,
        str: "aarch64-linux-android",
        description: "64-bit ARM devices (modern Android smartphones/tablets)",
    },
];

impl Arch for AndroidArch {
    fn platform() -> Box<dyn Platform> {
        Box::new(AndroidPlatform)
    }

    fn as_str(&self) -> &'static str {
        ANDROID_ARCHS
            .iter()
            .find(|info| info.arch == *self)
            .map(|info| info.str)
            .expect("Unsupported Android Arch")
    }

    fn parse_from_str<S: AsRef<str>>(s: S) -> Self {
        ANDROID_ARCHS
            .iter()
            .find(|info| info.str.to_lowercase() == s.as_ref().to_lowercase())
            .map(|info| info.arch)
            .context(format!("Unsupported Android Arch '{}'", s.as_ref()))
            .unwrap()
    }

    fn all_strings() -> Vec<&'static str> {
        ANDROID_ARCHS.iter().map(|info| info.str).collect()
    }

    fn all_display_strings() -> Vec<(String, String)> {
        ANDROID_ARCHS
            .iter()
            .map(|info| (info.str.to_string(), info.description.to_string()))
            .collect()
    }

    fn env_var_name() -> &'static str {
        ANDROID_ARCHS_ENV
    }
}

pub struct WebArch;

impl Arch for WebArch {
    fn platform() -> Box<dyn Platform> {
        Box::new(WebPlatform)
    }

    fn as_str(&self) -> &'static str {
        "wasm32-unknown-unknown"
    }

    fn parse_from_str<S: AsRef<str>>(_s: S) -> Self {
        WebArch
    }

    fn all_strings() -> Vec<&'static str> {
        vec!["wasm32-unknown-unknown"]
    }

    fn all_display_strings() -> Vec<(String, String)> {
        vec![(
            "wasm32-unknown-unknown".to_string(),
            "WebAssembly".to_string(),
        )]
    }

    fn env_var_name() -> &'static str {
        "WEB_ARCHS"
    }
}

//
// Platform Section
//

pub trait Platform {
    fn identifier() -> &'static str
    where
        Self: Sized;
}

pub trait PlatformBuilder: Platform {
    type Arch: Arch;
    type Params: Default;

    fn build(
        mode: Mode,
        project_dir: &std::path::Path,
        target_arch: Vec<Self::Arch>,
        params: Self::Params,
    ) -> anyhow::Result<std::path::PathBuf>;
}

pub struct IosPlatform;

impl Platform for IosPlatform {
    fn identifier() -> &'static str {
        "iOS Bindings Builder"
    }
}

pub struct AndroidPlatform;

impl Platform for AndroidPlatform {
    fn identifier() -> &'static str {
        "Android Bindings Builder"
    }
}

pub struct WebPlatform;

impl Platform for WebPlatform {
    fn identifier() -> &'static str {
        "Web Bindings Builder"
    }
}
