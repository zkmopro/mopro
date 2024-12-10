fn main() {
    let available_archs = vec!["aarch64-apple-ios", "aarch64-apple-ios-sim", "x86_64-apple-ios"];
    
    let ios_archs: Vec<String> = if let Ok(ios_archs) = std::env::var("IOS_ARCHS") {
        ios_archs.split(',').map(|arch| arch.to_string()).collect()
    } else {
        // Default case: select all supported architectures if none are provided
        available_archs.iter().map(|&arch| arch.to_string()).collect()
    };

    // Check 'IOS_ARCH' input validation
    for arch in &ios_archs {
        assert!(
            available_archs.contains(&arch.as_str()),
            "Unsupported architecture: {}",
            arch
        );
    
    }

    // A simple wrapper around a build command provided by mopro.
    // In the future this will likely be published in the mopro crate itself.
    mopro_ffi::app_config::ios::build(&ios_archs);
}
