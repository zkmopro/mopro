pub const XCFRAMEWORK_NAME: &str = "MoproBindings.xcframework";
pub const MOPRO_SWIFT_FILE: &str = "mopro.swift";
pub const JNILIBS_DIR: &str = "jniLibs";
pub const MOPRO_KOTLIN_FILE: &str = "mopro.kt";

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

// Architecture strings need to be aligned with those in the mopro-ffi.
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

impl IosArch {
    pub fn all_strings() -> Vec<&'static str> {
        IOS_ARCHS.iter().map(|info| info.str).collect()
    }

    pub fn all_display_strings() -> Vec<(String, String)> {
        IOS_ARCHS
            .iter()
            .map(|info| (info.str.to_string(), info.description.to_string()))
            .collect()
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

// Architecture strings need to be aligned with those in the mopro-ffi.
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

impl AndroidArch {
    pub fn as_str(&self) -> &'static str {
        ANDROID_ARCHS
            .iter()
            .find(|info| info.arch == *self)
            .map(|info| info.str)
            .expect("Unsupported Android Arch")
    }

    pub fn all_strings() -> Vec<&'static str> {
        ANDROID_ARCHS.iter().map(|info| info.str).collect()
    }

    pub fn all_display_strings() -> Vec<(String, String)> {
        ANDROID_ARCHS
            .iter()
            .map(|info| (info.str.to_string(), info.description.to_string()))
            .collect()
    }
}

//
// Platform Section
//
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Platform {
    Ios,
    Android,
    Web,
}

struct PlatformInfo {
    platform: Platform,
    str: &'static str,
}

const PLATFORMS: [PlatformInfo; 3] = [
    PlatformInfo {
        platform: Platform::Ios,
        str: "ios",
    },
    PlatformInfo {
        platform: Platform::Android,
        str: "android",
    },
    PlatformInfo {
        platform: Platform::Web,
        str: "web",
    },
];

impl Platform {
    pub fn as_str(&self) -> &'static str {
        PLATFORMS
            .iter()
            .find(|info| info.platform == *self)
            .map(|info| info.str)
            .expect("Unsupported Platform")
    }

    pub fn parse_from_str(s: &str) -> Self {
        PLATFORMS
            .iter()
            .find(|info| info.str.to_lowercase() == s.to_lowercase())
            .map(|info| info.platform)
            .expect("Unsupported Platform String")
    }

    pub fn all_strings() -> Vec<&'static str> {
        PLATFORMS.iter().map(|info| info.str).collect()
    }

    pub fn from_idx(idx: usize) -> Self {
        PLATFORMS[idx].platform
    }

    pub fn arch_key(&self) -> &str {
        match self {
            Self::Ios => "IOS_ARCHS",
            Self::Android => "ANDROID_ARCHS",
            Self::Web => "",
        }
    }

    pub fn binding_name(&self) -> &str {
        match self {
            Self::Ios => "iOS",
            Self::Android => "Android",
            Self::Web => "WASM",
        }
    }

    pub fn binding_dir(&self) -> &str {
        match self {
            Self::Ios => "MoproiOSBindings",
            Self::Android => "MoproAndroidBindings",
            Self::Web => "MoproWasmBindings",
        }
    }
}

//
// Framework Section
//
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Framework {
    Ios,
    Android,
    Web,
    Flutter,
    ReactNative,
}

struct FrameworkInfo {
    framework: Framework,
    str: &'static str,
}

const FRAMEWORKS_INFO: [FrameworkInfo; 5] = [
    FrameworkInfo {
        framework: Framework::Ios,
        str: "ios",
    },
    FrameworkInfo {
        framework: Framework::Android,
        str: "android",
    },
    FrameworkInfo {
        framework: Framework::Web,
        str: "web",
    },
    FrameworkInfo {
        framework: Framework::Flutter,
        str: "flutter",
    },
    FrameworkInfo {
        framework: Framework::ReactNative,
        str: "react-native",
    },
];

impl Framework {
    pub fn as_str(&self) -> &'static str {
        FRAMEWORKS_INFO
            .iter()
            .find(|info| info.framework == *self)
            .map(|info| info.str)
            .expect("Unsupported Framework")
    }

    pub fn parse_from_str(s: &str) -> Self {
        FRAMEWORKS_INFO
            .iter()
            .find(|info| info.str.to_lowercase() == s.to_lowercase())
            .map(|info| info.framework)
            .expect("Unsupported Framework String")
    }

    pub fn from_idx(idx: usize) -> Self {
        FRAMEWORKS_INFO[idx].framework
    }

    pub fn all_strings() -> Vec<&'static str> {
        FRAMEWORKS_INFO.iter().map(|info| info.str).collect()
    }

    pub fn contains(framework: &str) -> bool {
        FRAMEWORKS_INFO
            .iter()
            .any(|f| f.str.to_lowercase() == framework.to_lowercase())
    }
}
