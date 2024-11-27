# Flutter Setup

After completing the [Rust setup](rust-setup.md) and setting up either [iOS setup](ios-setup.md) or [Android setup](android-setup.md), you're ready to create a cross-platform project using [Flutter](https://flutter.dev/). <br/>
Flutter is a framework for building natively compiled, multi-platform applications from a single codebase.

## Flutter app example

<div style={{ display: 'flex', justifyContent: 'center', gap: '10px' }}>
    <img src="/img/flutter-android.png" alt="Android app screenshot" width="250"/>
    <img src="/img/flutter-ios.png" alt="iOS app screenshot" width="250"/>
</div>

You can now clone the repository from [zkmopro/flutter-app](https://github.com/zkmopro/flutter-app) with

```sh
git clone https://github.com/zkmopro/flutter-app
```

## Running The Example App

### Prerequisites

1. **Install Flutter**

   If Flutter is not already installed, you can follow the [official Flutter installation guide](https://docs.flutter.dev/get-started/install) for your operating system.

2. **Check Flutter Environment**

   After installing Flutter, verify that your development environment is properly set up by running the following command in your terminal:

   ```bash
   flutter doctor
   ```

   This command will identify any missing dependencies or required configurations.

3. **Install Flutter Dependencies**

   Navigate to the root directory of the project in your terminal and run:

   ```bash
   flutter pub get
   ```

   This will install the necessary dependencies for the project.

### Running the App via VS Code

1. Open the project in VS Code.
2. Open the "Run and Debug" panel.
3. Start an emulator (iOS/Android) or connect your physical device.
4. Select "example" in the run menu and press "Run".

### Running the App via Console

If you prefer using the terminal to run the app, use the following steps:

1. For Android:

   Ensure you have an Android emulator running or a device connected. Then run:

   ```bash
   flutter run
   ```

2. For iOS:

   Make sure you have an iOS simulator running or a device connected. Then run:

   ```bash
   flutter run
   ```

## Integrating Your ZKP

The example app comes with a simple prover generated from a Circom circuit. To integrate your own prover, follow the steps below.

### Setup

Follow the [Rust Setup steps from the mopro official docs](https://zkmopro.org/docs/getting-started/rust-setup) to generate the platform-specific libraries.

### Copying The Generated Libraries

#### iOS

```
flutter-app/
├── ...
└── mopro_flutter_plugin
    └── ios/
        ├── ...
        ├── Classes/
        │   ├── ...
        │   └── mopro.swift
        └── MoproBindings.xcframework/...
```

1. Replace `mopro.swift` file at `mopro_flutter_plugin/ios/Classes/mopro.swift` with the one generated during the [Setup](#setup).
2. Replace the directory `mopro_flutter_plugin/ios/MoproBindings.xcframework` with the one generated during the [Setup](#setup).

#### Android

```
flutter-app/
├── ...
└── mopro_flutter_plugin
    └── android/
        ├── ...
        └── src/
            ├── ...
            └── main/
                ├── ...
                ├── jniLibs/...
                └── kotlin/
                    ├── ...
                    └── uniffi/mopro/mopro.kt
```

1. Replace the directory `mopro_flutter_plugin/android/src/main/jniLibs` with the one generated during the [Setup](#setup).
2. Replace `mopro.kt` file at `mopro_flutter_plugin/android/src/main/kotlin/uniffi/mopro/mopro.kt` with the one generated during the [Setup](#setup).

### zKey

```
flutter-app/
├── ...
├── assets/multiplier2_final.zkey
└── lib/main.dart
```

1. Place your `.zkey` file in your app's assets folder and remove the example file `assets/multiplier2_final.zkey`. If your `.zkey` has a different file name, don't forget to update the asset definition in your app's `pubspec.yaml`:

   ```diff
   assets:
   -  - assets/multiplier2_final.zkey
   +  - assets/your_new_zkey_file.zkey
   ```

2. Load the new `.zkey` file in your Dart code by updating the file path in `lib/main.dart`:

   ```diff
   var inputs = <String, List<String>>{};
   inputs["a"] = ["3"];
   inputs["b"] = ["5"];
   - proofResult = await _moproFlutterPlugin.generateProof("assets/multiplier2_final.zkey", inputs);
   + proofResult = await _moproFlutterPlugin.generateProof("assets/your_new_zkey_file.zkey", inputs);
   ```

Don't forget to modify the input values for your specific case!

## Modifying The Flutter Plugin code

```
flutter-app/
├── ...
└── mopro_flutter_plugin
```

You can find the Flutter plugin code that enables the communication between Flutter and you generated libraries in the `mopro_flutter_plugin` directory. However, typical IDEs may not provide platform-specific features such as syntax highlighting, code completion, or error detection if you load the whole project in your IDE. Here are some tips on how to edit the platform-specific plugin code:

:::info

### Android

Open the `./android` directory in Android Studio. You will be able to browse to the plugin code in `Project` view:

<div style={{ display: 'flex', justifyContent: 'center', gap: '10px' }}>
    <img src="/img/flutter-plugin-android.png" alt="mopro.kt & MoproFlutterPlugin.kt" width="250"/>
</div>

### iOS

Open the `./ios` directory in Xcode. You can find the plugin code in the `Project navigator`:

<div style={{ display: 'flex', justifyContent: 'center', gap: '10px' }}>
    <img src="/img/flutter-plugin-ios.png" alt="mopro.swift & MoproFlutterPlugin.swift" width="250"/>
</div>
:::
