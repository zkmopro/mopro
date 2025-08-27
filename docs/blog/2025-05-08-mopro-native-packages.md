---
slug: mopro-native-packages
title: Integrating Mopro Native Packages Across Mobile Platforms
authors:
    name: Moven Tsai
    title: Developer on the Mopro Team
    url: https://github.com/moven0831
    image_url: https://github.com/moven0831.png
tags: [multi-platform, native-package, zkEmail, noir]
---

> **TL; DR** Mopro now ships pre-built native packages for Swift (iOS), Kotlin (Android), Flutter, and React Native.  
> Just one import and one build. Proving made simple!

## Announcing Mopro Native Packages

We're excited to launch Mopro native packages, enabling developers to effortlessly generate and verify zero-knowledge proofs (ZKPs) directly on mobile devices. These native packages leverage Rust's performance and seamlessly integrate with popular mobile frameworks. Built using the Mopro CLI, they're available for direct import via each platform's package manager.

You can also easily create your own customized native packages by following [zkmopro-docs](https://zkmopro.org/docs/getting-started).

| Framework             | Package Manager             | Default Packages                                                                    | zkEmail Packages via Mopro                                                              |
| -------------------- | --------------------------- | ----------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------- |
| **Swift (iOS)**      | Xcode / SwiftPM / CocoaPods | [mopro-swift-package](https://github.com/zkmopro/mopro-swift-package)               | [zkemail-swift-package](https://github.com/zkmopro/zkemail-swift-package)               |
| **Kotlin (Android)** | JitPack                     | [mopro-kotlin-package](https://github.com/zkmopro/mopro-kotlin-package)             | [zkemail-kotlin-package](https://github.com/zkmopro/zkemail-kotlin-package)             |
| **Flutter**          | pub.dev                     | [mopro_flutter_package](https://github.com/zkmopro/mopro_flutter_package)           | [zkemail_flutter_package](https://github.com/zkmopro/zkemail_flutter_package)           |
| **React Native**     | npm / yarn                  | [mopro-react-native-package](https://github.com/zkmopro/mopro-react-native-package) | [zkemail-react-native-package](https://github.com/zkmopro/zkemail-react-native-package) |

This blog provides a quick guide on integrating these packages into your projects, outlines how we built them (so you can customize your own), addresses challenges we overcame, and highlights future developments. Let's get started!

## Import, Build, Prove - That's It

Mopro's native packages simplify the integration process dramatically. Unlike the traditional approach that requires crafting APIs, generating bindings, and manually building app templates, these pre-built packages allow developers to import them directly via package managers and immediately begin developing application logic.

For ZK projects, converting your Rust-based solutions into mobile-native packages is straightforward with Mopro. Our guide on ["How to Build the Package"](https://github.com/zkmopro/mopro-swift-package?tab=readme-ov-file#how-to-build-the-package) explains the process clearly.

For instance, our zkEmail native packages were created by first [defining ZK proving and verification APIs in Rust](https://github.com/zkmopro/mopro-zkemail-nr/blob/main/src/lib.rs), generating bindings with `mopro build`, and embedding these into native packages. The circuit is the header-only proof from [zkemail.nr_header_demo](https://github.com/Mach-34/zkemail.nr_header_demo).

Here's how zkEmail performs on Apple M3 chips:

| zkEmail Operation | iOS, Time (ms) | Android, Time (ms) |
| ----------------- | -------------- | ------------------ |
| Proof Generation  | 1,309           | 3,826               |
| Verification      | 962             | 2,857               |

<p align="center">
    <table>
    <tr>
        <td align="center">
        <a href="/img/zkemail-flutter-app-ios.png" target="_blank" rel="noopener noreferrer">
            <img src="/img/zkemail-flutter-app-ios.png" alt="iOS zkEmail App Example" width="300"/>
        </a>
        <br />
        <sub><b>iOS</b></sub>
        </td>
        <td align="center">
        <a href="/img/zkemail-flutter-app-android.png" target="_blank" rel="noopener noreferrer">
            <img src="/img/zkemail-flutter-app-android.png" alt="Android zkEmail App Example" width="300"/>
        </a>
        <br />
        <sub><b>Android</b></sub>
        </td>
    </tr>
    </table>
    <p align="center">
        <em>Flutter App for iOS & Android zkEmail Example</em>
    </p>
</p>

Notice that, with Mopro and the use of [Noir-rs](https://github.com/zkmopro/noir-rs), we port zkEmail into native packages while keeping the proof size align with Noir's Barretenberg backend CLI. It transfers the API logic directly to mobile platforms with no extra work or glue code needed!

### How it worked before Mopro

Previously, integrating ZKPs into mobile applications involved more manual work and platform-specific implementations. For example, developers might have used solutions like:

-   **Swoir:** [https://github.com/Swoir/Swoir/tree/main](https://github.com/Swoir/Swoir/tree/main)
-   **noir-android:** [https://github.com/madztheo/noir_android/tree/main](https://github.com/madztheo/noir_android/tree/main)

These approaches often required developers to handle bridging code and manage dependencies separately for each platform, unlike the streamlined process Mopro now offers.

With Mopro, developers can leverage pre-built native packages and import them directly via package managers. This, combined with automated binding generation, significantly reduces the need for manual API crafting and platform-specific glue code.

While developers still write their application logic using platform-specific languages, Mopro simplifies the integration of core ZK functionalities, especially when leveraging Rust's extensive cryptography ecosystem.

## Under The Hood

Developing native packages involved tackling several technical challenges to ensure smooth and efficient operation across different platforms.

This section dives into two key challenges we addressed:
1. Optimizing static library sizes for iOS to manage package distribution and download speeds.
2. Ensuring compatibility with Android's release mode to prevent runtime errors due to code shrinking.

### Optimizing Static Library Sizes for iOS

#### Why Static Linking?

UniFFI exports Swift bindings as a static archive (`libMoproBindings.a`). Static linking ensures all Rust symbols are available at link-time, simplifying Xcode integration. However, it bundles all Rust dependencies (Barretenberg Backend, rayon, big-integer math), resulting in larger archive sizes.

#### Baseline Size

The full build creates an archive around **≈ 153 MB** in size. When uploading files over 100 MB to GitHub, Git LFS takes over by replacing the file with a text pointer in the repository while storing the actual content on a remote server like GitHub.com. This setup can cause issues for package managers that try to fetch the package directly from a GitHub URL for a release publish.

While uploading large files may be acceptable for some package management platforms or remote servers like Cloudflare R2, the large file size slows down:

- CocoaPods or SwiftPM downloads
- CI cache recovery
- Cloning the repository, especially on slower connections

#### Our Solution: Zip & Unzip Strategy

To keep development fast and responsive, we compress the entire `MoproBindings.xcframework` before uploading it to GitHub and publishing it to CocoaPods, reducing its size to about **≈ 41 MB**.

We also found that by customizing `script_phase` in the `.podspec` (check our implementation in [`ZKEmailSwift.podspec`](https://github.com/zkmopro/zkemail-swift-package/blob/b5c3a94f8580b0332ced2c8409a1017530a56e38/ZKEmailSwift.podspec#L93-L103)), we can unzip the bindings during pod install. This gives us the best of both worlds: (1) smaller packages for distribution and (2) full compatibility with Xcode linking. The added CPU cost is minor compared to the time saved on downloads.

#### Comparison With Android

On Android, dynamic `.so` libraries (around 22 MB in total) are used, with symbols loaded lazily at runtime to keep the package size small. In contrast, because iOS's constraint on third-party Rust dynamic libraries in App Store builds, static linking with compression is currently the most viable option, to the best of our knowledge.

### Ensuring Android Release Mode Compatibility

Another challenge we tackled was ensuring compatibility with Android's release mode. By default, Android's release build process applies [code shrinking and obfuscation](https://developer.android.com/build/shrink-code) to optimize app size. While beneficial for optimization, this process caused a `java.lang.UnsatisfiedLinkError` for Mopro apps.

The root cause was that code shrinking interfered with [JNA (Java Native Access)](https://mozilla.github.io/uniffi-rs/latest/kotlin/gradle.html#jna-dependency), a crucial dependency for UniFFI, which we use for Rust-to-Kotlin bindings. The shrinking process was removing or altering parts of JNA that were necessary for the bindings to function correctly, leading to the `UnsatisfiedLinkError` when the app tried to call the native Rust code.

#### The Fix: Adjusting Gradle Build Configurations

Our solution, as detailed in [GitHub Issue #416](https://github.com/zkmopro/mopro/issues/416), involves a configuration adjustment in the consuming application's `android/build.gradle.kts` file (or `android/app/build.gradle` for older Android projects). Developers using Mopro need to explicitly disable code and resource shrinking for their release builds:

```kotlin
android {
    // ...
    buildTypes {
        getByName("release") {
            // Disables code shrinking, obfuscation, and optimization for
            // your project's release build type.
            minifyEnabled = false
            // Disables resource shrinking, which is performed by the
            // Android Gradle plugin.
            shrinkResources = false
        }
    }
}
```

#### Impact and Future Considerations

This configuration ensures that JNA and, consequently, the UniFFI bindings remain intact, allowing Mopro-powered Android apps to build and run successfully in release mode. This approach aligns with recommendations found in the official Flutter documentation for handling [similar issues](https://docs.flutter.dev/deployment/android#shrink-your-code-with-r8). While this increases the final app size slightly, it guarantees the stability and functionality of the native ZK operations. We are also actively exploring ways to refine this in the future to allow for optimized builds without compromising JNA's functionality.

## The Road Ahead

### a. Manual Tweaks for Cross-Platform Frameworks

Cross-platform frameworks like React Native and Flutter require additional glue code to define modules, as they straddle multiple runtimes. Each layer needs its own integration.

For example, in our [zkEmail React Native package](https://github.com/zkmopro/zkemail-react-native-package), we use three separate wrappers.

- \[TypeScript\] [`MoproReactNativePackageModule.ts`](https://github.com/zkmopro/zkemail-react-native-package/blob/main/src/MoproReactNativePackageModule.ts): declares the public API and lets the React Native code-gen load the native module.
- \[Swift\] [`MoproReactNativePackageModule.swift`](https://github.com/zkmopro/zkemail-react-native-package/blob/main/ios/MoproReactNativePackageModule.swift): loads bindings into Objective-C–discoverable classes.
- \[Kotlin\] [`MoproReactNativePackageModule.kt`](https://github.com/zkmopro/zkemail-react-native-package/blob/main/android/src/main/java/expo/modules/moproreactnativepackage/MoproReactNativePackageModule.kt): loads bindings and bridges via JNI.

Similarly, for our [zkEmail Flutter package](https://github.com/zkmopro/zkemail_flutter_package), a comparable set of wrappers is employed:

- \[Dart\] [`zkemail_flutter_package.dart`](https://github.com/zkmopro/zkemail_flutter_package/blob/main/lib/zkemail_flutter_package.dart): defines the public Dart API for the Flutter plugin, invoking methods on the native side via platform channels.
- \[Swift\] [`ZkemailFlutterPackagePlugin.swift`](https://github.com/zkmopro/zkemail_flutter_package/blob/main/ios/Classes/ZkemailFlutterPackagePlugin.swift): calls the underlying Rust-generated Swift bindings.
- \[Kotlin\] [`ZkemailFlutterPackagePlugin.kt`](https://github.com/zkmopro/zkemail_flutter_package/blob/main/android/src/main/kotlin/com/zkmopro/zkemail_flutter_package/ZkemailFlutterPackagePlugin.kt): bridges Dart calls to the Rust-generated Kotlin bindings.

### b. Support for Custom Package Names

Initially, we encountered naming conflicts when the same XCFramework was used in multiple Xcode projects. Addressing this to allow fully customizable package names is an ongoing effort.

Initial progress was made with updates in [issue#387](https://github.com/zkmopro/mopro/issues/387) and a partial fix in [PR#404](https://github.com/zkmopro/mopro/pull/404). Further work to complete this feature is being tracked in [issue#413](https://github.com/zkmopro/mopro/issues/413).

## What's Next: Shaping Mopro's Future Together

Currently, the Mopro CLI helps you create app templates via the `mopro create` command, once bindings are generated with `mopro build`.

Our vision is to enhance this by enabling the automatic generation of fully customized native packages. This would include managing all necessary glue code for cross-platform frameworks, potentially through a new command (maybe like `mopro pack`) or by extending existing commands. We believe this will significantly streamline the developer workflow. If you're interested in shaping this feature, we invite you to check out the discussion and contribute your ideas in [issue #419](https://github.com/zkmopro/mopro/issues/419).

By achieving this, we aim to unlock seamless mobile proving capabilities, simplifying adoption for developers leveraging existing ZK solutions or porting Rust-based ZK projects. Your contributions can help us make mobile ZK development more accessible for everyone!

If you find it interesting, feel free to reach out to the Mopro team on Telegram: [@zkmopro](https://t.me/zkmopro), or better yet, dive into the codebase and open a PR! We're excited to see what the community builds with Mopro.

Happy proving!
