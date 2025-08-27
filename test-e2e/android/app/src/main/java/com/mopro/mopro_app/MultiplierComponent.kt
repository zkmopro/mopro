import androidx.compose.foundation.layout.*
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.width
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import com.mopro.mopro_app.getFilePathFromAssets
import uniffi.MoproBindings.*

@Composable
fun MultiplierComponent() {
    var arkworksProvingTime by remember { mutableStateOf("Proving time: -") }
    var arkworksVerifyingTime by remember { mutableStateOf("Verifying time: -") }
    var arkworksValid by remember { mutableStateOf("Valid: -") }
    var arkworksOutput by remember { mutableStateOf("Output: -") }
    
    var rapidsnarkProvingTime by remember { mutableStateOf("Proving time: -") }
    var rapidsnarkVerifyingTime by remember { mutableStateOf("Verifying time: -") }
    var rapidsnarkValid by remember { mutableStateOf("Valid: -") }
    var rapidsnarkOutput by remember { mutableStateOf("Output: -") }
    
    var res by remember {
        mutableStateOf(
            CircomProofResult(
                proof = CircomProof(
                    a = G1(x = "", y = "", z = ""),
                    b = G2(x = listOf(), y = listOf(), z = listOf()),
                    c = G1(x = "", y = "", z = ""),
                    protocol = "",
                    curve = ""
                ),
                inputs = listOf()
            )
        )
    }

    val input_str: String = "{\"b\":[\"5\"],\"a\":[\"3\"]}"
    val zkeyPath = getFilePathFromAssets("multiplier2_final.zkey")
    val witnesscalcZkeyPath = getFilePathFromAssets("multiplier2_witnesscalc_final.zkey")

    Column(
        modifier = Modifier
            .fillMaxSize()
            .padding(16.dp),
        horizontalAlignment = Alignment.CenterHorizontally
    ) {
        Text(
            text = "Multiplier Proof Demo",
            style = MaterialTheme.typography.headlineMedium,
            fontWeight = FontWeight.Bold,
            modifier = Modifier.padding(bottom = 24.dp)
        )

        // Arkworks Section
        Card(
            modifier = Modifier
                .fillMaxWidth()
                .padding(bottom = 16.dp)
        ) {
            Column(
                modifier = Modifier.padding(16.dp),
                horizontalAlignment = Alignment.CenterHorizontally
            ) {
                Text(
                    text = "Arkworks",
                    style = MaterialTheme.typography.titleLarge,
                    modifier = Modifier.padding(bottom = 8.dp)
                )
                
                Button(
                    onClick = {
                        Thread(
                            Runnable {
                                val startTime = System.currentTimeMillis()
                                res = generateCircomProof(zkeyPath, input_str, ProofLib.ARKWORKS)
                                val endTime = System.currentTimeMillis()
                                arkworksProvingTime = "Proving time: ${endTime - startTime} ms"
                            }
                        ).start()
                    },
                    modifier = Modifier
                        .fillMaxWidth()
                        .padding(vertical = 4.dp)
                        .testTag("circomGenerateProofButton")
                ) {
                    Text("Generate Arkworks Proof")
                }

                Button(
                    onClick = {
                        val startTime = System.currentTimeMillis()
                        arkworksValid = "Valid: ${verifyCircomProof(zkeyPath, res, ProofLib.ARKWORKS)}"
                        val endTime = System.currentTimeMillis()
                        arkworksVerifyingTime = "Verifying time: ${endTime - startTime} ms"
                        arkworksOutput = "Output: ${res.inputs}"
                    },
                    modifier = Modifier
                        .fillMaxWidth()
                        .padding(vertical = 4.dp)
                        .testTag("circomVerifyProofButton")
                ) {
                    Text("Verify Arkworks Proof")
                }

                Text(text = arkworksProvingTime, modifier = Modifier.padding(vertical = 4.dp))
                Text(text = arkworksVerifyingTime, modifier = Modifier.padding(vertical = 4.dp))
                Text(text = arkworksValid, modifier = Modifier.padding(vertical = 4.dp))
                Text(text = arkworksOutput, modifier = Modifier.padding(vertical = 4.dp))
            }
        }

        // Rapidsnark Section
        Card(
            modifier = Modifier
                .fillMaxWidth()
                .padding(bottom = 16.dp)
        ) {
            Column(
                modifier = Modifier.padding(16.dp),
                horizontalAlignment = Alignment.CenterHorizontally
            ) {
                Text(
                    text = "Rapidsnark",
                    style = MaterialTheme.typography.titleLarge,
                    modifier = Modifier.padding(bottom = 8.dp)
                )

                Button(
                    onClick = {
                        Thread(
                            Runnable {
                                val startTime = System.currentTimeMillis()
                                res = generateCircomProof(witnesscalcZkeyPath, input_str, ProofLib.RAPIDSNARK)
                                val endTime = System.currentTimeMillis()
                                rapidsnarkProvingTime = "Proving time: ${endTime - startTime} ms"
                            }
                        ).start()
                    },
                    modifier = Modifier
                        .fillMaxWidth()
                        .padding(vertical = 4.dp)
                        .testTag("rapidsnarkGenerateProofButton")
                ) {
                    Text("Generate Rapidsnark Proof")
                }

                Button(
                    onClick = {
                        val startTime = System.currentTimeMillis()
                        rapidsnarkValid = "Valid: ${verifyCircomProof(witnesscalcZkeyPath, res, ProofLib.RAPIDSNARK)}"
                        val endTime = System.currentTimeMillis()
                        rapidsnarkVerifyingTime = "Verifying time: ${endTime - startTime} ms"
                        rapidsnarkOutput = "Output: ${res.inputs}"
                    },
                    modifier = Modifier
                        .fillMaxWidth()
                        .padding(vertical = 4.dp)
                        .testTag("rapidsnarkVerifyProofButton")
                ) {
                    Text("Verify Rapidsnark Proof")
                }

                Text(text = rapidsnarkProvingTime, modifier = Modifier.padding(vertical = 4.dp))
                Text(text = rapidsnarkVerifyingTime, modifier = Modifier.padding(vertical = 4.dp))
                Text(text = rapidsnarkValid, modifier = Modifier.padding(vertical = 4.dp))
                Text(text = rapidsnarkOutput, modifier = Modifier.padding(vertical = 4.dp))
            }
        }
    }
}