# Getting started

This tutorial walks you through building a static library from scratch using the **Circom**, **Halo2**, or **Noir** adapter, and demonstrates how to integrate it with Android, iOS, and Web platforms. It also shows how to create example templates for mobile development.

If you already have an **existing Rust project** and want to generate bindings for a mobile native SDK or to build a mobile native app, check out the [Rust Setup](setup/rust-setup.md) guide for detailed instructions.

If you already have an **existing mobile frontend**, you only need to follow Steps [#0](#0-prerequisites) - [#3](#3-build-bindings) to generate the bindings, then proceed to the relevant integration guide below:

-   [iOS Setup](setup/ios-setup.md)
-   [Android Setup](setup/android-setup.md)
-   [React Native Setup](setup/react-native-setup.md)
-   [Flutter Setup](setup/flutter-setup.md)

## 0. Prerequisites

Make sure you've installed the [prerequisites](/docs/prerequisites).

## 1. Install CLI

Clone the `mopro` repository and install the `mopro` CLI tool.

```sh
git clone https://github.com/zkmopro/mopro
cd mopro/cli
cargo install --path .
cd ../..
```

You can run

```sh
mopro --help
```

to see a full list of commands and their usage details with, for example,

```sh
mopro init --help
```

## 2. Initialize adapters

Navigate to the folder where you want to build the app. Select the adapters using the `mopro` CLI.

```sh
mopro init
```

## 3. Build bindings

Navigate to your project directory. (e.g. `cd mopro-example-app`) <br/>
Build bindings for specific targets (iOS, Android, Web).

```sh
mopro build
```

:::warning
The process of building bindings may take a few minutes.
:::

:::info
Running your project in `release` mode significantly enhances performance compared to `debug` mode. This is because the Rust compiler applies optimizations that improve runtime speed and reduce binary size, making your application more efficient.

:::

## 4. Create templates

Create templates for developing your mobile app.

```sh
mopro create
```

Follow the instructions to open the development tools

### For iOS

```sh
open ios/MoproApp.xcodeproj
```

### For Android

```sh
open android -a Android\ Studio
```

### For Web

```sh
cd web && yarn && yarn start
```

### For React Native

```sh
cd react-native && npm install
```

:::note
Setup the `ANDROID_HOME` environment for Android

```sh
export ANDROID_HOME=~/Library/Android/sdk
```

:::

```sh
npm run ios # for iOS simulator
npm run ios:device # for iOS device
npm run android # for Android emulator/devices
```

:::info
See more details in [react-native-app](https://github.com/zkmopro/react-native-app)
:::

### For Flutter

```sh
flutter doctor
```

```sh
flutter pub get
```

Connect devices or turn on simulators before running

```sh
flutter run
```

:::info
See more details in [flutter-app](https://github.com/zkmopro/flutter-app)
:::

## 5. Update bindings

If you make changes to `src/lib.rs`—such as adding functions with `#[uniffi::export]`—and want to update the generated bindings across all platforms, simply run:

```sh
mopro build
mopro update
```

This will automatically detect and update the corresponding bindings in each platform template you've set up.

## 6. What's next

-   **Update your ZK circuits** as needed. After making changes, be sure to run:

    ```sh
    mopro build
    mopro update
    ```

    This ensures the bindings are regenerated and reflect your latest updates.

-   **Build your mobile app frontend** according to your business logic and user flow.
-   **Expose additional Rust functionality:**
    If a function is missing in Swift, Kotlin, React Native, or Flutter, you can:

    -   Add the required Rust crate in `Cargo.toml`
    -   Annotate your function with `#[uniffi::export]` (See the [Rust setup](setup/rust-setup.md#setup-any-rust-project) guide for details).<br/>
        Once exported, the function will be available across all supported platforms.
        :::warning
        When using React Native or Flutter, don’t forget to update the module’s API definitions to ensure the framework can access the new Swift/Kotlin bindings.<br/>
        See more details in [react-native-app](https://github.com/zkmopro/react-native-app) or [flutter-app](https://github.com/zkmopro/flutter-app)
        :::
