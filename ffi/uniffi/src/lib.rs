pub mod android;
pub mod ios;

pub use android::{AndroidBindingsParams, AndroidPlatform, ArchBuildConfig};
pub use ios::{IosPlatform, IosBindingsParams};

pub use mopro_build_common::{
    build_from_env, build_from_str_arch,
    AndroidArch, IosArch,
    Mode,
};
