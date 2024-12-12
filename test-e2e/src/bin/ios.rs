use mopro_ffi::app_config::ios::IOS_ARCHS;

fn main() {
    let ios_archs: Vec<String> = if let Ok(ios_archs) = std::env::var("IOS_ARCHS") {
        ios_archs.split(',').map(|arch| arch.to_string()).collect()
    } else {
        // Default case: select all supported architectures if none are provided
        IOS_ARCHS.iter().map(|&arch| arch.to_string()).collect()
    };

    // Check 'IOS_ARCH' input validation
    for arch in &ios_archs {
        assert!(
            IOS_ARCHS.contains(&arch.as_str()),
            "Unsupported architecture: {}",
            arch
        );
    }

    // A simple wrapper around a build command provided by mopro.
    // In the future this will likely be published in the mopro crate itself.
    mopro_ffi::app_config::ios::build(&ios_archs);
}
