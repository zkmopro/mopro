# mopro-ffi

`mopro-ffi` contains the logic for building a static library that can be invoked from a mobile app. This includes a macro for configuring `uniffi` as well as build commands for packaging an `xcframework` (for iOS) and `jniLibs` (for Android). 

## Development

### Prerequisites

1. Ensure you have Rust installed
2. If you're on OSX ensure the developer tools are installed

### Building

Run `cargo build` to build the library and `cargo test` to run the unit tests.
