use mopro_ffi::app_config::constants::{ANDROID_BINDINGS_DIR, IOS_BINDINGS_DIR};

//
// Platform Section
//
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
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

    pub fn parse_from_str(s: &str) -> Option<Self> {
        PLATFORMS
            .iter()
            .find(|info| info.str.to_lowercase() == s.to_lowercase())
            .map(|info| info.platform)
    }

    pub fn all_strings() -> Vec<&'static str> {
        PLATFORMS.iter().map(|info| info.str).collect()
    }

    pub fn from_idx(idx: usize) -> Self {
        PLATFORMS[idx].platform
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
            Self::Ios => IOS_BINDINGS_DIR,
            Self::Android => ANDROID_BINDINGS_DIR,
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
