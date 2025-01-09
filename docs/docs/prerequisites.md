---
sidebar_position: 2
---

# Prerequisites

Depending on what platforms and adapters you use, there are several prerequisites to install before getting started.

-   General
    -   [Rust](https://www.rust-lang.org/learn/get-started)
    -   [Cmake](https://cmake.org/download/)
-   iOS
    -   [Xcode](https://developer.apple.com/xcode/)
-   Android
    -   [Android Studio](https://developer.android.com/studio)
    -   Also see [configuration](#android-configuration) below
    -   [JDK(Java Development Kit)](https://www.oracle.com/java/technologies/downloads)
-   Web(WASM)
    -   [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
    -   [Download Chrome/ChromeDriver](https://googlechromelabs.github.io/chrome-for-testing/)
-   Circom
    -   Pre-built `zkey` and `wasm` files for your circuits
-   Halo2
    -   Pre-generated SRS (Structured Reference String) file, typically used as the universal setup for your circuits

## iOS configuration

Ensure that the command-line tools path is correctly set in Xcode. You can check this by navigating to Xcode > Settings > Locations.

![xcode - settings - location](/img/xcode-setting.png)

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

3. Set it to your `NDK_PATH` environment variable

```sh
NDK_PATH=$ANDROID_HOME/ndk/26.1.10909125
```

> Reference: [Running Rust on Android with UniFFI](https://sal.dev/android/intro-rust-android-uniffi/).
