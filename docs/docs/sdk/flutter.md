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

Choose **Flutter** to build the bindings, or run

```sh
mopro build --platforms flutter
```

to generate the flutter package.

Then you will have a `mopro_flutter_bindings` in the project directory.

## Integrate the bindings into a Flutter Package

1. Clone the SDK template repository:

```sh
git clone https://github.com/zkmopro/mopro_flutter_package
```

2. Replace the generated bindings:

replace the entire bindings directory `mopro_flutter_package` with your generated files in the current folder:

```t
├── android
├── cargokit
├── flutter_rust_bridge.yaml
├── ios
├── lib
├── pubspec.yaml
└── rust
```

or running e.g.

```sh
cp -R \
  mopro_flutter_bindings/android \
  mopro_flutter_bindings/cargokit \
  mopro_flutter_bindings/flutter_rust_bridge.yaml \
  mopro_flutter_bindings/ios \
  mopro_flutter_bindings/lib \
  mopro_flutter_bindings/pubspec.yaml \
  mopro_flutter_bindings/rust \
  mopro_flutter_package/
```

## How to install the Flutter SDK

Add `mopro_flutter_bindings` to your project by manually editing `pubspec.yaml`.

```yaml
dependencies:
    flutter:
        sdk: flutter

    mopro_flutter_bindings: # Or change to your own package name
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

Update the main function to initialize the Rust library before running the app:

```dart
void main() async {
  await RustLib.init();
  runApp(const MyApp());
}
```

Import the package and use it:

```dart
// Import the package
import 'package:mopro_flutter_bindings/src/rust/third_party/mopro_example_app.dart'; // Change to your library name
import 'package:mopro_flutter_bindings/src/rust/frb_generated.dart';

final zkeyPath = await copyAssetToFileSystem(
    'assets/multiplier2_final.zkey',
)

// Corresponds to the inputs of multiplier2.circom
const int a = 3;
const int b = 5;
final Map<String, List<String>> inputs = {
    'a': [a.toString()],
    'b': [b.toString()],
};
// Convert inputs to JSON string
final String inputsJson = jsonEncode(inputs);

// Use the zkey asset path provided in pubspec.yaml
final CircomProofResult proofResult = await generateCircomProof(
    zkeyPath: zkeyPath,
    circuitInputs: inputsJson,
    proofLib: ProofLib.arkworks,
);

final bool isValid = await verifyCircomProof(
    zkeyPath: zkeyPath,
    proofResult: proofResult,
    proofLib: ProofLib.arkworks,
);

print('Generated Proof: ${proofResult.proof}');
print('Public Inputs/Outputs: ${proofResult.inputs}');
print('Verification result: $isValid');
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
