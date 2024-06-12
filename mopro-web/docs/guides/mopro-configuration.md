---
sidebar_position: 4
---

# Mopro Configuration

This config file is best used together with `mopro-cli`.

By creating a `toml` configuration file you can specify what build settings you want to use. Example is provided in `config-example.toml`:

```toml
# config-example.toml

[build]
# For iOS device_type can be x86_64, simulator, device
ios_device_type = "simulator" # Options: x86_64, simulator, device
# For Android device_type can be x86_64, x86, arm, arm64
android_device_type = "arm64" # Options: x86_64, x86, arm, arm64

# debug is for Rust library to be in debug mode and release for release mode
# We recommend release mode by default for performance
build_mode = "release"    # Options: debug, release

[circuit]
dir = "examples/circom/keccak256" # Directory of the circuit
name = "keccak256_256_test"       # Name of the circuit

[dylib]
use_dylib = false         # Options: true, false
name = "keccak256.dylib" # Name of the dylib file, only used if use_dylib is true

# Note: circom-witness-rs is experimental
# See https://github.com/zkmopro/mopro/issues/32 for updates
# Only works for keccak256_256_test circuit now
[witness]
use_native_witness_generation = false       # Options: true, false
```