fn main() {
    let available_archs = vec![
        "x86_64-linux-android",
        "i686-linux-android",
        "armv7-linux-androideabi",
        "aarch64-linux-android",
    ];

    let android_archs: Vec<String> = if let Ok(android_archs) = std::env::var("ANDROID_ARCHS") {
        android_archs.split(',').map(|arch| arch.to_string()).collect()
    } else {
        // Default case: select all supported architectures if none are provided
        available_archs.iter().map(|arch| arch.to_string()).collect()
    };

    // Check 'ANDRIOD_ARCH' input validation
    for arch in &android_archs {
        assert!(
            available_archs.contains(&arch.as_str()),
            "Unsupported architecture: {}",
            arch
        );
    }

    // A simple wrapper around a build command provided by mopro.
    // In the future this will likely be published in the mopro crate itself.
    mopro_ffi::app_config::android::build(&android_archs);
}
