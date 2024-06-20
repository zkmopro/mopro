# iOS

## Getting started with a new project

1. Install `mopro` cli. See [Gettin Started](../getting-started#install-dependencies).
2. Create a new project

```sh
mopro init --platforms ios \
cd mopro-example-app
```

3. Prepare circuits

```sh
mopro prepare
```

4. Build the project

```sh
mopro build --platforms ios
```

5. Open the project in xcode

```sh
open ios/ExampleApp/ExampleApp.xcworkspace
```

## Getting started with exported bindings

1. Install `mopro` cli. See [Gettin Started](../getting-started#install-dependencies).
2. Prepare circuits

```sh
mopro prepare
```

3. Build the project

```sh
mopro build --platforms ios
```

4. Export bindings

```sh
mopro export-bindings --platforms ios --destination out
```

5. Create an `xcframework` with `xcodebuild`

e.g. for simulator

```sh
xcodebuild -create-xcframework \
	-library out/ios/aarch64-apple-ios-sim/release/libmopro_ffi.a \
	-headers out/ios/Bindings \
	-output "out/ios/Mopro.xcframework"
```

e.g. for both simulator and device<br/>
Please specify `ios_device_type` in [mopro-config.toml](configuration) to build for both `device` and `simulator`.

```sh
xcodebuild -create-xcframework \
	-library out/ios/aarch64-apple-ios-sim/release/libmopro_ffi.a \
	-headers out/ios/Bindings \
	-library out/ios/aarch64-apple-ios/release/libmopro_ffi.a \
	-headers out/ios/Bindings \
	-output "out/ios/Mopro.xcframework"
```

:::info
Ref: [Building an iOS App with Rust Using UniFFI](https://forgen.tech/en/blog/post/building-an-ios-app-with-rust-using-uniffi)
:::

6. Import both the XCFramework `Mopro.xcframework` and the Swift file bindings `Bindings/mopro.swift` files into your project (drag and drop should work).
7. Use `moproFFI` in swift like

```swift
import moproFFI

...
try initializeMopro()
...
```

## Swift API

### `MoproCircom`

Initialize a circom object. <br/>

Usage:

```swift
let moproCircom = MoproCircom()
```

### `initialize`

Initializes the instance with the given `zkeyPath` and `wasmPath`.

```swift
func initialize(zkeyPath: String, wasmPath: String)  throws
```

Usage:

```swift
try moproCircom.initialize(zkeyPath: zkeyPath, wasmPath: wasmPath)
```

### `generateProof`

Generates a proof based on the provided circuit inputs.

```swift
func generateProof(circuitInputs: [String: [String]])  throws -> GenerateProofResult
```

Usage:

```swift
var inputs = [String: [String]]()
let a = 3
let b = 5
inputs["a"] = [String(a)]
inputs["b"] = [String(b)]
let generateProofResult = try moproCircom.generateProof(circuitInputs: inputs)
```

### `verifyProof`

Verifies the provided proof against the given inputs.

```swift
func verifyProof(proof: Data, publicInput: Data)  throws -> Bool
```

Usage:

```swift
let isValid = try moproCircom.verifyProof(
    proof: generateProofResult.proof,
    publicInput: generateProofResult.inputs
)
```

### `generateProof2`

Generates a proof based on the provided circuit inputs.<br/>
The zkey and wasm are precompiled during `cargo build`. You can specify the [mopro-config.toml](configuration) to build the default circuits.

```swift
func generateProof2(circuitInputs: [String: [String]]) throws -> GenerateProofResult
```

### `verifyProof2`

Verifies the provided proof against the given inputs.<br/>
The zkey and wasm are precompiled during `cargo build`. You can specify the [mopro-config.toml](configuration) to build the default circuits.

```swift
func verifyProof2(proof: Data, publicInput: Data) throws -> Bool
```

### `toEthereumInputs`

Convert public inputs data to a string array.

```swift
func toEthereumInputs(inputs: Data)  -> [String]
```

### `toEthereumProof`

Convert proof data to a proof structure which can be submitted to a verifier contract.

```swift
func toEthereumProof(proof: Data)  -> ProofCalldata
```
