# React Native Setup

This tutorial will guide you through integrating the iOS bindings and Android bindings into an [React Native](https://reactnative.dev/)) project. Before you begin, make sure you’ve completed the ["Getting Started - 3. Mopro build"](/docs/getting-started.md#3-build-bindings) process with selecting **react native** platform and have the `MoproReactNativeBindings` folder ready:

React Native is a _JavaScript_ framework that enables developers to build native apps for multiple platforms with a single codebase.

In this tutorial, you will learn how to create a native Mopro module on both Android and iOS simulators/devices. <br/>

<div style={{ display: 'flex', justifyContent: 'center', gap: '10px', margin: '10px' }}>
    <img src="/img/react-native-android.png" alt="First Image" width="250"/>
    <img src="/img/react-native-ios.png" alt="Second Image" width="250"/>
</div>

:::info
In this example, we use Circom circuits and their corresponding `.zkey` files. The process is similar for other provers.
:::

## 0. Initialize an React Native project

First let's create a new React Native project. If you already have a React Native project you can skip this step.

-   Getting started with React Native: [Official documentation](https://reactnative.dev/docs/turbo-native-modules-introduction)
    For example,

    ```sh
    npx @react-native-community/cli@latest init TurboModuleExample --version 0.82
    ```

    :::info
    A **Turbo Module** is required for React Native bindings. It enables communication with native platform APIs that are not available through React Native’s core modules or third-party libraries.
    :::

-   After creating a React Native project, you should be able to run with a commands like within the projects

    ```bash
    npm run start
    ```

    to starg a server.

    ```bash
    npm run ios
    ```

    for iOS simulators. And

    ```bash
    npm run android
    ```

    for Android emulators.

## 1. Copy React Native Module

-   Copy the exported `MoproReactNativeBindings` folder into the project directory you created with the CLI.
    After doing so, your project structure should look like this:

        ```sh
        .
        ├── __tests__
        ├── android
        ├── app.json
        ├── App.tsx
        ├── ios
        ├── MoproReactNativeBindings
        ├── package.json
        ...
        ```

## 2. Configure the React Native project

:::info
Please refer to [react-native-app](https://github.com/zkmopro/react-native-app) to see the latest update.
:::

### 2-1 Install dependencies

-   Install `react-native-monorepo-config` package

    ```sh
    npm add react-native-monorepo-config
    ```

### 2-2 Add workspace for the binding

-   In your root `package.json`, add a `"workspaces"` field like this:
    ```json title="package.json"
    {
        ...
        "private": true,
        "workspaces": [
            "MoproReactNativeBindings"
        ]
        ...
    }
    ```

### 2-3 Add `react-native.config.js`

-   Add a `react-native.config.js` file in the root. Please refer to the latest [`react-native.config.js`](https://github.com/zkmopro/react-native-app/blob/ubrn/react-native.config.js)

### 2-4 Update `metro.config.js`

-   Update `metro.config.js` in the root. Please refer to the latest [`metro.config.js`](https://github.com/zkmopro/react-native-app/blob/ubrn/metro.config.js)

### 2-5 Update `index.js`

-   Update `index.js` in the root to initialize UniFFI settings. For example,

    ```ts title="index.js"
    import { AppRegistry } from "react-native";
    import App from "./App";
    import { name as appName } from "./app.json";
    import { uniffiInitAsync } from "mopro-ffi"; // name of the bindings

    uniffiInitAsync().then(() => {
        AppRegistry.registerComponent(appName, () => App);
    });
    ```

## 3. Run the app

:::info
Please refer to [react-native-app](https://github.com/zkmopro/react-native-app) to see the latest update.
:::

### 3-1. Install the dependencies

Install `react-native-fs` to use the assets (proving keys).

```sh
npm add react-native-fs
```

And make sure you run

```sh
npm install
```

and

```sh
cd ios && pod install && cd ..
```

for iOS.

### 3-2. Check the script

The `android` and `ios` scripts should be defined as follows to support running with the mobile native modules:

```json title="package.json"
{
    ...
    "scripts": {
        ...
        "assets": "npx react-native-asset",
        "prebuild": "npm run assets && cd ios && pod install && cd ..",
        "ios": "npm run prebuild && react-native run-ios",
        "android": "npm run assets && react-native run-android",
        ...
    }
    ...
}
```

### 3.3 Generate proofs in the app

Here is an example demonstrating how to generate a proof within an app:

```ts
import {
    generateCircomProof,
    verifyCircomProof,
    CircomProofResult,
    ProofLib,
} from "mopro-ffi";

const circuitInputs = {
    a: ["3"],
    b: ["5"],
};
const result: CircomProofResult = generateCircomProof(
    zkeyPath.replace("file://", ""),
    JSON.stringify(circuitInputs),
    ProofLib.Arkworks
);
const valid: boolean = verifyCircomProof(
    zkeyPath.replace("file://", ""),
    result,
    ProofLib.Arkworks
);
```

:::info
To load zkey from assets, you can refer to the script: [`loadAssets()`](https://github.com/zkmopro/react-native-app/blob/0c34f13f17268749adb433ffe5065b1960a2a68d/src/App.tsx#L30-L52)
:::

:::info
Please refer to [react-native-app](https://github.com/zkmopro/react-native-app) to see the latest update.
:::

### 3.4 Run in simulators

-   Start a react native server
    ```bash
    npm run start
    ```
-   **Android**

    Export `ANDROID_HOME`

    ```bash
    export ANDROID_HOME="~/Library/Android/sdk"
    ```

    Then run

    ```bash
    npm run android
    ```

-   **iOS**

    ```bash
    npm run ios
    ```

    To run the app on a real iOS device, open the Xcode workspace:

    ```bash
    open ios/MyTestLibraryExample.xcworkspace
    ```

    Then, in Xcode, select your project in the sidebar, go to **Signing & Capabilities** → **Signing**, and choose your Apple account (team) under Team.

## 4. What's next

-   **Update your ZK circuits** as needed. After making changes, be sure to run:

    ```sh
    mopro build
    mopro update
    ```

    :::warning
    `mopro update` only works if the Android project was created _within_ the Rust project directory during mopro init. Otherwise, you can manually update the bindings by following [Step 2-1](#2-1-use-a-framework) and [Step 3-2](#3-2-include-mopro-bindings-in-the-native-android-module).
    :::

    This ensures the bindings are regenerated and reflect your latest updates.

-   **Build your mobile app frontend** according to your business logic and user flow.
-   **Expose additional Rust functionality:**
    If a function is missing in Swift, Kotlin, React Native, or Flutter, you can:

    -   Add the required Rust crate in `Cargo.toml`
    -   Annotate your function with `#[uniffi::export]` (See the [Rust setup](rust-setup.md#-customize-the-bindings) guide for details).<br/>
        Once exported, the function will be available across all supported platforms.
