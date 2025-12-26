---
title: Swift SDK
---

In this section, you'll learn how to build a Swift SDK using Mopro's native bindings.
The process includes the following steps:

1. [Generate the binding using the Mopro CLI](#generate-the-bindings-using-the-mopro-cli)
2. [Integrate the binding into a Swift Package](#integrate-the-bindings-into-a-swift-package)
3. [How to install the Swift SDK in Xcode](#how-to-install-the-swift-sdk-in-xcode)
4. [How to use the package](#how-to-use-the-package)
5. [How to publish the package](#how-to-publish-the-package)

## Generate the bindings using the Mopro CLI

To get started with building Mopro bindings, refer to the [Getting Started](/docs/getting-started) section.
If you’d like to generate custom bindings for your own circuits or proving schemes, see the guide:
[Rust Setup for Android/iOS Bindings](/docs/setup/rust-setup).

Then you will have a `MoproiOSBindings` in the project directory.

## Integrate the bindings into a Swift Package

1. Clone the SDK template repository:

```sh
git clone https://github.com/zkmopro/mopro-swift-package
```

2. Replace the generated bindings:

Replace the contents of the `Sources/MoproiOSBindings` directory with your own generated bindings. Or, copy your bindings using the following command:

```sh
cp -r ..<PATH/TO/YOUR/BINDINGS>/MoproiOSBindings ./Sources
```

## How to install the Swift SDK in Xcode

### Option 1. Using Xcode

1. Open your project in Xcode.
2. Go to **File > Add package dependencies**.
3. In Search or Enter Package URL (e.g. enter the URL: https://github.com/zkmopro/mopro-swift-package)
4. Choose the version and add the package to your project.

### Option 2: Using Package.swift

Add the following to your `Package.swift` dependencies:

```swift
let package = Package(
    name: "YourSwiftProject",
    // ...
    dependencies: [
        .package(url: "https://github.com/zkmopro/mopro-swift-package", from: "0.3.0") // Or change to your own URL
    ],
    // ...
    targets: [
        .target(
            name: "YourSwiftProject",
            dependencies: [
                .product(name: "MoproFFI", package: "mopro-swift-package")
            ],
        ),
    ]
)
```

### Option 3: Using CocoaPods

1. Add the following to your `Podfile`:

```ruby
pod 'MoproFFI', :git => 'https://github.com/zkmopro/mopro-swift-package.git', :branch => 'main'
```

2. Run the installation command:

```sh
pod install
```

## How to use the package

Here is an example of how to integrate and use this example package

```swift
import MoproFFI // Name of the package

// ...
let generateProofResult = try generateCircomProof(zkeyPath: zkeyPath, circuitInputs: input_str, proofLib: ProofLib.arkworks)
```

Or checkout the [test-e2e](https://github.com/zkmopro/mopro/blob/793626f32ed34dcde382f5f304c301563126bc9d/test-e2e/ios/mopro-test/ContentView.swift#L90) app.

:::warning
The current `mopro-swift-package` supports only the Circom `multiplier2` circuit. <br/>
To use your own circuits, please follow the Getting Started guide and replace the witness function with your own WASM or witness files.
:::

## How to publish the package

If you want to publish a package that can be used by React Native or Flutter, you can distribute it via [CocoaPods](https://cocoapods.org).

To do this, you'll need to update the [`.podspec`](https://github.com/zkmopro/mopro-swift-package/blob/main/MoproFFI.podspec) file accordingly.

Here is an example script to help you release your package on CocoaPods: [release.yml](https://github.com/zkmopro/zkemail-swift-package/blob/main/.github/workflows/release.yml)

## Acknowledgement

This project is heavily inspired by [ezkl-swift-package](https://github.com/zkonduit/ezkl-swift-package) and follows a similar approach for integrating native cryptographic libraries into Swift via a Swift Package.
