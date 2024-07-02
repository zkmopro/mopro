package com.mopro.mopro_app

import MultiplierComponent
import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview
import com.mopro.mopro_app.ui.theme.MoproappTheme
import java.io.File
import java.io.IOException
import java.io.InputStream
import java.io.OutputStream

@Throws(IOException::class)
private fun copyFile(`in`: InputStream, out: OutputStream) {
    val buffer = ByteArray(1024)
    var read: Int
    while ((`in`.read(buffer).also { read = it }) != -1) {
        out.write(buffer, 0, read)
    }
}

class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        enableEdgeToEdge()
        setContent {
            MoproappTheme {
                Scaffold(modifier = Modifier.fillMaxSize()) { innerPadding ->
                    val s = assets.open("multiplier2_final.zkey")
                    val f = File(filesDir, "multiplier2_final.zkey")
                    copyFile(s, f.outputStream())
                    Column(modifier = Modifier.fillMaxSize().padding(paddingValues = innerPadding)) {
                        MultiplierComponent(f.absolutePath)
                    }
                }
            }
        }
    }
}

@Composable
fun Greeting(name: String, modifier: Modifier = Modifier) {
    Text(
        text = "Hello $name!",
        modifier = modifier
    )
}

@Preview(showBackground = true)
@Composable
fun GreetingPreview() {
    MoproappTheme {
        Greeting("Android")
    }
}