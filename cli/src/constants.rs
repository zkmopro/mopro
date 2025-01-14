pub const MODES: [&str; 2] = ["debug", "release"];

//
// Architeture Section
//

// Note that *_ARCH should align with `ios.rs` and `andriod.rs` in "mopro-ffi/src/app_config"
pub const IOS_ARCHS: [&str; 3] = [
    "aarch64-apple-ios",
    "aarch64-apple-ios-sim",
    "x86_64-apple-ios",
];

#[derive(Debug, Copy, Clone, PartialEq, Eq)]

pub enum IosArch {
    Aarch64Apple,
    Aarch64AppleSim,
    X8664Apple,
}

impl From<usize> for IosArch {
    fn from(idx: usize) -> Self {
        IOS_ARCHS[idx].into()
    }
}

impl From<&str> for IosArch {
    fn from(platform: &str) -> Self {
        match platform.to_lowercase().as_str() {
            "aarch64-apple-ios" => IosArch::Aarch64Apple,
            "aarch64-apple-ios-sim" => IosArch::Aarch64AppleSim,
            "x86_64-apple-ios" => IosArch::X8664Apple,
            _ => panic!("Unknown ios arch selected."),
        }
    }
}
impl From<IosArch> for &str {
    fn from(arch: IosArch) -> Self {
        match arch {
            IosArch::Aarch64Apple => "aarch64-apple-ios",
            IosArch::Aarch64AppleSim => "aarch64-apple-ios-sim",
            IosArch::X8664Apple => "x86_64-apple-ios",
        }
    }
}

pub const ANDROID_ARCHS: [&str; 4] = [
    "x86_64-linux-android",
    "i686-linux-android",
    "armv7-linux-androideabi",
    "aarch64-linux-android",
];

#[derive(Debug, Copy, Clone, PartialEq, Eq)]

pub enum AndroidArch {
    X8664Linux,
    I686Linux,
    Armv7LinuxAbi,
    Aarch64Linux,
}

impl From<usize> for AndroidArch {
    fn from(idx: usize) -> Self {
        ANDROID_ARCHS[idx].into()
    }
}

impl From<&str> for AndroidArch {
    fn from(platform: &str) -> Self {
        match platform.to_lowercase().as_str() {
            "x86_64-linux-android" => AndroidArch::X8664Linux,
            "i686-linux-android" => AndroidArch::I686Linux,
            "armv7-linux-androideabi" => AndroidArch::Armv7LinuxAbi,
            "aarch64-linux-android" => AndroidArch::Aarch64Linux,
            _ => panic!("Unknown android arch selected."),
        }
    }
}
impl From<AndroidArch> for &str {
    fn from(arch: AndroidArch) -> Self {
        match arch {
            AndroidArch::X8664Linux => "x86_64-linux-android",
            AndroidArch::I686Linux => "i686-linux-android",
            AndroidArch::Armv7LinuxAbi => "armv7-linux-androideabi",
            AndroidArch::Aarch64Linux => "aarch64-linux-android",
        }
    }
}

//
// Platform Section
//

pub const PLATFORMS: [&str; 3] = ["ios", "android", "web"];
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Platform {
    Ios,
    Android,
    Web,
}

impl Platform {
    pub fn as_usize(&self) -> usize {
        *self as usize
    }

    pub fn as_str(&self) -> &str {
        (*self).into()
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
}

impl From<usize> for Platform {
    fn from(idx: usize) -> Self {
        PLATFORMS[idx].into()
    }
}

impl From<&str> for Platform {
    fn from(platform: &str) -> Self {
        match platform.to_lowercase().as_str() {
            "ios" => Platform::Ios,
            "android" => Platform::Android,
            "web" => Platform::Web,
            _ => panic!("Unknown platform selected."),
        }
    }
}

impl From<Platform> for &str {
    fn from(platform: Platform) -> Self {
        match platform {
            Platform::Ios => "ios",
            Platform::Android => "android",
            Platform::Web => "web",
        }
    }
}

//
// Adapter Section
//

pub const ADAPTERS: [&str; 2] = ["circom", "halo2"];
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Adapter {
    Circom,
    Halo2,
}

impl Adapter {
    pub fn as_usize(&self) -> usize {
        *self as usize
    }

    pub fn as_str(&self) -> &str {
        (*self).into()
    }
}

impl From<&str> for Adapter {
    fn from(adapter: &str) -> Self {
        match adapter.to_lowercase().as_str() {
            "circom" => Adapter::Circom,
            "halo2" => Adapter::Halo2,
            _ => panic!("Unknown adapter selected."),
        }
    }
}

impl From<Adapter> for &str {
    fn from(adapter: Adapter) -> Self {
        match adapter {
            Adapter::Circom => "circom",
            Adapter::Halo2 => "halo2",
        }
    }
}

impl From<usize> for Adapter {
    fn from(idx: usize) -> Self {
        ADAPTERS[idx].into()
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

pub const FRAMEWORKS: [Framework; 5] = [
    Framework::Ios,
    Framework::Android,
    Framework::Web,
    Framework::Flutter,
    Framework::ReactNative,
];

impl Framework {
    pub fn as_str(&self) -> &str {
        (*self).into()
    }
}

impl From<String> for Framework {
    fn from(app: String) -> Self {
        match app.to_lowercase().as_str() {
            "ios" => Framework::Ios,
            "android" => Framework::Android,
            "web" => Framework::Web,
            "flutter" => Framework::Flutter,
            "react-native" => Framework::ReactNative,
            _ => panic!("Unknown platform selected."),
        }
    }
}

impl From<Framework> for &str {
    fn from(framework: Framework) -> Self {
        match framework {
            Framework::Ios => "ios",
            Framework::Android => "android",
            Framework::Web => "web",
            Framework::Flutter => "flutter",
            Framework::ReactNative => "react-native",
        }
    }
}

impl From<Framework> for String {
    fn from(framework: Framework) -> Self {
        let str: &str = framework.into();
        str.into()
    }
}
