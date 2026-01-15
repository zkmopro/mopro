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
[Rust Setup for Android/iOS Bindings](/docs/setup/rust-setup). Choose the **React Native** platform, or run

```sh
mopro build --platform react-native
```

Then you will have a `MoproReactNativeBindings` in the project directory.

## Integrate the bindings into a React Native Package

Then, replace the entire bindings directory `MoproReactNativeBindings` with your generated files in the current folder:

```t
├── android
├── babel.config.js
├── cpp
├── example # Optional: keep this folder
├── ios
├── lib
├── MoproFfiFramework.xcframework
├── node_modules
├── package-lock.json
├── package.json
├── README.md
├── src
├── tsconfig.build.json
├── tsconfig.json
├── turbo.json
└── ubrn.config.yaml
```

or running e.g.

```sh
cp -R \
  MoproReactNativeBindings/android \
  MoproReactNativeBindings/ios \
  MoproReactNativeBindings/src \
  MoproReactNativeBindings/lib \
  MoproReactNativeBindings/MoproFfiFramework.xcframework \
  MoproReactNativeBindings/package.json \
  mopro-react-native-package/
```

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
 "mopro-ffi": "github:zkmopro/mopro-react-native-package", // Or change to your own URL
}
```

## How to use the package

Here is an example of how to integrate and use this package

```ts
import {
    CircomProofResult,
    generateCircomProof,
    ProofLib,
    verifyCircomProof,
} from "mopro-ffi";

const circuitInputs = {
    a: [a],
    b: [b],
};

const circomProofResult: CircomProofResult = await generateCircomProof(
    ZKEY_PATH,
    JSON.stringify(circuitInputs),
    ProofLib.Arkworks
);

const isValid: boolean = await verifyCircomProof(
    ZKEY_PATH,
    circomProofResult,
    ProofLib.Arkworks
);

console.log("Proof verification result:", isValid);
```

:::note
To learn how to read a .zkey file from an app, please refer to the [`loadAssets`](https://github.com/zkmopro/react-native-app/blob/7bd97d6256644727253716fddcc4f07c17a61293/src/App.tsx#L30) function in the React Native app.
:::

:::warning
The default bindings are built specifically for the `multiplier2` circom circuit. If you'd like to update the circuit or switch to a different proving scheme, please refer to the [How to Build the Package](#how-to-build-the-package) section.<br/>
Circuit source code: https://github.com/zkmopro/circuit-registry/tree/main/multiplier2<br/>
Example .zkey file for the circuit: http://ci-keys.zkmopro.org/multiplier2_final.zkey<br/>
:::

And in `index.js`, for example, replace this with

```diff
import { AppRegistry } from 'react-native';
import App from './src/App';
import { name as appName } from './app.json';
+ import { uniffiInitAsync } from 'mopro-ffi';

+ uniffiInitAsync().then(() => {
    AppRegistry.registerComponent(appName, () => App);
+ });

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
