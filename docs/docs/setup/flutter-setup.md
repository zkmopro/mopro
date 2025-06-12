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

        <details>
            <summary>Circom types in `/mopro_flutter_plugin/ios/Classes/MoproFlutterPlugin.swift`</summary>
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

         class FlutterG2 {
            let x: [String]
            let y: [String]
            let z: [String]

            init(x: [String], y: [String], z: [String]) {
               self.x = x
               self.y = y
               self.z = z
            }
         }

         class FlutterCircomProof {
            let a: FlutterG1
            let b: FlutterG2
            let c: FlutterG1
            let `protocol`: String
            let curve: String

            init(a: FlutterG1, b: FlutterG2, c: FlutterG1, `protocol`: String, curve: String) {
               self.a = a
               self.b = b
               self.c = c
               self.`protocol` = `protocol`
               self.curve = curve
            }
         }

         class FlutterCircomProofResult {
            let proof: FlutterCircomProof
            let inputs: [String]

            init(proof: FlutterCircomProof, inputs: [String]) {
               self.proof = proof
               self.inputs = inputs
            }
         }
        ```
        </details>

-   Define helper functions to bridge types between the Mopro bindings and the Flutter framework:

      <details>
            <summary>Helper functions in `/mopro_flutter_plugin/ios/Classes/MoproFlutterPlugin.swift`</summary>
        ```swift title="/mopro_flutter_plugin/ios/Classes/MoproFlutterPlugin.swift"
         // convert the mopro proofs to be exposed to Flutter framework
         func convertCircomProof(res: CircomProofResult) -> [String: Any] {
            let g1a = FlutterG1(x: res.proof.a.x, y: res.proof.a.y, z: res.proof.a.z)
            let g2b = FlutterG2(x: res.proof.b.x, y: res.proof.b.y, z: res.proof.b.z)
            let g1c = FlutterG1(x: res.proof.c.x, y: res.proof.c.y, z: res.proof.c.z)
            let circomProof = FlutterCircomProof(
                  a: g1a, b: g2b, c: g1c, `protocol`: res.proof.protocol, curve: res.proof.curve)
            let circomProofResult = FlutterCircomProofResult(proof: circomProof, inputs: res.inputs)
            let resultMap: [String: Any] = [
               "proof": [
                  "a": [
                     "x": circomProofResult.proof.a.x,
                     "y": circomProofResult.proof.a.y,
                     "z": circomProofResult.proof.a.z,
                  ],
                  "b": [
                     "x": circomProofResult.proof.b.x,
                     "y": circomProofResult.proof.b.y,
                     "z": circomProofResult.proof.b.z,
                  ],
                  "c": [
                     "x": circomProofResult.proof.c.x,
                     "y": circomProofResult.proof.c.y,
                     "z": circomProofResult.proof.c.z,
                  ],
                  "protocol": circomProofResult.proof.protocol,
                  "curve": circomProofResult.proof.curve,
               ],
               "inputs": circomProofResult.inputs,
            ]
            return resultMap
         }

         // convert the Flutter proofs to be used in mopro bindings
         func convertCircomProofResult(proof: [String: Any]) -> CircomProofResult {
            let proofMap = proof["proof"] as! [String: Any]
            let aMap = proofMap["a"] as! [String: String]
            let g1a = G1(x: aMap["x"] ?? "0", y: aMap["y"] ?? "0", z: aMap["z"] ?? "1")
            let bMap = proofMap["b"] as! [String: [String]]
            let g2b = G2(
               x: bMap["x"] ?? ["1", "0"], y: bMap["y"] ?? ["1", "0"], z: bMap["z"] ?? ["1", "0"])
            let cMap = proofMap["c"] as! [String: String]
            let g1c = G1(x: cMap["x"] ?? "0", y: cMap["y"] ?? "0", z: cMap["z"] ?? "1")
            let circomProof = CircomProof(
               a: g1a, b: g2b, c: g1c, `protocol`: proofMap["protocol"] as! String,
               curve: proofMap["curve"] as! String)
            let circomProofResult = CircomProofResult(
               proof: circomProof, inputs: proof["inputs"] as! [String])
            return circomProofResult
         }

        ```
        </details>

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

         case "verifyCircomProof":
         guard let args = call.arguments as? [String: Any],
            let zkeyPath = args["zkeyPath"] as? String,
            let proof = args["proof"] as? [String: Any],
            let proofLib = args["proofLib"] as? ProofLib
         else {
            result(FlutterError(code: "ARGUMENT_ERROR", message: "Missing arguments", details: nil))
            return
         }

         do {
            let circomProofResult = convertCircomProofResult(proof: proof)
            // Call the function from mopro.swift
            let valid = try verifyCircomProof(
               zkeyPath: zkeyPath, proofResult: circomProofResult, proofLib: proofLib)

            // Return the proof and inputs as a map supported by the StandardMethodCodec
            result(valid)
         } catch {
            result(
               FlutterError(
                  code: "PROOF_VERIFICATION_ERROR", message: "Failed to verify proof",
                  details: error.localizedDescription))
         }
      }
   }
   // ...
}
```

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
      <details>
            <summary>Circom types in `/mopro_flutter_plugin/android/src/main/kotlin/com/example/mopro_flutter_plugin/MoproFlutterPlugin.kt`</summary>
         ```kotlin title="/mopro_flutter_plugin/android/src/main/kotlin/com/example/mopro_flutter_plugin/MoproFlutterPlugin.kt"
         class FlutterG1(x: String, y: String, z: String) {
            val x = x
            val y = y
            val z = z
         }

         class FlutterG2(x: List<String>, y: List<String>, z: List<String>) {
            val x = x
            val y = y
            val z = z
         }

         class FlutterCircomProof(a: FlutterG1, b: FlutterG2, c: FlutterG1, protocol: String, curve: String) {
            val a = a
            val b = b
            val c = c
            val protocol = protocol
            val curve = curve
         }

         class FlutterCircomProofResult(proof: FlutterCircomProof, inputs: List<String>) {
            val proof = proof
            val inputs = inputs
         }
         ```

      </details>

-   Define helper functions to bridge types between the Mopro bindings and the Flutter framework:

      <details>
            <summary>Helper functions in `/mopro_flutter_plugin/android/src/main/kotlin/com/example/mopro_flutter_plugin/MoproFlutterPlugin.kt`</summary>
        ```kotlin title="/mopro_flutter_plugin/android/src/main/kotlin/com/example/mopro_flutter_plugin/MoproFlutterPlugin.kt"
         // convert the mopro proofs to be exposed to Flutter framework
         fun convertCircomProof(res: CircomProofResult): Map<String, Any> {
            val g1a = FlutterG1(res.proof.a.x, res.proof.a.y, res.proof.a.z)
            val g2b = FlutterG2(res.proof.b.x, res.proof.b.y, res.proof.b.z)
            val g1c = FlutterG1(res.proof.c.x, res.proof.c.y, res.proof.c.z)
            val circomProof = FlutterCircomProof(g1a, g2b, g1c, res.proof.protocol, res.proof.curve)
            val circomProofResult = FlutterCircomProofResult(circomProof, res.inputs)
            // Convert to Map before sending
            val resultMap = mapOf(
               "proof" to mapOf(
                  "a" to mapOf(
                     "x" to circomProofResult.proof.a.x,
                     "y" to circomProofResult.proof.a.y,
                     "z" to circomProofResult.proof.a.z
                  ),
                  "b" to mapOf(
                     "x" to circomProofResult.proof.b.x,
                     "y" to circomProofResult.proof.b.y,
                     "z" to circomProofResult.proof.b.z
                  ),
                  "c" to mapOf(
                     "x" to circomProofResult.proof.c.x,
                     "y" to circomProofResult.proof.c.y,
                     "z" to circomProofResult.proof.c.z
                  ),
                  "protocol" to circomProofResult.proof.protocol,
                  "curve" to circomProofResult.proof.curve
               ),
               "inputs" to circomProofResult.inputs
            )
            return resultMap
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
            val bMap = proofMap["b"] as Map<String, Any>
            val g2b = G2(
               bMap["x"] as List<String>,
               bMap["y"] as List<String>,
               bMap["z"] as List<String>
            )
            val cMap = proofMap["c"] as Map<String, Any>
            val g1c = G1(
               cMap["x"] as String,
               cMap["y"] as String,
               cMap["z"] as String
            )
            val circomProof = CircomProof(
               g1a,
               g2b,
               g1c,
               proofMap["protocol"] as String,
               proofMap["curve"] as String
            )
            val circomProofResult = CircomProofResult(circomProof, proofResult["inputs"] as List<String>)
            return circomProofResult
         }
        ```
        </details>

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
      } else if (call.method == "verifyCircomProof") {
         val zkeyPath = call.argument<String>("zkeyPath") ?: return result.error(
            "ARGUMENT_ERROR",
            "Missing zkeyPath",
            null
         )

         val proof = call.argument<Map<String, Any>>("proof") ?: return result.error(
            "ARGUMENT_ERROR",
            "Missing proof",
            null
         )

         val proofLibIndex = call.argument<Int>("proofLib") ?: return result.error(
            "ARGUMENT_ERROR",
            "Missing proofLib",
            null
         )

         val proofLib = if (proofLibIndex == 0) ProofLib.ARKWORKS else ProofLib.RAPIDSNARK

         val circomProofResult = convertCircomProofResult(proof)
         val res = verifyCircomProof(zkeyPath, circomProofResult, proofLib)
         result.success(res)
      } else {
         result.notImplemented()
      }
  }
   // ...
}
```

## 4. Define Dart APIs

:::info
Please refer to [flutter-app](https://github.com/zkmopro/flutter-app) to see the latest update.
:::

-   Define the types for the native module. Add the following types in the file `mopro_flutter_plugin/lib/mopro_types.dart`:

   <details>
      <summary>Types in `/mopro_flutter_plugin/lib/mopro_types.dart`</summary>
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

      class G2Point {
         final List<String> x;
         final List<String> y;
         final List<String> z;

         G2Point(this.x, this.y, this.z);

         @override
         String toString() {
            return "G2Point(\nx: $x, \ny: $y, \nz: $z)";
         }
      }

      class ProofCalldata {
         final G1Point a;
         final G2Point b;
         final G1Point c;
         final String protocol;
         final String curve;

         ProofCalldata(this.a, this.b, this.c, this.protocol, this.curve);

         @override
         String toString() {
            return "ProofCalldata(\na: $a, \nb: $b, \nc: $c, \nprotocol: $protocol, \ncurve: $curve)";
         }
      }

      enum ProofLib { arkworks, rapidsnark }

      class CircomProofResult {
         final ProofCalldata proof;
         final List<String> inputs;

         CircomProofResult(this.proof, this.inputs);

         factory CircomProofResult.fromMap(Map<Object?, Object?> proofResult) {
            var proof = proofResult["proof"] as Map<Object?, Object?>;
            var inputs = proofResult["inputs"] as List;
            var a = proof["a"] as Map<Object?, Object?>;
            var b = proof["b"] as Map<Object?, Object?>;
            var c = proof["c"] as Map<Object?, Object?>;

            var g1a = G1Point(a["x"] as String, a["y"] as String, a["z"] as String);
            var g2b = G2Point((b["x"] as List).cast<String>(),
               (b["y"] as List).cast<String>(), (b["z"] as List).cast<String>());
            var g1c = G1Point(c["x"] as String, c["y"] as String, c["z"] as String);
            return CircomProofResult(
               ProofCalldata(g1a, g2b, g1c, proof["protocol"] as String,
                  proof["curve"] as String),
               inputs.cast<String>());
         }

         Map<String, dynamic> toMap() {
            return {
               "proof": {
                  "a": {"x": proof.a.x, "y": proof.a.y, "z": proof.a.z},
                  "b": {"x": proof.b.x, "y": proof.b.y, "z": proof.b.z},
                  "c": {"x": proof.c.x, "y": proof.c.y, "z": proof.c.z},
                  "protocol": proof.protocol,
                  "curve": proof.curve
               },
               "inputs": inputs
            };
         }
      }
      ```

   </details>

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
    import 'package:mopro_flutter/mopro_flutter.dart';
    import 'package:mopro_flutter/mopro_types.dart';

    final _moproFlutterPlugin = MoproFlutter();

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
    -   Annotate your function with `#[uniffi::export]` (See the [Rust setup](setup/rust-setup.md#setup-any-rust-project) guide for details).<br/>
        Once exported, the function will be available across all supported platforms.
