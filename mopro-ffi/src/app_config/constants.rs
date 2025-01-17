pub const ARCH_X86_64: &str = "x86_64";
pub const ARCH_ARM_64: &str = "aarch64";
pub const ARCH_I686: &str = "x86";
pub const ARCH_ARM_V7_ABI: &str = "armeabi-v7a";
pub const ARCH_ARM_64_V8: &str = "arm64-v8a";

pub const ENV_CONFIG: &str = "CONFIGURATION";
pub const ENV_ANDROID_ARCHS: &str = "ANDROID_ARCHS";
pub const ENV_IOS_ARCHS: &str = "IOS_ARCHS";

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

    pub fn all_strings() -> Vec<&'static str> {
        MODES.iter().map(|info| info.str).collect()
    }
}

//
// Architeture Section
//
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum IosArch {
    Aarch64Apple,
    Aarch64AppleSim,
    X8664Apple,
}

struct IosArchInfo {
    arch: IosArch,
    str: &'static str,
}

// Architecture strings need to be aligned with those in the CLI.
const IOS_ARCHS: [IosArchInfo; 3] = [
    IosArchInfo {
        arch: IosArch::Aarch64Apple,
        str: "aarch64-apple-ios",
    },
    IosArchInfo {
        arch: IosArch::Aarch64AppleSim,
        str: "aarch64-apple-ios-sim",
    },
    IosArchInfo {
        arch: IosArch::X8664Apple,
        str: "x86_64-apple-ios",
    },
];

impl IosArch {
    pub fn as_str(&self) -> &'static str {
        IOS_ARCHS
            .iter()
            .find(|info| info.arch == *self)
            .map(|info| info.str)
            .expect("Unsupported iOS Arch")
    }

    pub fn parse_from_str(s: &str) -> Self {
        IOS_ARCHS
            .iter()
            .find(|info| info.str.to_lowercase() == s.to_lowercase())
            .map(|info| info.arch)
            .expect("Unsupported iOS String")
    }

    pub fn all_strings() -> Vec<&'static str> {
        IOS_ARCHS.iter().map(|info| info.str).collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AndroidArch {
    X8664Linux,
    I686Linux,
    Armv7LinuxAbi,
    Aarch64Linux,
}

struct AndriodArchInfo {
    arch: AndroidArch,
    str: &'static str,
}

// Architecture strings need to be aligned with those in the CLI.
const ANDROID_ARCHS: [AndriodArchInfo; 4] = [
    AndriodArchInfo {
        arch: AndroidArch::X8664Linux,
        str: "x86_64-linux-android",
    },
    AndriodArchInfo {
        arch: AndroidArch::I686Linux,
        str: "i686-linux-android",
    },
    AndriodArchInfo {
        arch: AndroidArch::Armv7LinuxAbi,
        str: "armv7-linux-androideabi",
    },
    AndriodArchInfo {
        arch: AndroidArch::Aarch64Linux,
        str: "aarch64-linux-android",
    },
];

impl AndroidArch {
    pub fn as_str(&self) -> &'static str {
        ANDROID_ARCHS
            .iter()
            .find(|info| info.arch == *self)
            .map(|info| info.str)
            .expect("Unsupported Android Arch")
    }

    pub fn parse_from_str(s: &str) -> Self {
        ANDROID_ARCHS
            .iter()
            .find(|info| info.str.to_lowercase() == s.to_lowercase())
            .map(|info| info.arch)
            .expect("Unsupported Android String")
    }

    pub fn all_strings() -> Vec<&'static str> {
        ANDROID_ARCHS.iter().map(|info| info.str).collect()
    }
}
