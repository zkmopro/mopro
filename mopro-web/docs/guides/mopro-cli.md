# mopro-cli

`mopro` is a CLI tool for client-side proving of Zero Knowledge Proofs. It simplifies the process of initializing, building, updating, and testing projects across different platforms and configurations.

Think of it as Foundry for client-side proving.

## Installation

To use `mopro`, you need to have Rust and Cargo installed on your system. You can install them from [the official Rust website](https://www.rust-lang.org/learn/get-started).

Run `cargo install --path .` to install the `mopro` CLI util.

## Usage

Here are the basic commands of `mopro`:

-   `mopro init`: Initialize a new project with support for multiple platforms.

    -   options:
        ```sh
            --adapter <ADAPTER>            [default: circom]
            --platforms <PLATFORMS>...     [default: core]
            --project-name <PROJECT_NAME>  [default: mopro-example-app]
        ```

-   `mopro deps`: Install required dependencies.
-   `mopro prepare`: Prepare and build circuit and its artifacts.
    -   options:
        ```sh
            --config <CONFIG>  [default: mopro-config.toml]
        ```
-   `mopro build`: Build the project for specified platforms.

    -   options:
        ```sh
            --config <CONFIG>           [default: mopro-config.toml]
            --adapter <ADAPTER>         [default: circom]
            --platforms <PLATFORMS>...  [default: core]
        ```

-   `mopro test`: Run tests for specific platform and test cases.
    -   options:
        ```sh
            --config <CONFIG>           [default: mopro-config.toml]
            --adapter <ADAPTER>         [default: circom]
            --platforms <PLATFORMS>...  [default: core]
            --test-case <TEST_CASE>
        ```
-   `mopro export-bindings`: Export platform bindings to some other directory.

    -   options:
        ```sh
            --platforms <PLATFORMS>...   [default: ios]
            -d, --destination <DESTINATION>
        ```

(May be added soon: `mopro update`: Update bindings with new API for specified platforms.)

## Prerequisites

To use `mopro-cli`, make sure you have installed the [prerequisites](/docs/prerequisites).

## Examples

### Basic example

Initialize, build and test a circuit with Rust bindings:

-   Set `MOPRO_ROOT`

    ```sh
    export MOPRO_ROOT=/Users/user/repos/github.com/zkmopro/mopro
    ```

-   Install dependencies

    ```sh
    mopro deps
    ```

-   Default to circom adapter and core Rust bindings

    ```sh
    mopro init
    ```

-   Go to the newly created directory

    ```sh
    cd mopro-example-app
    ```

-   Prepare circuit artifacts

    ```sh
    mopro prepare
    ```

-   Build the project

    ```sh
    mopro build
    ```

-   Run end-to-end-test

    ```sh
    mopro test
    ```

### iOS

Initialize and build an app with iOS support.

```sh
mopro init --platforms ios
cd mopro-example-app
mopro prepare
mopro build --platforms ios

# Open project in XCode
open ios/ExampleApp/ExampleApp.xcworkspace

# Currently testing only available for Rust bindings,
# Can run iOS tests from newly created Xcode project
mopro test
```

### Android

Initialize and build an app with Android support.

```sh
mopro init --platforms android
cd mopro-example-app
mopro prepare
mopro build --platforms android

# Open android project in Android Studio
open android -a Android\ Studio
```

### Web

Initialize and build a web app.

```sh
mopro init --platforms web
cd mopro-example-app
mopro prepare
mopro build --platforms web
```

Open web project directory and run frontend locally.

```sh
cd web
npm install
npm run dev
```

### Exporting bindings

To export bindings to a different directory:

`mopro export-bindings --destination <DESTINATION_DIR> --platforms <IOS_AND_OR_ANDROID>`

This will the following files, assuming they've been built, to the destination directory:

```
├── android
│   ├── jniLibs
│   │   └── arm64-v8a
│   │       └── libuniffi_mopro.so
│   └── uniffi
│       └── mopro
│           └── mopro.kt
└── ios
    ├── Bindings
    │   ├── module.modulemap
    │   ├── mopro.swift
    │   └── moproFFI.h
    └── aarch64-apple-ios-sim
        └── release
            └── libmopro_ffi.a
```

#### Use the bindings in iOS

-   Create a XCFramework with `xcodebuild`
    ```sh
    xcodebuild -create-xcframework \
    -library <DESTINATION_DIR>/ios/aarch64-apple-ios-sim/release/libmopro_ffi.a \
    -headers <DESTINATION_DIR>/ios/Bindings \
    -output "<DESTINATION_DIR>/ios/Mopro.xcframework"
    ```
-   Import both the XCFramework `Mopro.xcframework` and the Swift file bindings `Bindings/mopro.swift` files into your project (drag and drop should work).
-   Use moproFFI in swift like

    ```swift
    import moproFFI

    ...
    try initializeMopro()
    ...
    ```

> Reference: https://forgen.tech/en/blog/post/building-an-ios-app-with-rust-using-uniffi

#### Use the bindings in Android

-   Add dependency in `<ANDROID_APP_DIR>/app/build.gradle.kts`
    ```kts
     dependencies {
     ...
     implementation("net.java.dev.jna:jna:5.13.0@aar")
     ...
    }
    ```
-   Sync gradle
-   Move the `<DESTINATION_DIR>/android/jniLibs/` folder to `app/src/main/`
-   Move the `<DESTINATION_DIR>/android/uniffi/` folder to `app/src/main/java/`
-   Use moproFFI in kotlin like

    ```kotlin
      import uniffi.mopro.initializeMopro

      ...
      initializeMopro()
      ...
    ```
> Reference: https://sal.dev/android/intro-rust-android-uniffi/

## Contributing

Contributions to `mopro` are welcome. Please feel free to submit issues and pull requests.