# Android Setup

Before starting, ensure you have completed ["Getting Started - 3. Mopro build"](/docs/getting-started.md#3-build-bindings) and selected the android platform during the build process. This step uses the mopro cli to generate the required bindings for your android project.

After successfully completing the build, a folder named **MoproAndroidBindings** will appear in your project directory. The structure of this folder should look like this:

```sh
MoproAndroidBindings
├── jniLibs
│   ├── arm64-v8a
│   │   └── libuniffi_mopro.so
│   ├── armeabi-v7a
│   │   └── libuniffi_mopro.so
│   ├── x86
│   │   └── libuniffi_mopro.so
│   └── x86_64
│       └── libuniffi_mopro.so
└── uniffi
    └── mopro
        └── mopro.kt
```

## Demo video of this tutorial

<p align="center">
<iframe width="560" height="315" src="https://www.youtube.com/embed/r6WolEEHuMw?si=TZbYI8WUAd2Qr5p7" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>
</p>

First we will create an android app through Android Studio. If you already have an app project, you can skip this step. We'll do **File -> New -> New Project** and create an Empty Activity.

![create an android app project](/img/android-example-1.png)

Your android project should be opened now.

:::info
Please make sure you choose the **Android** view like this.
![android directory view](/img/android-example-5.png)
:::

Then add jna to `app/build.gradle.kts`

```kts
dependencies {
   ...
   implementation("net.java.dev.jna:jna:5.13.0@aar")
   ...
}
```

![add jna dependency](/img/android-example-2.png)

Sync gradle with **File -> Sync Project with Gradle Files**, or press

![android sync gradle](/img/android-example-4.png)

Open Finder and drag folders:

1. Move the `MoproAndroidBindings/jniLibs/` folder into `app/src/main/jniLibs/`.
2. Move the `MoproAndroidBindings/uniffi/mopro/mopro.kts` file into `app/src/main/java/uniffi/mopro/mopro.kt`

![android bindings](/img/android-bindings.png)

Create an asset folder: **File -> New -> Folder -> Assets Folder**. <br/>
Paste the zkey in the assets folder.

![android paste zkey](/img/android-example-3.png)

## Proving from the app

In your project, there should be a file named `MainActivity.kt`

> It should be under `app/src/main/java/com/example/YOUR_APP/MainActivity.kt`

Import the following functions:

```kotlin
import android.content.Context
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.padding
import androidx.compose.ui.unit.dp
import androidx.compose.material3.Button
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.runtime.Composable
import androidx.compose.runtime.remember
import androidx.compose.ui.platform.LocalContext
import kotlinx.coroutines.launch
import uniffi.mopro.generateCircomProof
import java.io.File
import java.io.FileOutputStream
import java.io.IOException
import java.io.InputStream
import java.io.OutputStream
```

This will make the proving functions `generateCircomProof` available in this module and also help to load zkey.

In the `MainActivity.kt`, make your `setContent` function look like this:

```kotlin
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

```kotlin
// Utility function for efficient file copying
@Throws(IOException::class)
private fun copyFile(inputStream: InputStream, outputStream: OutputStream) {
    val buffer = ByteArray(1024) // Standard buffer size used in existing codebase
    var read: Int
    while (inputStream.read(buffer).also { read = it } != -1) {
        outputStream.write(buffer, 0, read)
    }
}

// Improved function to load zkey with caching and better error handling
private fun copyAssetToInternalStorage(context: Context, assetFileName: String): String? {
    val file = File(context.filesDir, assetFileName)
    
    // Check if file already exists and is not corrupted
    if (file.exists() && file.length() > 0) {
        return file.absolutePath
    }
    
    return try {
        context.assets.open(assetFileName).use { inputStream ->
            FileOutputStream(file).use { outputStream ->
                copyFile(inputStream, outputStream)
                outputStream.flush()
            }
        }
        file.absolutePath
    } catch (e: IOException) {
        e.printStackTrace()
        // Clean up partial file if it exists
        if (file.exists()) {
            file.delete()
        }
        null
    } catch (e: Exception) {
        e.printStackTrace()
        // Clean up partial file if it exists
        if (file.exists()) {
            file.delete()
        }
        null
    }
}

// Alternative Composable approach for better integration with Jetpack Compose
@Composable
fun getFilePathFromAssets(name: String): String {
    val context = LocalContext.current
    return remember {
        val assetManager = context.assets
        val inputStream = assetManager.open(name)
        val file = File(context.filesDir, name)
        
        // Only copy if file doesn't exist or is corrupted
        if (!file.exists() || file.length() == 0L) {
            try {
                copyFile(inputStream, FileOutputStream(file))
            } catch (e: IOException) {
                e.printStackTrace()
                // Return empty string as fallback, handle error in UI
                ""
            } finally {
                inputStream.close()
            }
        }
        file.absolutePath
    }
}
```

**Key improvements:**

1. **Caching**: Files are only copied once and reused on subsequent calls
2. **Standard buffer**: Uses the standard 1KB buffer size consistent with existing codebase
3. **File existence check**: Avoids unnecessary copying if file already exists
4. **Better error handling**: Includes cleanup of partial files on errors
5. **Resource management**: Properly closes streams using Kotlin's `use` function
6. **Compose integration**: Provides a `@Composable` alternative that integrates better with modern Android development
7. **Corruption detection**: Checks file size to detect incomplete copies

**Usage in your MainScreen:**

```kotlin
@Composable
fun MainScreen(context: Context) {
    val coroutineScope = rememberCoroutineScope()

    Column(
        modifier = Modifier
            .fillMaxSize()
            .padding(16.dp)
    ) {
        Button(onClick = {
            coroutineScope.launch {
                // Use the improved function
                val assetFilePath = copyAssetToInternalStorage(context, "multiplier2_final.zkey")
                assetFilePath?.let { path ->
                    val inputs = mutableMapOf<String, List<String>>()
                    inputs["a"] = listOf("3")
                    inputs["b"] = listOf("5")
                    val res = generateCircomProof(path, inputs)
                    println(res)
                } ?: run {
                    // Handle error case
                    println("Failed to load zkey file")
                }
            }
        }) {
            Text(text = "Generate Proof")
        }
    }
}
```

**Alternative approach using the Composable function:**

```kotlin
@Composable
fun MainScreen() {
    val coroutineScope = rememberCoroutineScope()
    
    // Get zkey path using the Composable function
    val zkeyPath = getFilePathFromAssets("multiplier2_final.zkey")

    Column(
        modifier = Modifier
            .fillMaxSize()
            .padding(16.dp)
    ) {
        Button(onClick = {
            coroutineScope.launch {
                if (zkeyPath.isNotEmpty()) {
                    val inputs = mutableMapOf<String, List<String>>()
                    inputs["a"] = listOf("3")
                    inputs["b"] = listOf("5")
                    val res = generateCircomProof(zkeyPath, inputs)
                    println(res)
                } else {
                    println("Failed to load zkey file")
                }
            }
        }) {
            Text(text = "Generate Proof")
        }
    }
}
```

At the bottom of this file we'll create a view with a function to generate a proof. In this example we're going to prove a simple circuit that accepts two inputs named `a` and `b` and generates an output `c`.

```kotlin
@Composable
fun MainScreen(context: Context) {
    val coroutineScope = rememberCoroutineScope()

    Column(
        modifier = Modifier
            .fillMaxSize()
            .padding(16.dp)
    ) {
        Button(onClick = {
            coroutineScope.launch {
                // Use the improved function
                val assetFilePath = copyAssetToInternalStorage(context, "multiplier2_final.zkey")
                assetFilePath?.let { path ->
                    val inputs = mutableMapOf<String, List<String>>()
                    inputs["a"] = listOf("3")
                    inputs["b"] = listOf("5")
                    val res = generateCircomProof(path, inputs)
                    println(res)
                } ?: run {
                    // Handle error case
                    println("Failed to load zkey file")
                }
            }
        }) {
            Text(text = "Generate Proof")
        }
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
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Button
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.runtime.remember
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.unit.dp
import java.io.File
import java.io.FileOutputStream
import java.io.IOException
import java.io.InputStream
import java.io.OutputStream
import kotlinx.coroutines.launch
import uniffi.mopro.generateCircomProof

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

    Column(modifier = Modifier.fillMaxSize().padding(16.dp)) {
        Button(
                onClick = {
                    coroutineScope.launch {
                        val assetFilePath =
                                copyAssetToInternalStorage(context, "multiplier2_final.zkey")
                        assetFilePath?.let { path ->
                            val inputs = mutableMapOf<String, List<String>>()
                            inputs["a"] = listOf("3")
                            inputs["b"] = listOf("5")
                            val res = generateCircomProof(path, inputs)
                            println(res)
                        } ?: run {
                            println("Failed to load zkey file")
                        }
                    }
                }
        ) { Text(text = "Generate Proof") }
    }
}

// Utility function for efficient file copying
@Throws(IOException::class)
private fun copyFile(inputStream: InputStream, outputStream: OutputStream) {
    val buffer = ByteArray(1024) // Standard buffer size used in existing codebase
    var read: Int
    while (inputStream.read(buffer).also { read = it } != -1) {
        outputStream.write(buffer, 0, read)
    }
}

// Improved function to load zkey with caching and better error handling
private fun copyAssetToInternalStorage(context: Context, assetFileName: String): String? {
    val file = File(context.filesDir, assetFileName)
    
    // Check if file already exists and is not corrupted
    if (file.exists() && file.length() > 0) {
        return file.absolutePath
    }
    
    return try {
        context.assets.open(assetFileName).use { inputStream ->
            FileOutputStream(file).use { outputStream ->
                copyFile(inputStream, outputStream)
                outputStream.flush()
            }
        }
        file.absolutePath
    } catch (e: IOException) {
        e.printStackTrace()
        // Clean up partial file if it exists
        if (file.exists()) {
            file.delete()
        }
        null
    } catch (e: Exception) {
        e.printStackTrace()
        // Clean up partial file if it exists
        if (file.exists()) {
            file.delete()
        }
        null
    }
}

// Alternative Composable approach for better integration with Jetpack Compose
@Composable
fun getFilePathFromAssets(name: String): String {
    val context = LocalContext.current
    return remember {
        val assetManager = context.assets
        val inputStream = assetManager.open(name)
        val file = File(context.filesDir, name)
        
        // Only copy if file doesn't exist or is corrupted
        if (!file.exists() || file.length() == 0L) {
            try {
                copyFile(inputStream, FileOutputStream(file))
            } catch (e: IOException) {
                e.printStackTrace()
                // Return empty string as fallback, handle error in UI
                ""
            } finally {
                inputStream.close()
            }
        }
        file.absolutePath
    }
}

```

</details>

You should now be able to run the Android app (`^`+`R` or `ctrl`+`R`) on the simulator or a device and build a proof. The app should log the proof.
