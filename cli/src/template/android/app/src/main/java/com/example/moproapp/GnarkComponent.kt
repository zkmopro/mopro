package com.example.moproapp

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.Button
import androidx.compose.material3.Card
import androidx.compose.material3.CardDefaults
import androidx.compose.material3.CircularProgressIndicator
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.OutlinedCard
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import uniffi.mopro.generateGnarkProof
import uniffi.mopro.verifyGnarkProof
import uniffi.mopro.GnarkProofResult

@Composable
fun GnarkComponent() {
    var provingTime by remember { mutableStateOf<String?>(null) }
    var verifyingTime by remember { mutableStateOf<String?>(null) }
    var valid by remember { mutableStateOf<String?>(null) }
    var errorMessage by remember { mutableStateOf<String?>(null) }
    var isGeneratingProof by remember { mutableStateOf(false) }
    var isVerifyingProof by remember { mutableStateOf(false) }
    var proofResult by remember { mutableStateOf<GnarkProofResult?>(null) }

    val r1csPath = getFilePathFromAssets("cubic_circuit.r1cs")
    val pkPath = getFilePathFromAssets("cubic_circuit.pk")
    val vkPath = getFilePathFromAssets("cubic_circuit.vk")
    // Cubic circuit: x^3 + x + 5 = Y. For X=3: 27+3+5=35
    val witnessJson = """{"X": "3", "Y": "35"}"""

    val isBusy = isGeneratingProof || isVerifyingProof
    val scrollState = rememberScrollState()

    Column(
        modifier = Modifier
            .fillMaxSize()
            .verticalScroll(scrollState)
            .padding(16.dp),
        horizontalAlignment = Alignment.CenterHorizontally,
        verticalArrangement = Arrangement.spacedBy(12.dp)
    ) {
        Text(
            text = "Gnark Cubic",
            style = MaterialTheme.typography.titleLarge,
            fontWeight = FontWeight.Bold,
            fontSize = 22.sp
        )
        Text(
            text = "Proves x³ + x + 5 = Y (e.g. X=3 → Y=35) using Groth16 BN254.",
            style = MaterialTheme.typography.bodyMedium,
            textAlign = TextAlign.Center,
            modifier = Modifier.padding(bottom = 8.dp)
        )

        OutlinedCard(
            modifier = Modifier.fillMaxWidth(),
            colors = CardDefaults.outlinedCardColors()
        ) {
            Column(
                modifier = Modifier.padding(16.dp),
                horizontalAlignment = Alignment.CenterHorizontally,
                verticalArrangement = Arrangement.spacedBy(12.dp)
            ) {
                if (isBusy) {
                    CircularProgressIndicator(modifier = Modifier.padding(8.dp))
                    Text(
                        text = if (isGeneratingProof) "Generating proof…" else "Verifying…",
                        style = MaterialTheme.typography.bodyMedium
                    )
                }

                Button(
                    onClick = {
                        isGeneratingProof = true
                        provingTime = null
                        valid = null
                        verifyingTime = null
                        errorMessage = null
                        Thread {
                            try {
                                val startTime = System.currentTimeMillis()
                                proofResult = generateGnarkProof(r1csPath, pkPath, witnessJson)
                                val endTime = System.currentTimeMillis()
                                provingTime = "${endTime - startTime} ms"
                            } catch (e: Exception) {
                                provingTime = "Failed"
                                errorMessage = e.message ?: "Proof generation failed"
                                proofResult = null
                                e.printStackTrace()
                            } finally {
                                isGeneratingProof = false
                            }
                        }.start()
                    },
                    modifier = Modifier.fillMaxWidth().testTag("gnarkGenerateProofButton"),
                    enabled = !isBusy
                ) {
                    Text("Generate proof")
                }

                Button(
                    onClick = {
                        isVerifyingProof = true
                        verifyingTime = null
                        valid = null
                        errorMessage = null
                        val currentProof = proofResult
                        if (currentProof == null) {
                            valid = "false"
                            isVerifyingProof = false
                            return@Button
                        }
                        Thread {
                            try {
                                val startTime = System.currentTimeMillis()
                                val isValid = verifyGnarkProof(r1csPath, vkPath, currentProof)
                                val endTime = System.currentTimeMillis()
                                verifyingTime = "${endTime - startTime} ms"
                                valid = isValid.toString()
                            } catch (e: Exception) {
                                verifyingTime = "Failed"
                                errorMessage = e.message ?: "Verification failed"
                                e.printStackTrace()
                            } finally {
                                isVerifyingProof = false
                            }
                        }.start()
                    },
                    modifier = Modifier.fillMaxWidth().testTag("gnarkVerifyProofButton"),
                    enabled = !isBusy && proofResult != null
                ) {
                    Text("Verify proof")
                }
            }
        }

        if (provingTime != null || verifyingTime != null || valid != null || errorMessage != null) {
            Card(
                modifier = Modifier.fillMaxWidth(),
                colors = CardDefaults.cardColors(containerColor = MaterialTheme.colorScheme.surfaceVariant)
            ) {
                Column(
                    modifier = Modifier.padding(16.dp),
                    verticalArrangement = Arrangement.spacedBy(4.dp)
                ) {
                    Text("Results", fontWeight = FontWeight.SemiBold, fontSize = 16.sp)
                    provingTime?.let { Text("Proving: $it", style = MaterialTheme.typography.bodyMedium) }
                    verifyingTime?.let { Text("Verifying: $it", style = MaterialTheme.typography.bodyMedium) }
                    valid?.let {
                        Text(
                            "Valid: $it",
                            style = MaterialTheme.typography.bodyMedium,
                            fontWeight = if (it == "true") FontWeight.Bold else FontWeight.Normal
                        )
                    }
                    errorMessage?.let { Text("Error: $it", style = MaterialTheme.typography.bodyMedium) }
                }
            }
        }

        Spacer(modifier = Modifier.height(24.dp))
    }
}
