# Android Setup

This tutorial will guide you through integrating the Android bindings into an Android Studio project. Before you begin, make sure you’ve completed the ["Getting Started - 3. Mopro build"](/docs/getting-started.md#3-build-bindings) process with selecting **Android** platform and have the `MoproAndroidBindings` folder ready:

```sh
MoproAndroidBindings
├── jniLibs
│   ├── arm64-v8a
│   ├── armeabi-v7a
│   ├── x86
│   └── x86_64
└── uniffi
    └── mopro
        └── mopro.kt
```

Watch the demo video below for a step-by-step guide to integrating the bindings into Android Studio, or follow the written instructions that follow.

<p align="center">
<iframe width="560" height="315" src="https://www.youtube.com/embed/r6WolEEHuMw?si=TZbYI8WUAd2Qr5p7" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>
</p>

:::info
In this example, we use Circom circuits and their corresponding `.zkey` files. The process is similar for other provers.
:::

## 0. Initialize an Android Studio project

We will create an android app through Android Studio. If you already have an app project, you can skip this step. We'll do `File` -> `New` -> `New Project` and create an Empty Activity. We suggest putting this Android project inside the rust project folder created above.

<p align="center">
![create an android app project](/img/android-example-1.png)
</p>

Your android project should be opened now.

:::info
Please make sure you choose the **Android** view like this.
![android directory view](/img/android-example-5.png)
:::

## 1. Add dependencies

Then add [jna](https://github.com/java-native-access/jna) to `app/build.gradle.kts`

```kts title="app/build.gradle.kts"
dependencies {
   ...
   implementation("net.java.dev.jna:jna:5.13.0@aar")
   ...
}
```

<p align="center">
![add jna dependency](/img/android-example-2.png)
</p>

Sync gradle with **File -> Sync Project with Gradle Files**, or press

<p align="center">
![android sync gradle](/img/android-example-4.png)
</p>

## 2. Add the `MoproAndroidBindings` folder into the project

Copy folders:

1. Move the `MoproAndroidBindings/jniLibs/` folder into `app/src/main/jniLibs/`. For example:

    ```sh
    cp -r MoproAndroidBindings/jniLibs android/app/src/main
    ```

2. Move the `MoproAndroidBindings/uniffi/mopro/mopro.kts` file into `app/src/main/java/uniffi/mopro/mopro.kt`. For example:
    ```sh
    cp -r MoproAndroidBindings/uniffi android/app/src/main/java
    ```

The folder structure will be like

```sh title="app/src"
├── main
│   ├── AndroidManifest.xml
│   ├── java
│   │   ├── com
│   │   │   └── example
│   │   │       └── YOUR_APP
│   │   └── uniffi
│   │       └── mopro
│   │           └── mopro.kt
│   ├── jniLibs
│   │   ├── arm64-v8a
│   │   ├── armeabi-v7a
│   │   ├── x86
│   │   └── x86_64
│   ...
```

<p align="center">
![android bindings](/img/android-bindings.png)
</p>

## 3. Place proving keys into the project

Create an asset folder: `File` -> `New` -> `Folder` -> `Assets Folder`. <br/>
Paste the keys in the assets folder.

<p align="center">
![android paste zkey](/img/android-example-3.png)
</p>

:::warning
Although relative paths may work locally in Rust, the proving keys should be copied into the project to ensure they are accessible by the mobile app.
:::

## 4. Proving from the app

In your project, there should be a file named `MainActivity.kt`

:::info
It should be under `app/src/main/java/com/example/YOUR_APP/MainActivity.kt`
:::

Import the following functions:

```kotlin title="MainActivity.kt"
import androidx.compose.runtime.*
import android.content.Context
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.padding
import androidx.compose.ui.unit.dp
import androidx.compose.material3.Button
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import kotlinx.coroutines.launch
import uniffi.mopro.*
import java.io.File
import java.io.FileOutputStream
import java.io.IOException
```

This will make the proving functions `generateCircomProof` available in this module and also help to load zkey.

In the `MainActivity.kt`, make your `setContent` function look like this:

```kotlin title="MainActivity.kt"
    setContent {
        // A surface container using the 'background' color from the theme
        Surface(
            modifier = Modifier.fillMaxSize(),
            color = MaterialTheme.colorScheme.background
        ) {
            MainScreen(this)
        }
    }
```

Add a private function to load zkey. It is used to copy a file from the app's assets directory to the app's internal storage so that we can read the path of the zkey file.

```kotlin title="MainActivity.kt"
private fun copyAssetToInternalStorage(context: Context, assetFileName: String): String? {
    val file = File(context.filesDir, assetFileName)
    return try {
        context.assets.open(assetFileName).use { inputStream ->
            FileOutputStream(file).use { outputStream ->
                val buffer = ByteArray(1024)
                var length: Int
                while (inputStream.read(buffer).also { length = it } > 0) {
                    outputStream.write(buffer, 0, length)
                }
                outputStream.flush()
            }
        }
        file.absolutePath
    } catch (e: IOException) {
        e.printStackTrace()
        null
    }
}
```

At the bottom of this file we'll create a view with a function to generate a proof. In this example we're going to prove a simple circuit that accepts two inputs named `a` and `b` and generates an output `c`.

```kotlin title="MainActivity.kt"
@Composable
fun MainScreen(context: Context) {
    val coroutineScope = rememberCoroutineScope()
    var res by remember { mutableStateOf( "Proof: ") }

    Column(
        modifier = Modifier
            .fillMaxSize()
            .padding(16.dp)
    ) {
        Button(onClick = {
            coroutineScope.launch {
                val assetFilePath = copyAssetToInternalStorage(context, "multiplier2_final.zkey")
                assetFilePath?.let { path ->
                    val input_str: String = "{\"b\":[\"5\"],\"a\":[\"3\"]}"
                    res = generateCircomProof(path, input_str, ProofLib.ARKWORKS).toString()
                    println(res)
                }
            }
        }) {
            Text(text = "Generate Proof")
        }
        Text(text=res)
    }
}
```

<details>
  <summary>Full `MainActivity.kt` (simplified)</summary>

```kotlin
package com.example.moproandroidapp // Your application ID

import android.content.Context
import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Button
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import java.io.File
import java.io.FileOutputStream
import java.io.IOException
import kotlinx.coroutines.launch
import uniffi.mopro.*
class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContent {
            // A surface container using the 'background' color from the theme
            Surface(
                    modifier = Modifier.fillMaxSize(),
                    color = MaterialTheme.colorScheme.background
            ) { MainScreen(this) }
        }
    }
}

@Composable
fun MainScreen(context: Context) {
    val coroutineScope = rememberCoroutineScope()
    var res by remember {
        mutableStateOf(
            "Proof: "
        )
    }

    Column(
        modifier = Modifier
            .fillMaxSize()
            .padding(16.dp)
    ) {
        Button(onClick = {
            coroutineScope.launch {
                val assetFilePath = copyAssetToInternalStorage(context, "multiplier2_final.zkey")
                assetFilePath?.let { path ->
                    val input_str: String = "{\"b\":[\"5\"],\"a\":[\"3\"]}"
                    res = generateCircomProof(path, input_str, ProofLib.ARKWORKS).toString()
                    println(res)
                }
            }
        }) {
            Text(text = "Generate Proof")
        }
        Text(text=res)
    }
}

private fun copyAssetToInternalStorage(context: Context, assetFileName: String): String? {
    val file = File(context.filesDir, assetFileName)
    return try {
        context.assets.open(assetFileName).use { inputStream ->
            FileOutputStream(file).use { outputStream ->
                val buffer = ByteArray(1024)
                var length: Int
                while (inputStream.read(buffer).also { length = it } > 0) {
                    outputStream.write(buffer, 0, length)
                }
                outputStream.flush()
            }
        }
        file.absolutePath
    } catch (e: IOException) {
        e.printStackTrace()
        null
    }
}

```

</details>

You should now be able to run the Android app (`^`+`R` or `ctrl`+`R`) on the simulator or a device and build a proof. The app should log the proof. For a more complete example including other provers and verification check [here](https://github.com/zkmopro/mopro/blob/63d6cbaf1bdbcb089f889f86b2a6a0443c6a8679/test-e2e/android/app/src/main/java/com/mopro/mopro_app/MainActivity.kt).

<p align="center">
![android run app](/img/android-example-6.jpg)
</p>

## 5. What's next?

-   **Update your ZK circuits** as needed. After making changes, be sure to run:

    ```sh
    mopro build
    mopro update
    ```

    :::warning
    `mopro update` only works if the Android project was created _within_ the Rust project directory during mopro init. Otherwise, you can manually update the bindings by following [Step 2](#2-add-the-moproandroidbindings-folder-into-the-project).
    :::

    This ensures the bindings are regenerated and reflect your latest updates.

-   **Build your mobile app frontend** according to your business logic and user flow.
-   **Expose additional Rust functionality:**
    If a function is missing in Swift, Kotlin, React Native, or Flutter, you can:

    -   Add the required Rust crate in `Cargo.toml`
    -   Annotate your function with `#[uniffi::export]` (See the [Rust setup](setup/rust-setup.md#setup-any-rust-project) guide for details).<br/>
        Once exported, the function will be available across all supported platforms.
