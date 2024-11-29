# Flutter Setup

After completing the [Rust setup](rust-setup.md) and setting up either [iOS setup](ios-setup.md) or [Android setup](android-setup.md), you're ready to create a cross-platform project using [Flutter](https://flutter.dev/). <br/>
Flutter is a framework for building natively compiled, multi-platform applications from a single codebase.

## 1. Prerequisites

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

## 2. Integrating mopro into a Flutter app

Depending on your case, you may want to integrate mopro into a [new](#21-creating-a-new-mopro-enabled-flutter-app) or [existing](#22-integrating-mopro-into-existing-flutter-app) Flutter app.

## 2.1 Creating a New mopro-Enabled Flutter App

Use the provided example app as a starting point.

<div style={{ display: 'flex', justifyContent: 'center', gap: '10px' }}>
    <img src="/img/flutter-android.png" alt="Android app screenshot" width="250"/>
    <img src="/img/flutter-ios.png" alt="iOS app screenshot" width="250"/>
</div>

Clone the repository from [zkmopro/flutter-app](https://github.com/zkmopro/flutter-app) with

```sh
git clone https://github.com/zkmopro/flutter-app
```

### Running the App via VS Code

1. Open the project in VS Code.
2. Open the "Run and Debug" panel.
3. Start an emulator (iOS/Android) or connect your physical device.
4. Select "example" in the run menu and press "Run".

### Running the App via Console

If you prefer using the terminal to run the app, use the following steps:

1. Ensure you have either an Android or iOS emulator running or a device connected.
2. Execute the following command:
   ```bash
   flutter run
   ```

### Integrating mopro bindings

The example app comes with a simple prover generated from a Circom circuit. To integrate your own prover, follow the [Rust Setup steps](/setup/rust-setup.md) to generate the platform-specific libraries. Then, follow the steps below to integrate the generated libraries into the Flutter app.

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

1. Replace `mopro.swift` file at `mopro_flutter_plugin/ios/Classes/mopro.swift` with the one generated during the [Rust Setup](/setup/rust-setup.md).
2. Replace the directory `mopro_flutter_plugin/ios/MoproBindings.xcframework` with the one generated during the [Rust Setup](/setup/rust-setup.md).

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

1. Replace the directory `mopro_flutter_plugin/android/src/main/jniLibs` with the one generated during the [Rust Setup](/setup/rust-setup.md).
2. Replace `mopro.kt` file at `mopro_flutter_plugin/android/src/main/kotlin/uniffi/mopro/mopro.kt` with the one generated during the [Rust Setup](/setup/rust-setup.md).

#### Customizing the zKey

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

## 2.2 Integrating mopro Into Existing Flutter App

If you already have a Flutter project, follow the steps below to integrate mopro.

1. Copy the `mopro_flutter_plugin` directory from [the repository](https://github.com/zkmopro/flutter-app) root into the root folder of your existing Flutter project:

   ```
   your-flutter-app/
   ├── ...
   ├── lib/
   ├── android/
   ├── ios/
   ├── pubspec.yaml
   └── mopro_flutter_plugin/
   ```

2. Add the plugin to `pubspec.yaml` as a dependency:

   ```yaml
   ---
   dependencies:
   flutter:
     sdk: flutter
   mopro_flutter_plugin:
     path: ./mopro_flutter_plugin
   ```

3. Follow the steps described in [Integrating mopro bindings](#integrating-mopro-bindings) section to generate your platform-specific libraries.
4. Place the libraries in the corresponding directories for [iOS](#ios) and [Android](#android) as described [above](#integrating-mopro-bindings).
5. Follow the steps described in [Customizing the zKey section](#customizing-the-zkey) to load your `.zkey` file.

## 3. Modifying The Flutter Plugin code (Optional)

You can find the Flutter plugin code that enables the communication between Flutter and your generated libraries in the `mopro_flutter_plugin` directory. However, typical IDEs may not provide platform-specific features such as syntax highlighting, code completion, or error detection if you load the whole project in your IDE. Here are some tips on how to edit the platform-specific plugin code:

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
