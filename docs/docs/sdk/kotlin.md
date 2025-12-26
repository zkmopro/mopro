---
title: Kotlin SDK
---

In this section, you'll learn how to build a Kotlin SDK using Mopro's native bindings.
The process includes the following steps:

1. [Generate the binding using the Mopro CLI](#generate-the-bindings-using-the-mopro-cli)
2. [Integrate the binding into a Kotlin Package](#integrate-the-bindings-into-a-kotlin-package)
3. [How to install the Kotlin SDK via JitPack](#how-to-install-the-kotlin-sdk-via-jitpack)
4. [How to use the package](#how-to-use-the-package)

## Generate the bindings using the Mopro CLI

To get started with building Mopro bindings, refer to the [Getting Started](/docs/getting-started) section.
If you’d like to generate custom bindings for your own circuits or proving schemes, see the guide:
[Rust Setup for Android/iOS Bindings](/docs/setup/rust-setup).

Then you will have a `MoproAndroidBindings` in the project directory.

## Integrate the bindings into a Kotlin Package

1. Clone the SDK template repository:

```sh
git clone https://github.com/zkmopro/mopro-kotlin-package
```

2. Replace the generated bindings:

Replace the bindings directory `MoproAndroidBindings/uniffi` and `MoproAndroidBindings/jniLibs` with your generated files in the following location:

-   `android/app/src/main/java/uniffi`
-   `android/app/src/main/jniLibs`

Alternatively, you can run the following commands to copy your generated bindings into the correct location:

```sh
cp -r PATH/TO/MoproAndroidBindings/uniffi android/app/src/main/java
cp -r PATH/TO/MoproAndroidBindings/jniLibs android/app/src/main
```

## How to install the Kotlin SDK via JitPack

To get this library from GitHub using [JitPack](https://jitpack.io/):

**Step 1.** Add the JitPack repository to your `settings.gradle.kts` at the end of repositories:

```kotlin
dependencyResolutionManagement {
    repositoriesMode.set(RepositoriesMode.FAIL_ON_PROJECT_REPOS)
    repositories {
        mavenCentral()
        maven { url = uri("https://jitpack.io") }
    }
}
```

**Step 2.** Add the dependency to your `build.gradle.kts`:

```kotlin
  dependencies {
      implementation("com.github.zkmopro:mopro-kotlin-package:Tag") // Or change to your own URL
  }
```

Replace `Tag` with the desired release version, e.g. `v0.1.0`. See the [JitPack page](https://jitpack.io/#zkmopro/mopro-kotlin-package) for available versions.

**Note:** If you're using an Android template from `mopro create`, comment out these UniFFI dependencies in your build file to prevent duplicate class errors.

```kotlin
  // // Uniffi
  // implementation("net.java.dev.jna:jna:5.13.0@aar")
  // implementation("org.jetbrains.kotlinx:kotlinx-coroutines-core:1.6.4")
```

## How to use the package

Here is an example of how to integrate and use this example package

```kotlin
import uniffi.mopro.generateCircomProof
import uniffi.mopro.verifyCircomProof
import uniffi.mopro.ProofLib

val inputStr = "{\"b\":[\"5\"],\"a\":[\"3\"]}"
val zkeyPath = "/path/to/multiplier2_final.zkey"
val proof = generateCircomProof(zkeyPath, inputStr, ProofLib.ARKWORKS)
val isValid = verifyCircomProof(zkeyPath, proof, ProofLib.ARKWORKS)
```

Or checkout the [test-e2e](https://github.com/zkmopro/mopro/blob/793626f32ed34dcde382f5f304c301563126bc9d/test-e2e/android/app/src/main/java/com/mopro/mopro_app/MultiplierComponent.kt#L53) app.

:::warning
The current `mopro-kotlin-package` supports only the Circom `multiplier2` circuit. <br/>
To use your own circuits, please follow the Getting Started guide and replace the witness function with your own WASM or witness files.
:::
