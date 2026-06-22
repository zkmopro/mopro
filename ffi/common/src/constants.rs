use color_eyre::eyre::ContextCompat;

pub const BUILD_MODE_ENV: &str = "CONFIGURATION";
pub const IOS_ARCHS_ENV: &str = "IOS_ARCHS";
pub const ANDROID_ARCHS_ENV: &str = "ANDROID_ARCHS";
pub const FLUTTER_ARCHS_ENV: &str = "FLUTTER_ARCHS";
pub const REACT_NATIVE_ARCHS_ENV: &str = "REACT_NATIVE_ARCHS";

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

pub const FLUTTER_BINDINGS_DIR: &str = "mopro_flutter_bindings";
pub const REACT_NATIVE_BINDINGS_DIR: &str = "MoproReactNativeBindings";

// ---------------------------------------------------------------------------
// Mode
// ---------------------------------------------------------------------------

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Mode {
    Debug,
    Release,
}

struct ModeInfo { mode: Mode, str: &'static str }
const MODES: [ModeInfo; 2] = [
    ModeInfo { mode: Mode::Debug,   str: "debug"   },
    ModeInfo { mode: Mode::Release, str: "release" },
];

impl Mode {
    pub fn as_str(&self) -> &'static str {
        MODES.iter().find(|i| i.mode == *self).map(|i| i.str)
            .expect("Unsupported Mode")
    }
    pub fn parse_from_str(s: &str) -> Self {
        MODES.iter().find(|i| i.str.to_lowercase() == s.to_lowercase())
            .map(|i| i.mode)
            .expect("Unsupported Mode string — only 'release' and 'debug'")
    }
    pub fn from_idx(idx: usize) -> Self { MODES[idx].mode }
    pub fn idx(s: &str) -> Option<usize> {
        MODES.iter().enumerate().find(|(_, m)| m.str == s).map(|(i, _)| i)
    }
    pub fn all_strings() -> Vec<&'static str> { MODES.iter().map(|i| i.str).collect() }
}

// ---------------------------------------------------------------------------
// Arch trait + PlatformBuilder trait
//
// Platform structs (IosPlatform, WebPlatform, etc.) are NOT defined here —
// each ffi/backends/* crate defines its own so that `impl PlatformBuilder for
// <LocalStruct>` satisfies the Rust orphan rule.
// ---------------------------------------------------------------------------

pub trait Arch {
    fn as_str(&self) -> &'static str;
    fn parse_from_str<S: AsRef<str>>(s: S) -> Self;
    fn all_strings() -> Vec<&'static str>;
    fn all_display_strings() -> Vec<(String, String)>;
    fn env_var_name() -> &'static str;
}

pub trait PlatformBuilder {
    type Arch: Arch;
    type Params: Default;

    fn identifier() -> &'static str;
    fn build(
        mode: Mode,
        project_dir: &std::path::Path,
        target_arch: Vec<Self::Arch>,
        params: Self::Params,
    ) -> anyhow::Result<std::path::PathBuf>;
}

// ---------------------------------------------------------------------------
// iOS arch
// ---------------------------------------------------------------------------

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum IosArch { Aarch64Apple, Aarch64AppleSim, X8664Apple }

struct IosArchInfo { arch: IosArch, str: &'static str, description: &'static str }
const IOS_ARCHS: [IosArchInfo; 3] = [
    IosArchInfo { arch: IosArch::Aarch64Apple,    str: "aarch64-apple-ios",     description: "64-bit iOS devices" },
    IosArchInfo { arch: IosArch::Aarch64AppleSim, str: "aarch64-apple-ios-sim", description: "ARM64 iOS simulator" },
    IosArchInfo { arch: IosArch::X8664Apple,      str: "x86_64-apple-ios",      description: "x86_64 iOS simulator" },
];

impl Arch for IosArch {
    fn as_str(&self) -> &'static str {
        IOS_ARCHS.iter().find(|i| i.arch == *self).map(|i| i.str).expect("Unsupported iOS Arch")
    }
    fn parse_from_str<S: AsRef<str>>(s: S) -> Self {
        IOS_ARCHS.iter().find(|i| i.str.to_lowercase() == s.as_ref().to_lowercase())
            .map(|i| i.arch)
            .context(format!("Unsupported iOS Arch '{}'", s.as_ref())).unwrap()
    }
    fn all_strings() -> Vec<&'static str> { IOS_ARCHS.iter().map(|i| i.str).collect() }
    fn all_display_strings() -> Vec<(String, String)> {
        IOS_ARCHS.iter().map(|i| (i.str.to_string(), i.description.to_string())).collect()
    }
    fn env_var_name() -> &'static str { IOS_ARCHS_ENV }
}

// ---------------------------------------------------------------------------
// Android arch
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AndroidArch { X8664Linux, I686Linux, Armv7LinuxAbi, Aarch64Linux }

struct AndroidArchInfo { arch: AndroidArch, str: &'static str, description: &'static str }
const ANDROID_ARCHS: [AndroidArchInfo; 4] = [
    AndroidArchInfo { arch: AndroidArch::X8664Linux,    str: "x86_64-linux-android",   description: "64-bit Android emulators" },
    AndroidArchInfo { arch: AndroidArch::I686Linux,     str: "i686-linux-android",      description: "32-bit Android emulators" },
    AndroidArchInfo { arch: AndroidArch::Armv7LinuxAbi, str: "armv7-linux-androideabi", description: "32-bit ARM devices" },
    AndroidArchInfo { arch: AndroidArch::Aarch64Linux,  str: "aarch64-linux-android",   description: "64-bit ARM devices" },
];

impl Arch for AndroidArch {
    fn as_str(&self) -> &'static str {
        ANDROID_ARCHS.iter().find(|i| i.arch == *self).map(|i| i.str).expect("Unsupported Android Arch")
    }
    fn parse_from_str<S: AsRef<str>>(s: S) -> Self {
        ANDROID_ARCHS.iter().find(|i| i.str.to_lowercase() == s.as_ref().to_lowercase())
            .map(|i| i.arch)
            .context(format!("Unsupported Android Arch '{}'", s.as_ref())).unwrap()
    }
    fn all_strings() -> Vec<&'static str> { ANDROID_ARCHS.iter().map(|i| i.str).collect() }
    fn all_display_strings() -> Vec<(String, String)> {
        ANDROID_ARCHS.iter().map(|i| (i.str.to_string(), i.description.to_string())).collect()
    }
    fn env_var_name() -> &'static str { ANDROID_ARCHS_ENV }
}

// ---------------------------------------------------------------------------
// Web arch
// ---------------------------------------------------------------------------

pub struct WebArch;
impl Arch for WebArch {
    fn as_str(&self) -> &'static str { "wasm32-unknown-unknown" }
    fn parse_from_str<S: AsRef<str>>(_s: S) -> Self { WebArch }
    fn all_strings() -> Vec<&'static str> { vec!["wasm32-unknown-unknown"] }
    fn all_display_strings() -> Vec<(String, String)> {
        vec![("wasm32-unknown-unknown".to_string(), "WebAssembly".to_string())]
    }
    fn env_var_name() -> &'static str { "WEB_ARCHS" }
}

// ---------------------------------------------------------------------------
// Flutter arch
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlutterArch {
    Aarch64Apple, Aarch64AppleSim, X8664Apple,
    X8664Linux, I686Linux, Armv7LinuxAbi, Aarch64Linux,
}

struct FlutterArchInfo { arch: FlutterArch, str: &'static str, description: &'static str }
const FLUTTER_ARCHS: [FlutterArchInfo; 7] = [
    FlutterArchInfo { arch: FlutterArch::Aarch64Apple,    str: "aarch64-apple-ios",      description: "64-bit iOS devices" },
    FlutterArchInfo { arch: FlutterArch::Aarch64AppleSim, str: "aarch64-apple-ios-sim",  description: "ARM64 iOS simulator" },
    FlutterArchInfo { arch: FlutterArch::X8664Apple,      str: "x86_64-apple-ios",       description: "x86_64 iOS simulator" },
    FlutterArchInfo { arch: FlutterArch::X8664Linux,      str: "x86_64-linux-android",   description: "64-bit Android emulators" },
    FlutterArchInfo { arch: FlutterArch::I686Linux,       str: "i686-linux-android",     description: "32-bit Android emulators" },
    FlutterArchInfo { arch: FlutterArch::Armv7LinuxAbi,   str: "armv7-linux-androideabi",description: "32-bit ARM devices" },
    FlutterArchInfo { arch: FlutterArch::Aarch64Linux,    str: "aarch64-linux-android",  description: "64-bit ARM devices" },
];

impl Arch for FlutterArch {
    fn as_str(&self) -> &'static str {
        FLUTTER_ARCHS.iter().find(|i| i.arch == *self).map(|i| i.str).expect("Unsupported Flutter Arch")
    }
    fn parse_from_str<S: AsRef<str>>(s: S) -> Self {
        FLUTTER_ARCHS.iter().find(|i| i.str.to_lowercase() == s.as_ref().to_lowercase())
            .map(|i| i.arch)
            .context(format!("Unsupported Flutter Arch '{}'", s.as_ref())).unwrap()
    }
    fn all_strings() -> Vec<&'static str> { FLUTTER_ARCHS.iter().map(|i| i.str).collect() }
    fn all_display_strings() -> Vec<(String, String)> {
        FLUTTER_ARCHS.iter().map(|i| (i.str.to_string(), i.description.to_string())).collect()
    }
    fn env_var_name() -> &'static str { FLUTTER_ARCHS_ENV }
}

// ---------------------------------------------------------------------------
// React Native arch
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReactNativeArch {
    Aarch64Apple, Aarch64AppleSim, X8664Apple,
    X8664Linux, I686Linux, Armv7LinuxAbi, Aarch64Linux,
}

struct ReactNativeArchInfo { arch: ReactNativeArch, str: &'static str, description: &'static str }
const REACT_NATIVE_ARCHS: [ReactNativeArchInfo; 7] = [
    ReactNativeArchInfo { arch: ReactNativeArch::Aarch64Apple,    str: "aarch64-apple-ios",      description: "64-bit iOS devices" },
    ReactNativeArchInfo { arch: ReactNativeArch::Aarch64AppleSim, str: "aarch64-apple-ios-sim",  description: "ARM64 iOS simulator" },
    ReactNativeArchInfo { arch: ReactNativeArch::X8664Apple,      str: "x86_64-apple-ios",       description: "x86_64 iOS simulator" },
    ReactNativeArchInfo { arch: ReactNativeArch::X8664Linux,      str: "x86_64-linux-android",   description: "64-bit Android emulators" },
    ReactNativeArchInfo { arch: ReactNativeArch::I686Linux,       str: "i686-linux-android",     description: "32-bit Android emulators" },
    ReactNativeArchInfo { arch: ReactNativeArch::Armv7LinuxAbi,   str: "armv7-linux-androideabi",description: "32-bit ARM devices" },
    ReactNativeArchInfo { arch: ReactNativeArch::Aarch64Linux,    str: "aarch64-linux-android",  description: "64-bit ARM devices" },
];

impl Arch for ReactNativeArch {
    fn as_str(&self) -> &'static str {
        REACT_NATIVE_ARCHS.iter().find(|i| i.arch == *self).map(|i| i.str).expect("Unsupported RN Arch")
    }
    fn parse_from_str<S: AsRef<str>>(s: S) -> Self {
        REACT_NATIVE_ARCHS.iter().find(|i| i.str.to_lowercase() == s.as_ref().to_lowercase())
            .map(|i| i.arch)
            .context(format!("Unsupported React Native Arch '{}'", s.as_ref())).unwrap()
    }
    fn all_strings() -> Vec<&'static str> { REACT_NATIVE_ARCHS.iter().map(|i| i.str).collect() }
    fn all_display_strings() -> Vec<(String, String)> {
        REACT_NATIVE_ARCHS.iter().map(|i| (i.str.to_string(), i.description.to_string())).collect()
    }
    fn env_var_name() -> &'static str { REACT_NATIVE_ARCHS_ENV }
}
