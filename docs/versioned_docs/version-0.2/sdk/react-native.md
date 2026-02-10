---
title: React Native SDK
---

In this section, you'll learn how to build a React Native SDK using Mopro's native bindings.
The process includes the following steps:

1. [Generate the binding using the Mopro CLI](#generate-the-bindings-using-the-mopro-cli)
2. [Integrate the binding into a React Native Package](#integrate-the-bindings-into-a-react-native-package)
3. [How to install the React Native SDK via npm](#how-to-install-the-react-native-sdk-via-npm)
4. [How to use the package](#how-to-use-the-package)
5. [How to run an example app](#how-to-run-an-example-app)

## Generate the bindings using the Mopro CLI

To get started with building Mopro bindings, refer to the [Getting Started](/docs/getting-started) section.
If you’d like to generate custom bindings for your own circuits or proving schemes, see the guide:
[Rust Setup for Android/iOS Bindings](/docs/setup/rust-setup).

Then you will have a `MoproAndroidBindings` and/or `MoproiOSBindings` in the project directory.

## Integrate the bindings into a React Native Package

1. Clone the SDK template repository:

```sh
git clone https://github.com/zkmopro/mopro-react-native-package
```

2. Replace the generated bindings:

-   **iOS:** Replace the bindings directory `MoproiOSBindings` with the generated files in the following location:
    -   `ios/MoproiOSBindings`
-   **Android:** Replace the bindings directory `MoproAndroidBindings/uniffi` and `MoproAndroidBindings/jniLibs` with your generated files in the following location:

    -   `android/src/main/java/uniffi`
    -   `android/src/main/jniLibs`

3. Define the module API

-   **iOS:**
    -   Define the native module API in [`ios/MoproReactNativePackageModule.swift`](https://github.com/zkmopro/mopro-react-native-package/blob/c859bd92e59c0198a47b2b13bc82e25f193529b6/ios/MoproReactNativePackageModule.swift#L91) to match the React Native type. Please refer to [Expo - Argument Types](https://docs.expo.dev/modules/module-api/#argument-types).
-   **Android:**
    -   Then define the native module API in [`android/src/main/java/expo/modules/moproreactnativepackage/MoproReactNativePackageModule.kt`](https://github.com/zkmopro/mopro-react-native-package/blob/c859bd92e59c0198a47b2b13bc82e25f193529b6/android/src/main/java/expo/modules/moproreactnativepackage/MoproReactNativePackageModule.kt#L113) to match the React Native type. Please refer to [Expo - Argument Types](https://docs.expo.dev/modules/module-api/#argument-types).
-   **React Native:**
    -   Define React Native's module APIs to pass messages between React Native and your desired platforms.
        -   [`src/MoproReactNativePackageModule.ts`](https://github.com/zkmopro/mopro-react-native-package/blob/c859bd92e59c0198a47b2b13bc82e25f193529b6/src/MoproReactNativePackageModule.ts#L9)
        -   [`src/MoproReactNativePackageView.tsx`](https://github.com/zkmopro/mopro-react-native-package/blob/c859bd92e59c0198a47b2b13bc82e25f193529b6/src/MoproReactNativePackageView.tsx#L9)

## How to install the React Native SDK via [npm](https://www.npmjs.com/)

Use a Node.js package manager in your React Native app to install dependencies. For example:

```sh
# npm
npm install https://github.com/zkmopro/mopro-react-native-package # Or change to your own URL
# yarn / pnpm
yarn add https://github.com/zkmopro/mopro-react-native-package # Or change to your own URL
```

Alternatively, you can manually add it to your package.json:

```json
"dependencies": {
 "mopro-react-native-package": "github:zkmopro/mopro-react-native-package", // Or change to your own URL
}
```

## How to use the package

Here is an example of how to integrate and use this package

```ts
import MoproReactNativePackage, { Result } from "mopro-react-native-package";

const circuitInputs = {
    a: [a],
    b: [b],
};

const proofResult: CircomProofResult =
    MoproReactNativePackage.generateCircomProof(
        ZKEY_PATH,
        JSON.stringify(circuitInputs)
    );

const isValid: boolean = await MoproReactNativePackage.verifyProof(
    ZKEY_PATH,
    proofResult
);

console.log("Proof verification result:", isValid);
```

## How to run an example app

-   Open the example app that uses the defined react native package in the [`example/`](https://github.com/zkmopro/mopro-react-native-package/tree/main/example) folder

    ```sh
    cd example
    ```

-   Install the dependencies
    ```sh
    npm install
    ```
-   Run on iOS simulator
    ```sh
    npm run ios
    ```
    Run on iOS device
    ```sh
    npm run ios:device
    ```
-   Run on Android emulator/device (if connected)
    Set the `ANDROID_HOME` environment variable.

    ```sh
    export ANDROID_HOME=~/Library/Android/sdk/
    ```

    Run on Android emulator/device (if connected)

    ```sh
    npm run android
    ```
