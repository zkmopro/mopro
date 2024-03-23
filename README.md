# mopro

Mopro is a toolkit for ZK app development on mobile. Mopro makes client-side proving on mobile simple.

## Getting started

We recommend you use [mopro-cli](https://github.com/oskarth/mopro/tree/main/mopro-cli#mopro-cli) to create and maintain your application. Here's how you can get started with your example in a few minutes.

If you prefer to watch a demo, you can find a <5m tutorial [here](https://www.loom.com/share/6ff382b0497c47aea9d0ef8b6e790dd8).

### Install dependencies

- Install prerequisites listed [here](https://github.com/oskarth/mopro?tab=readme-ov-file#prerequisites).

- Install mopro-cli locally:
  ```sh
  cd mopro-cli && cargo install --path .
  ```
- Set `MOPRO_ROOT` (replace user with your username):
  ```sh
  export MOPRO_ROOT=/Users/user/repos/github.com/oskarth/mopro
  ```
- Install `mopro` dependencies:
  ```sh
  mopro deps
  ```

### Create a project

- Create a working directory:
 ```sh
 mkdir ~/my-zk-app && cd my-zk-app
 ```

- Initialize a project:
  ```sh
  mopro init --platforms ios, android
  ```
- Go to your project folder:
  ```
  cd mopro-example-app
  ```

### Configure mopro settings

- Adapt `mopro-config.toml` to your needs
- Prepare circuit artifacts:
  ```sh
  mopro prepare
  ```

### Build, test and run your  project

- Build the project:
  ```sh
  mopro build
  ```
- Run end-to-end-test (in Rust only):
  ```sh
  mopro test
  ```
- Build the project for iOS:
  ```sh
  mopro build --platforms ios
  open ios/ExampleApp/ExampleApp.xcworkspace
  ```
- Build the project for Android:
  ```sh
  mopro build --platforms android
  open android -a Android\ Studio
  ```

See [mopro-cli](https://github.com/oskarth/mopro/tree/main/mopro-cli#mopro-cli) for more details on usage.

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

- General: [Rust and Cargo](https://www.rust-lang.org/learn/get-started)
- Circom: [circom](https://docs.circom.io/)
- Circom: [snarkjs](https://github.com/iden3/snarkjs)
- iOS: [Xcode](https://developer.apple.com/xcode/)
- iOS: [CocoaPods](https://cocoapods.org/)
- Android: [Android Studio](https://developer.android.com/studio) (also see configuration below)

### Android configuration

Some additional configuration is required for Android.

Install latest SDK: 
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

### mopro configuration

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
