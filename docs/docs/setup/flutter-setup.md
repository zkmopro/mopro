# Flutter Setup

This tutorial will guide you through integrating the iOS bindings and Android bindings into an [Flutter](https://flutter.dev/)) project. Before you begin, make sure you’ve completed the ["Getting Started - 3. Mopro build"](/docs/getting-started.md#3-build-bindings) process with selecting **iOS** platform and **Android** platform and have the `MoproiOSBindings` and `MoproAndroidBindings` folder ready: <br/>

Flutter is a framework for building natively compiled, multi-platform applications from a single codebase.

In this tutorial, you will learn how to create a native Mopro module on both Android and iOS simulators/devices. <br/>

<div style={{ display: 'flex', justifyContent: 'center', gap: '10px' }}>
    <img src="/img/flutter-android.png" alt="Android app screenshot" width="250"/>
    <img src="/img/flutter-ios.png" alt="iOS app screenshot" width="250"/>
</div>

:::info
In this example, we use Circom circuits and their corresponding `.zkey` files. The process is similar for other provers.
:::

## 0. Prerequisites

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

4. **Create a Flutter App**

    If you already have Flutter app, you can skip this step.
    If you don’t have a one, follow [this tutorial](https://codelabs.developers.google.com/codelabs/flutter-codelab-first) to set one up. Or run

    ```bash
    flutter create <YOUR_FLUTTER_APP>
    ```

## 1. Creating a Native Module

Create a plugin to integrate Mopro bindings into your project.

```bash
flutter create mopro_flutter_plugin -t plugins
```

:::info
To learn more about flutter packages/plugins, please refer to [Flutter - Developing packages & plugins](https://docs.flutter.dev/packages-and-plugins/developing-packages)
:::

To support both iOS and Android, we need to build native modules for each platform.

Start by adding the necessary platforms to the plugin:

Navigate to the `mopro_flutter_plugin` directory

```bash
cd mopro_flutter_plugin
```

and run the following command:

```bash
flutter create -t plugin --platforms ios,android .
```

## 2. Implement the module on iOS

:::info
Please refer to [flutter-app](https://github.com/zkmopro/flutter-app) to see the latest update.
:::

### 2-1 Use a framework

-   Get the `MoproiOSBindings` from `mopro build`.
    :::info
    See [Getting Started](/docs/getting-started.md)
    :::

-   Place the `MoproiOSBindings/mopro.swift` file to `mopro_flutter_plugin/ios/Classes/mopro.swift`<br/>
    Place the `MoproiOSBindings/MoproBindings.xcframework` file to `mopro_flutter_plugin/ios/MoproBindings.xcframework`.<br/>
    The structure will look like
    ```sh
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
-   Bundle the bindings in `mopro_flutter_plugin/ios/mopro_flutter_plugin.podspec`
    ```ruby title="/mopro_flutter_plugin/ios/mopro_flutter_plugin.podspec"
        ...
        s.source_files = 'Classes/**/*'
        s.vendored_frameworks = 'MoproBindings.xcframework'
        s.preserve_paths = 'MoproBindings.xcframework/**/*'
        ...
    ```

### 2-2 Create convertible types for Javascript library with swift.

-   Create these types in the file: `mopro_flutter_plugin/ios/Classes/MoproFlutterPlugin.swift`

        ```swift title="/mopro_flutter_plugin/ios/Classes/MoproFlutterPlugin.swift"
        class FlutterG1 {
            let x: String
            let y: String
            let z: String

            init(x: String, y: String, z: String) {
               self.x = x
               self.y = y
               self.z = z
            }
        }
        // ...
        ```

        :::note
        See the full implementation here: [`MoproFlutterPlugin.swift`](https://github.com/zkmopro/flutter-app/blob/3f3f2201607084532c13d4abedbc3f8b68b566ce/mopro_flutter_plugin/ios/Classes/MoproFlutterPlugin.swift#L6C1-L54C2)
        :::

-   Define helper functions to bridge types between the Mopro bindings and the Flutter framework:

        ```swift title="/mopro_flutter_plugin/ios/Classes/MoproFlutterPlugin.swift"
         // convert the mopro proofs to be exposed to Flutter framework
         func convertCircomProof(res: CircomProofResult) -> [String: Any] {
            let g1a = FlutterG1(x: res.proof.a.x, y: res.proof.a.y, z: res.proof.a.z)
            // ...
         }

         // convert the Flutter proofs to be used in mopro bindings
         func convertCircomProofResult(proof: [String: Any]) -> CircomProofResult {
            let proofMap = proof["proof"] as! [String: Any]
            let aMap = proofMap["a"] as! [String: String]
            let g1a = G1(x: aMap["x"] ?? "0", y: aMap["y"] ?? "0", z: aMap["z"] ?? "1")
            // ...
         }
        ```
        :::note
        See the full implementation here: [`MoproFlutterPlugin.swift`](https://github.com/zkmopro/flutter-app/blob/3f3f2201607084532c13d4abedbc3f8b68b566ce/mopro_flutter_plugin/ios/Classes/MoproFlutterPlugin.swift#L56C1-L103C2)
        :::

-   Define the native module API. See the [Writing custom platform-specific code](https://docs.flutter.dev/platform-integration/platform-channels) for details.

```swift title="/mopro_flutter_plugin/ios/Classes/MoproFlutterPlugin.swift"
public class MoproFlutterPlugin: NSObject, FlutterPlugin {
  public static func register(with registrar: FlutterPluginRegistrar) {
    let channel = FlutterMethodChannel(name: "mopro_flutter_plugin", binaryMessenger: registrar.messenger())
    let instance = MoproFlutterPlugin()
    registrar.addMethodCallDelegate(instance, channel: channel)
  }

  public func handle(_ call: FlutterMethodCall, result: @escaping FlutterResult) {
      switch call.method {
         case "generateCircomProof":
         guard let args = call.arguments as? [String: Any],
            let zkeyPath = args["zkeyPath"] as? String,
            let inputs = args["inputs"] as? String,
            let proofLib = args["proofLib"] as? ProofLib
         else {
            result(FlutterError(code: "ARGUMENT_ERROR", message: "Missing arguments", details: nil))
            return
         }

         do {
            // Call the function from mopro.swift
            let proofResult = try generateCircomProof(
               zkeyPath: zkeyPath, circuitInputs: inputs, proofLib: proofLib)
            let resultMap = convertCircomProof(res: proofResult)

            // Return the proof and inputs as a map supported by the StandardMethodCodec
            result(resultMap)
         } catch {
            result(
               FlutterError(
                  code: "PROOF_GENERATION_ERROR", message: "Failed to generate proof",
                  details: error.localizedDescription))
         }
      }
   }
   // ...
}
```

:::info
See the full implementation here: [`MoproFlutterPlugin.swift`](https://github.com/zkmopro/flutter-app/blob/3f3f2201607084532c13d4abedbc3f8b68b566ce/mopro_flutter_plugin/ios/Classes/MoproFlutterPlugin.swift#L115C5-L138C8)
:::

## 3. Implement the module on Android

### 3-1 Add dependency for [jna](https://github.com/java-native-access/jna) in the file `build.gradle`.

```kotlin title="/mopro_flutter_plugin/android/build.gradle"
dependencies {
  implementation("net.java.dev.jna:jna:5.13.0@aar")
}
```

### 3-2 Include Mopro bindings in the native Android module

-   Get the `MoproAndroidBindings` from `mopro build`.
    :::info
    See [Getting Started](/docs/getting-started.md)
    :::
-   Move the `jniLibs` directory to `mopro_flutter_plugin/android/src/main`. <br/>
    And move `uniffi` directory to `mopro_flutter_plugin/android/src/main/kotlin`.<br/>
    The folder structure should be as follows:

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

### 3-3 Create convertible types for Javascript library with kotlin.

-   Create these types in the file: `mopro_flutter_plugin/android/src/main/kotlin/com/example/mopro_flutter_plugin/MoproFlutterPlugin.kt`

    ```kotlin title="/mopro_flutter_plugin/android/src/main/kotlin/com/example/mopro_flutter_plugin/MoproFlutterPlugin.kt"
    class FlutterG1(x: String, y: String, z: String) {
       val x = x
       val y = y
       val z = z
    }
    // ...
    ```

    :::note
    See the full implementation here: [`MoproFlutterPlugin.kt`](https://github.com/zkmopro/flutter-app/blob/3f3f2201607084532c13d4abedbc3f8b68b566ce/mopro_flutter_plugin/android/src/main/kotlin/com/example/mopro_flutter/MoproFlutterPlugin.kt#L13C1-L36C2)
    :::

-   Define helper functions to bridge types between the Mopro bindings and the Flutter framework:

        ```kotlin title="/mopro_flutter_plugin/android/src/main/kotlin/com/example/mopro_flutter_plugin/MoproFlutterPlugin.kt"
         // convert the mopro proofs to be exposed to Flutter framework
         fun convertCircomProof(res: CircomProofResult): Map<String, Any> {
            val g1a = FlutterG1(res.proof.a.x, res.proof.a.y, res.proof.a.z)
            // ...
         }

         // convert the Flutter proofs to be used in mopro bindings
         fun convertCircomProofResult(proofResult: Map<String, Any>): CircomProofResult {
            val proofMap = proofResult["proof"] as Map<String, Any>
            val aMap = proofMap["a"] as Map<String, Any>
            val g1a = G1(
               aMap["x"] as String,
               aMap["y"] as String,
               aMap["z"] as String
            )
            // ...
         }
        ```

    :::note
    See the full implementation here: [`MoproFlutterPlugin.kt`](https://github.com/zkmopro/flutter-app/blob/3f3f2201607084532c13d4abedbc3f8b68b566ce/mopro_flutter_plugin/android/src/main/kotlin/com/example/mopro_flutter/MoproFlutterPlugin.kt#L38C1-L98C4)
    :::

-   Define the native module API. See the [Writing custom platform-specific code](https://docs.flutter.dev/platform-integration/platform-channels) for details.

```kotlin title=/mopro_flutter_plugin/android/src/main/kotlin/com/example/mopro_flutter_plugin/MoproFlutterPlugin.kt
class MoproFlutterPlugin: FlutterPlugin, MethodCallHandler {
  /// The MethodChannel that will the communication between Flutter and native Android
  ///
  /// This local reference serves to register the plugin with the Flutter Engine and unregister it
  /// when the Flutter Engine is detached from the Activity
  private lateinit var channel : MethodChannel

  override fun onAttachedToEngine(flutterPluginBinding: FlutterPlugin.FlutterPluginBinding) {
    channel = MethodChannel(flutterPluginBinding.binaryMessenger, "mopro_flutter_plugin")
    channel.setMethodCallHandler(this)
  }

  override fun onMethodCall(call: MethodCall, result: Result) {
      if (call.method == "generateCircomProof") {
         val zkeyPath = call.argument<String>("zkeyPath") ?: return result.error(
            "ARGUMENT_ERROR",
            "Missing zkeyPath",
            null
         )

         val inputs =
            call.argument<String>("inputs") ?: return result.error(
               "ARGUMENT_ERROR",
               "Missing inputs",
               null
            )

         val proofLibIndex = call.argument<Int>("proofLib") ?: return result.error(
            "ARGUMENT_ERROR",
            "Missing proofLib",
            null
         )

         val proofLib = if (proofLibIndex == 0) ProofLib.ARKWORKS else ProofLib.RAPIDSNARK

         val res = generateCircomProof(zkeyPath, inputs, proofLib)
         val resultMap = convertCircomProof(res)


         result.success(resultMap)
      // ...
      } else {
         result.notImplemented()
      }
  }
   // ...
}
```

:::note
See the full implementation here: [`MoproFlutterPlugin.kt`](https://github.com/zkmopro/flutter-app/blob/3f3f2201607084532c13d4abedbc3f8b68b566ce/mopro_flutter_plugin/android/src/main/kotlin/com/example/mopro_flutter/MoproFlutterPlugin.kt#L118C9-L144C38)
:::

## 4. Define Dart APIs

:::info
Please refer to [flutter-app](https://github.com/zkmopro/flutter-app) to see the latest update.
:::

-   Define the types for the native module. Add the following types in the file `mopro_flutter_plugin/lib/mopro_types.dart`:

    ```dart title="/mopro_flutter_plugin/lib/mopro_types.dart"
    import 'dart:typed_data';

    class G1Point {
       final String x;
       final String y;
       final String z;

       G1Point(this.x, this.y, this.z);

       @override
       String toString() {
          return "G1Point(\nx: $x, \ny: $y, \nz: $z)";
       }
    }

    enum ProofLib { arkworks, rapidsnark }

    // ...
    ```

    :::note
    See the full implementation here: [`mopro_types.dart`](https://github.com/zkmopro/flutter-app/blob/3f3f2201607084532c13d4abedbc3f8b68b566ce/mopro_flutter_plugin/lib/mopro_types.dart#L3C1-L81C2)
    :::

-   Add the native module's API functions in these files.

```dart title="mopro_flutter_plugin/lib/mopro_flutter_plugin_method_channel.dart"
class MethodChannelMoproFlutterPlugin extends MoproFlutterPluginPlatform {
   /// The method channel used to interact with the native platform.
   @visibleForTesting
   final methodChannel = const MethodChannel('mopro_flutter_plugin');

   @override
   Future<CircomProofResult?> generateCircomProof(
      String zkeyPath, String inputs, ProofLib proofLib) async {
      final proofResult = await methodChannel
        .invokeMethod<Map<Object?, Object?>>('generateCircomProof', {
            'zkeyPath': zkeyPath,
            'inputs': inputs,
            'proofLib': proofLib.index,
         });

      if (proofResult == null) {
         return null;
      }

      var circomProofResult = CircomProofResult.fromMap(proofResult);
      return circomProofResult;
   }
   // ...
}
```

:::note
See the full implementation here: [`mopro_flutter_method_channel.dart`](https://github.com/zkmopro/flutter-app/blob/3f3f2201607084532c13d4abedbc3f8b68b566ce/mopro_flutter_plugin/lib/mopro_flutter_method_channel.dart#L13C1-L30C4)
:::

```dart title="mopro_flutter_plugin/lib/mopro_flutter_plugin_platform_interface.dart"
abstract class MoproFlutterPluginPlatform extends PlatformInterface {
   //...
   Future<CircomProofResult?> generateCircomProof(
      String zkeyPath, String inputs, ProofLib proofLib) {
         throw UnimplementedError('generateCircomProof() has not been implemented.');
   }
   //...
}
```

:::note
See the full implementation here: [`mopro_flutter_platform_interface.dart`](https://github.com/zkmopro/flutter-app/blob/3f3f2201607084532c13d4abedbc3f8b68b566ce/mopro_flutter_plugin/lib/mopro_flutter_platform_interface.dart#L29C3-L32C4)
:::

```dart title="mopro_flutter_plugin/lib/mopro_flutter_plugin.dart"
class MoproFlutterPlugin {
   Future<String> copyAssetToFileSystem(String assetPath) async {
      // Load the asset as bytes
      final byteData = await rootBundle.load(assetPath);
      // Get the app's document directory (or other accessible directory)
      final directory = await getApplicationDocumentsDirectory();
      //Strip off the initial dirs from the filename
      assetPath = assetPath.split('/').last;

      final file = File('${directory.path}/$assetPath');

      // Write the bytes to a file in the file system
      await file.writeAsBytes(byteData.buffer.asUint8List());

      return file.path; // Return the file path
   }

   Future<CircomProofResult?> generateCircomProof(
      String zkeyFile, String inputs, ProofLib proofLib) async {
      return await copyAssetToFileSystem(zkeyFile).then((path) async {
         return await MoproFlutterPlatform.instance
            .generateCircomProof(path, inputs, proofLib);
      });
   }
   //...
}
```

:::note
See the full implementation here: [`mopro_flutter.dart`](https://github.com/zkmopro/flutter-app/blob/3f3f2201607084532c13d4abedbc3f8b68b566ce/mopro_flutter_plugin/lib/mopro_flutter.dart#L10C3-L33C1)
:::

## 5. Use the plugin

Follow the steps below to integrate mopro plugin.

1. Add the plugin to `pubspec.yaml` as a dependency:

    ```yaml
    ---
    dependencies:
    flutter:
        sdk: flutter
    mopro_flutter_plugin:
        path: ./mopro_flutter_plugin
    ```

2. Copy keys in the `assets` folder like this

    ```

    flutter-app/
    ├── ...
    ├── assets/multiplier2_final.zkey
    └── lib/main.dart

    ```

    and update `pubspec.yaml`

    ```yaml
    flutter:
        assets:
            - assets/multiplier2_final.zkey
    ```

3. Generate proofs in the app

    ```dart
    import 'package:mopro_flutter_plugin/mopro_flutter_plugin.dart';
    import 'package:mopro_flutter_plugin/mopro_types.dart';

    final _moproFlutterPlugin = MoproFlutterPlugin();

    var inputs = '{"a":["3"],"b":["5"]}';
    var proofResult = await _moproFlutterPlugin.generateCircomProof(
       "assets/multiplier2_final.zkey",
       inputs,
       ProofLib.arkworks
    );
    ```

## 6. Customizing the zKey

1. Place your `.zkey` file in your app's assets folder and remove the example file `assets/multiplier2_final.zkey`. If your `.zkey` has a different file name, don't forget to update the asset definition in your app's `pubspec.yaml`:

    ```diff
    assets:
    -  - assets/multiplier2_final.zkey
    +  - assets/your_new_zkey_file.zkey
    ```

2. Load the new `.zkey` file in your Dart code by updating the file path in `lib/main.dart`:

    ```diff
    var inputs = '{"a":["3"],"b":["5"]}';
    - proofResult = await _moproFlutterPlugin.generateCircomProof("assets/multiplier2_final.zkey", inputs, ProofLib.arkworks);
    + proofResult = await _moproFlutterPlugin.generateCircomProof("assets/your_new_zkey_file.zkey", inputs, ProofLib.arkworks);
    ```

Don't forget to modify the input values for your specific case!

## 7. What's next

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
