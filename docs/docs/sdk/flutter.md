---
title: Flutter SDK
---

In this section, you'll learn how to build a Flutter SDK using Mopro's native bindings.
The process includes the following steps:

1. [Generate the binding using the Mopro CLI](#generate-the-bindings-using-the-mopro-cli)
2. [Integrate the binding into a Flutter Package](#integrate-the-bindings-into-a-flutter-package)
3. [How to install the Flutter SDK](#how-to-install-the-flutter-sdk)
4. [How to use the package](#how-to-use-the-package)
5. [How to run an example app](#how-to-run-an-example-app)

## Generate the bindings using the Mopro CLI

To get started with building Mopro bindings, refer to the [Getting Started](/docs/getting-started) section.
If you’d like to generate custom bindings for your own circuits or proving schemes, see the guide:
[Rust Setup for Android/iOS Bindings](/docs/setup/rust-setup).

Then you will have a `MoproAndroidBindings` and/or `MoproiOSBindings` in the project directory.

## Integrate the bindings into a Flutter Package

1. Clone the SDK template repository:

```sh
git clone https://github.com/zkmopro/mopro_flutter_package
```

2. Replace the generated bindings:

-   **iOS:** Replace the bindings directory `MoproiOSBindings` with the generated files in the following location:
    -   `ios/MoproiOSBindings`
-   **Android:** Replace the bindings directory `MoproAndroidBindings/uniffi` and `MoproAndroidBindings/jniLibs` with your generated files in the following location:

    -   `android/src/main/kotlin/uniffi`
    -   `android/src/main/jniLibs`

3. Define the module API

-   **iOS:**
    -   Define the native module API in [`ios/Classes/MoproFlutterPackagePlugin.swift`](https://github.com/zkmopro/mopro_flutter_package/blob/d0ed1a1ce35b24afca8d8e28ea6dcba230c6b584/ios/Classes/MoproFlutterPackagePlugin.swift#L5) to match the Flutter type. Please refer to [Flutter - Data types support](https://docs.flutter.dev/platform-integration/platform-channels#codec).
-   **Android:**
    -   Then define the native module API in [`android/src/main/kotlin/com/example/mopro_flutter_package/MoproFlutterPackagePlugin.kt`](https://github.com/zkmopro/mopro_flutter_package/blob/d0ed1a1ce35b24afca8d8e28ea6dcba230c6b584/android/src/main/kotlin/com/example/mopro_flutter_package/MoproFlutterPackagePlugin.kt#L14) to match the Flutter type. Please refer to [Flutter - Data types support](https://docs.flutter.dev/platform-integration/platform-channels#codec).
-   **Flutter:**
    -   Define Flutter's platform channel APIs to pass messages between Flutter and your desired platforms.
        -   [`lib/mopro_flutter_package_method_channel.dart`](https://github.com/zkmopro/mopro_flutter_package/blob/d0ed1a1ce35b24afca8d8e28ea6dcba230c6b584/lib/mopro_flutter_package_method_channel.dart#L8)
        -   [`lib/mopro_flutter_package_platform_interface.dart`](https://github.com/zkmopro/mopro_flutter_package/blob/d0ed1a1ce35b24afca8d8e28ea6dcba230c6b584/lib/mopro_flutter_package_platform_interface.dart#L6)
        -   [`lib/mopro_flutter_package.dart`](https://github.com/zkmopro/mopro_flutter_package/blob/d0ed1a1ce35b24afca8d8e28ea6dcba230c6b584/lib/mopro_flutter_package.dart#L15)

## How to install the Flutter SDK

Add `mopro_flutter_package` to your project by manually editing `pubspec.yaml`.

```yaml
dependencies:
    flutter:
        sdk: flutter

    mopro_flutter_package: # Or change to your own package name
        git:
            url: https://github.com/zkmopro/mopro_flutter_package # Or change to your own URL
```

:::info
If you need to update your keys, include your compiled Circom `.zkey` file as an asset. Add the asset path to your `pubspec.yaml` under the flutter: section:

```yaml
flutter:
    uses-material-design: true # Ensure this is present
    assets:
        # Add the directory containing your .zkey file(s)
        - assets/circuits/
        # Or specify the file directly:
        # - assets/circuits/multiplier2_final.zkey
```

Make sure the path points correctly to where you've placed your `.zkey` file within your Flutter project.

:::

Run the following command in your terminal from the root of your Flutter project:

```sh
flutter pub get
```

## How to use the package

Here is an example of how to integrate and use this package

```dart
// Import the package
import 'package:mopro_flutter_package/mopro_flutter_package.dart';
import 'package:mopro_flutter_package/mopro_flutter_types.dart';

final MoproFlutterPackage _mopro = MoproFlutterPackage();
const int a = 3;
const int b = 5;
final Map<String, List<String>> inputs = {
    'a': [a.toString()],
    'b': [b.toString()],
};
// Convert inputs to JSON string
final String inputsJson = jsonEncode(inputs);

final GenerateProofResult proofResult = await _mopro.generateProof(
    zkeyPath: zkeyAssetName, // Use the zkey asset path provided in pubspec.yaml (e.g. "assets/multiplier2_final.zkey")
    inputs: inputsJson,
);

final bool isValid = await _mopro.verifyProof(
    zkeyPath: zkeyAssetName, // Use the same zkey asset path
    proof: proofResult.proof, // Use the generated proof
    inputs: proofResult.inputs, // Use the public inputs from proof generation
);
```

## How to run an example app

-   Open the example app that uses the defined flutter package in the [`example/`](https://github.com/zkmopro/mopro_flutter_package/tree/main/example) folder
    ```sh
    cd example
    ```
-   Install the dependencies
    ```sh
    flutter pub get
    ```
-   Open an iOS simulator/device or an Android emulator/device and run the example app
    ```sh
    flutter run
    ```
-   Clean the cache if you update the bindings and it throws errors
    ```sh
    flutter clean
    ```
