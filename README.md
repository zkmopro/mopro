# mopro

Mopro is a toolkit for ZK app development.

Making client-side proving on mobile simple.

## Getting started

See [mopro-cli](https://github.com/oskarth/mopro/tree/main/mopro-cli#mopro-cli) for how to get started.

## Overview

Set of libraries and utilities:

- `mopro-cli` - core Rust CLI util.
- `mopro-core` - core mobile Rust library.
- `mopro-ffi` - wraps `mopro-core` and exposes UniFFI bindings.
- `mopro-ios` - iOS CocoaPod library exposing native Swift bindings.
- `mopro-android` - Android library exposing native Kotlin bindings.
- `mopro-example-app` - example iOS app using `mopro-ios`.
- `ark-zkey` - helper utility to make zkey more usable and faster in arkworks.

## Architecture

The following illustration shows how mopro and its components fit together into the wider ZKP ecosystem:

![mopro architecture](images/mopro_architecture2.png)

## Prerequisites

Depending on what platforms and adapters you use, there are several prerequisites to install before getting started.

### Circom

Install:
- [circom](https://docs.circom.io/)
- [snarkjs](https://github.com/iden3/snarkjs)

### iOS

Install:
- [Xcode](https://developer.apple.com/xcode/)
- [CocoaPods](https://cocoapods.org/)

### Android

Install:
- [Android Studio](https://developer.android.com/studio)
- In Android Studio, go to `SDK Manager > SDK Tools`  and install `NDK (Side by Side)` (see [Android Developer site](https://developer.android.com/studio/projects/install-ndk#default-version))

Configure environment variables:
- Export `$ANDROID_HOME` and change `{USER_NAME}` to your username
    ```sh
    export ANDROID_HOME="/Users/{USER_NAME}/Library/Android/sdk"
    ```
-  Locate which NDK version you have by
    ```sh
    ls $ANDROID_HOME/ndk
    # 26.1.10909125
    ```
    and set it to your `NDK_PATH` environment variable. e.g.
    ```sh
    NDK_PATH=$ANDROID_HOME/ndk/26.1.10909125
    ```

Reference: [Running Rust on Android with UniFFI](https://sal.dev/android/intro-rust-android-uniffi/)

### Configuration

This config file is best used together with `mopro-cli`.

By creating a `toml` configuration file you can specify what build settings you want to use. Example is provided in `config-example.toml`:

```
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
```

## Community and Talks

Join the Telegram group [here](https://t.me/zkmopro).

Talk by @oskarth at ProgCrypto/Devconnect (Istanbul, November 2023): [Slides](https://docs.google.com/presentation/d/1afIEgm8oYRvteWxUd04CcMOxChAiHaD55d5AKd0RkvY/edit#slide=id.g284ac8f47d5_2_24) (no video)

Talk by @oskarth at ETHTaipei (Taipei, March 2024): [Slides](https://hackmd.io/@oskarth/S1yGjF8C6#), [Video](https://www.youtube.com/live/JB6zP9enkbc?si=04xz9XRLkChNiupw&t=14708)

## Contribute

Contributions of all kinds welcome! Please see open GH issues. Also feel free to join the Telegram chat.

## Performance

Preliminary benchmarks on an iPhone 14 Max Pro:

- Keccak256 (150k constraints): 1.5s
    - ~x10-20 faster vs comparable circuit in browser
- anon-aadhaar / RSA Verify: ~6.5s
    - ~5s for witness generation (still in WASM), ~2s prover time
    - 80% of time on witness generation
    - ~x10 faster vs browser on phone
- Bottlenecks: loading zkey and wasm witness generation

See https://hackmd.io/5ItB2D50QcavF18cWIrmfQ?view= for more benchmarks.

## Acknowledgements

This work is sponsored by a joint grant from [PSE](https://pse.dev/) and [0xPARC](https://0xparc.org/).
