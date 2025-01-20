---
sidebar_position: 2
---

# Prerequisites

Depending on what platforms and adapters you use, there are several prerequisites to install before getting started.

- General
  - [Rust](https://www.rust-lang.org/learn/get-started)
  - [Cmake](https://cmake.org/download/)
- iOS
  - [Xcode](https://developer.apple.com/xcode/)
- Android
  - [Android Studio](https://developer.android.com/studio)
  - Also see [configuration](#android-configuration) below
- Web(WASM)
  - [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
  - [Download Chrome/ChromeDriver](https://googlechromelabs.github.io/chrome-for-testing/)
- Circom
  - Pre-built `zkey` and `wasm` files for your circuits
- Halo2
  - Pre-generated SRS (Structured Reference String) file, typically used as the universal setup for your circuits

## Android configuration

Some additional configuration is required for Android.

First, install the latest SDK. In Android Studio, go to `SDK Manager > SDK Tools` and install `NDK (Side by Side)` (see [Android Developer site](https://developer.android.com/studio/projects/install-ndk#default-version)).

After that, set the following environment variables:

1. Export `$ANDROID_HOME` and change `{USER_NAME}` to your username

```sh
export ANDROID_HOME="/Users/{USER_NAME}/Library/Android/sdk"
```

2. Locate which NDK version you have

```sh
ls $ANDROID_HOME/ndk # => 26.1.10909125
```

3. Set it to your `ANDROID_NDK` environment variable

```sh
ANDROID_NDK=$ANDROID_HOME/ndk/26.1.10909125
```

4. Install `nasm` for your platform

- On macOS, you can install it using Homebrew:

  ```sh
  brew install nasm
  ```

- On a unix-based system, first check if `nasm` is already installed:

  ```sh
  whereis nasm
  ```

  If you see a path, then it is already installed. If you only see `nasm:`, then you need to install it. Download and install the latest version from the [official website](https://www.nasm.us/pub/nasm/releasebuilds/?C=M;O=D)

- On Windows, download the latest version from the [official website](https://www.nasm.us/pub/nasm/releasebuilds/?C=M;O=D) and add it to your PATH.
