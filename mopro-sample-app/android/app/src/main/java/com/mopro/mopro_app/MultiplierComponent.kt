import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.width
import androidx.compose.material3.Button
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import uniffi.mopro.GenerateProofResult
import uniffi.mopro.generateCircomProof
import uniffi.mopro.verifyCircomProof
import java.nio.file.Paths

@Composable
fun MultiplierComponent(zkeyPath: String) {
    var initTime by remember { mutableStateOf("init time:") }
    var provingTime by remember { mutableStateOf("proving time:") }
    var verifyingTime by remember { mutableStateOf("verifying time: ") }
    var valid by remember { mutableStateOf("valid:") }
    var output by remember { mutableStateOf("output:") }
    var res by remember {
        mutableStateOf<GenerateProofResult>(
            GenerateProofResult(proof = ByteArray(size = 0), inputs = ByteArray(size = 0))
        )
    }

    val inputs = mutableMapOf<String, List<String>>()
    inputs["a"] = listOf("3")
    inputs["b"] = listOf("5")

    Box(modifier = Modifier.fillMaxSize().padding(16.dp), contentAlignment = Alignment.Center) {
        Button(
            onClick = {
                Thread(
                    Runnable {
                        val startTime = System.currentTimeMillis()
                        res = generateCircomProof(zkeyPath, inputs)
                        val endTime = System.currentTimeMillis()
                        provingTime =
                            "proving time: " +
                                    (endTime - startTime).toString() +
                                    " ms"
                    }
                )
                    .start()
            },
            modifier = Modifier.padding(top = 20.dp)
        ) { Text(text = "generate proof") }
        Button(
            onClick = {
                val startTime = System.currentTimeMillis()
                valid = "valid: " + verifyCircomProof(zkeyPath, res.proof, res.inputs).toString()
                val endTime = System.currentTimeMillis()
                verifyingTime = "verifying time: " + (endTime - startTime).toString() + " ms"
                output = "output: " + uniffi.mopro.toEthereumInputs(res.inputs)
            },
            modifier = Modifier.padding(top = 120.dp)
        ) { Text(text = "verify proof") }
        Text(
            text = "Multiplier proof",
            modifier = Modifier.padding(bottom = 180.dp),
            fontWeight = FontWeight.Bold
        )

        Text(text = initTime, modifier = Modifier.padding(top = 200.dp).width(200.dp))
        Text(text = provingTime, modifier = Modifier.padding(top = 250.dp).width(200.dp))
        Text(text = valid, modifier = Modifier.padding(top = 300.dp).width(200.dp))
        Text(text = verifyingTime, modifier = Modifier.padding(top = 350.dp).width(200.dp))
        Text(text = output, modifier = Modifier.padding(top = 400.dp).width(200.dp))
    }
}