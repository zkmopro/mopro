import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.width
import androidx.compose.material3.Button
import androidx.compose.material3.Text
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import com.mopro.mopro_app.getFilePathFromAssets
import uniffi.MoproBindings.generateNoirProof
import uniffi.MoproBindings.verifyNoirProof

@Composable
fun NoirComponent() {
    var provingTime by remember { mutableStateOf("proving time:") }
    var verifyingTime by remember { mutableStateOf("verifying time: ") }
    var valid by remember { mutableStateOf("valid:") }
    var res by remember {
        mutableStateOf<ByteArray>(
            ByteArray(0)
        )
    }

    val input: List<String> = listOf("3", "5")

    val circuitPath = getFilePathFromAssets("noir_multiplier2.json")

    Box(modifier = Modifier.fillMaxSize().padding(16.dp), contentAlignment = Alignment.Center) {
        Button(
            onClick = {
                Thread(
                    Runnable {
                        val startTime = System.currentTimeMillis()
                        res = generateNoirProof(circuitPath,null , input)
                        val endTime = System.currentTimeMillis()
                        provingTime = "proving time: " + (endTime - startTime).toString() + " ms"
                    }
                ).start()
            },
            modifier = Modifier.padding(top = 20.dp).testTag("noirGenerateProofButton")
        ) { Text(text = "generate proof") }
        Button(
            onClick = {
                val startTime = System.currentTimeMillis()
                valid = "valid: " + verifyNoirProof(circuitPath, res).toString()
                val endTime = System.currentTimeMillis()
                verifyingTime = "verifying time: " + (endTime - startTime).toString() + " ms"
            },
            modifier = Modifier.padding(top = 120.dp).testTag("noirVerifyProofButton")
        ) { Text(text = "verify proof") }
        Text(
            text = "Multiplier proof",
            modifier = Modifier.padding(bottom = 180.dp),
            fontWeight = FontWeight.Bold
        )

        Text(text = provingTime, modifier = Modifier.padding(top = 250.dp).width(200.dp))
        Text(text = valid, modifier = Modifier.padding(top = 300.dp).width(200.dp))
        Text(text = verifyingTime, modifier = Modifier.padding(top = 350.dp).width(200.dp))
    }
}