# Android

## Getting started with a new project

1. Install `mopro` cli. See [Gettin Started](../getting-started#install-dependencies).
2. Create a new project

```sh
mopro init --platforms android \
cd mopro-example-app
```

3. Prepare circuits

```sh
mopro prepare
```

4. Build the project

```sh
mopro build --platforms android
```

5. Open the project in Android Studio

```sh
open android -a Android\ Studio
```

## Getting started with exported bindings

1. Install `mopro` cli. See [Gettin Started](../getting-started#install-dependencies).
2. Prepare circuits

```sh
mopro prepare
```

3. Build the project

```sh
mopro build --platforms android
```

4. Export bindings

```sh
mopro export-bindings --platforms android --destination out
```

5. add dependencies in `app/build.gradle.kts`

```kotlin
dependencies {
   ...
   implementation("net.java.dev.jna:jna:5.13.0@aar")
   ...
}
```

6. Sync gradle (shift+command+O)
7. Drag and drop folders. <br/>
   Move the `out/android/jniLibs/` folder into `app/src/main/jniLibs/`.<br/>
   Move the `out/android/uniffi/mopro/mopro.kts` file into `app/src/main/java/uniffi/mopro/mopro.kt`.<br/>
   ![android bindings](/img/android-bindings.png)
8. Use `mopro` by

```kotlin
import uniffi.mopro.initializeMopro

func initialize(){
	initializeMopro()
}
```

## Kotlin API

### `MoproCircom`

Initialize a circom object. <br/>

Usage:

```kotlin
var moproCircom = MoproCircom()
```

### `initialize`

Initializes the instance with the given `zkeyPath` and `wasmPath`.

```kotlin
@Throws(MoproException::class)
fun `initialize`(
    `zkeyPath`: String,
    `wasmPath`: String,
)
```

Usage:

```kotlin
moproCircom.initialize(zkeyPath, wasmPath)
```

### `generateProof`

Generates a proof based on the provided circuit inputs.

```kotlin
@Throws(MoproException::class)
fun `generateProof`(`circuitInputs`: Map<String, List<String>>): GenerateProofResult
```

Usage:

```kotlin
val inputs = mutableMapOf<String, List<String>>()
inputs["a"] = listOf("3")
inputs["b"] = listOf("5")
var generateProofResult = moproCircom.generateProof(inputs)
```

### `verifyProof`

Verifies the provided proof against the given inputs.

```kotlin
@Throws(MoproException::class)
fun `verifyProof`(
    `proof`: ByteArray,
    `publicInput`: ByteArray,
): Boolean
```

Usage:

```kotlin
var isValid = moproCircom.verifyProof(
	generateProofResult.proof,
	generateProofResult.inputs
)
```

### `generateProof2`

Generates a proof based on the provided circuit inputs.<br/>
The zkey and wasm are precompiled during `cargo build`. You can specify the [mopro-config.toml](configuration) to build the default circuits.

```kotlin
@Throws(MoproException::class)
fun `generateProof2`(`circuitInputs`: Map<String, List<String>>): GenerateProofResult
```

### `verifyProof2`

Verifies the provided proof against the given inputs.<br/>
The zkey and wasm are precompiled during `cargo build`. You can specify the [mopro-config.toml](configuration) to build the default circuits.

```kotlin
@Throws(MoproException::class)
fun `verifyProof2`(
    `proof`: ByteArray,
    `publicInput`: ByteArray,
): Boolean
```

### `toEthereumInputs`

Convert public inputs data to a string array.

```kotlin
fun `toEthereumInputs`(`inputs`: ByteArray): List<String>
```

### `toEthereumProof`

Convert proof data to a proof structure which can be submitted to a verifier contract.

```kotlin
fun `toEthereumProof`(`proof`: ByteArray): ProofCalldata
```
