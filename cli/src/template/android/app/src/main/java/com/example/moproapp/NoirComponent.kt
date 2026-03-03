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
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import uniffi.mopro.generateNoirProof
import uniffi.mopro.getNoirVerificationKey
import uniffi.mopro.verifyNoirProof

@Composable
fun NoirComponent() {
    val context = LocalContext.current
    var provingTime by remember { mutableStateOf<String?>(null) }
    var proofResult by remember { mutableStateOf<String?>(null) }
    var verificationTime by remember { mutableStateOf<String?>(null) }
    var verificationResult by remember { mutableStateOf<String?>(null) }
    var proofBytes by remember { mutableStateOf<ByteArray?>(null) }
    var verificationKey by remember { mutableStateOf<ByteArray?>(null) }
    var isGeneratingProof by remember { mutableStateOf(false) }
    var isVerifyingProof by remember { mutableStateOf(false) }
    var statusMessage by remember { mutableStateOf("Ready to generate proof") }

    val circuitFile = getFilePathFromAssets("noir_multiplier2.json")
    val srsFile = getFilePathFromAssets("noir_multiplier2.srs")

    val existingVk = remember {
        try {
            context.assets.open("noir_multiplier2.vk").readBytes()
        } catch (e: Exception) {
            null
        }
    }

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
            text = "Noir Multiplier",
            style = MaterialTheme.typography.titleLarge,
            fontWeight = FontWeight.Bold,
            fontSize = 22.sp
        )
        Text(
            text = "Proves a × b = c using Noir (e.g. 3 × 5) with Keccak for Solidity.",
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
                Text(
                    text = statusMessage,
                    style = MaterialTheme.typography.bodyMedium,
                    textAlign = TextAlign.Center,
                    fontWeight = if (isBusy) FontWeight.SemiBold else FontWeight.Normal
                )
                if (isBusy) {
                    CircularProgressIndicator(modifier = Modifier.padding(8.dp))
                }

                Button(
                    onClick = {
                        isGeneratingProof = true
                        provingTime = null
                        proofResult = null
                        statusMessage = "Generating proof…"
                        Thread {
                            try {
                                val inputs = listOf("3", "5")
                                val onChain = true
                                val lowMemoryMode = false
                                val vk: ByteArray = existingVk ?: run {
                                    statusMessage = "Generating verification key…"
                                    getNoirVerificationKey(circuitFile, srsFile, onChain, lowMemoryMode)
                                }
                                verificationKey = vk
                                statusMessage = "Generating proof…"
                                val startTime = System.currentTimeMillis()
                                proofBytes = generateNoirProof(
                                    circuitFile,
                                    srsFile,
                                    inputs,
                                    onChain,
                                    vk,
                                    lowMemoryMode
                                )
                                val endTime = System.currentTimeMillis()
                                provingTime = "${endTime - startTime} ms"
                                proofResult = "Proof: ${proofBytes?.size ?: 0} bytes"
                                statusMessage = "Proof generated"
                            } catch (e: Exception) {
                                provingTime = "Failed"
                                proofResult = "Error: ${e.message}"
                                statusMessage = "Proof failed"
                                e.printStackTrace()
                            } finally {
                                isGeneratingProof = false
                            }
                        }.start()
                    },
                    modifier = Modifier.fillMaxWidth().testTag("noirGenerateProofButton"),
                    enabled = !isBusy
                ) { Text("Generate proof") }

                Button(
                    onClick = {
                        isVerifyingProof = true
                        verificationTime = null
                        verificationResult = null
                        statusMessage = "Verifying proof…"
                        Thread {
                            try {
                                proofBytes?.let { proof ->
                                    verificationKey?.let { vk ->
                                        val onChain = true
                                        val lowMemoryMode = false
                                        val startTime = System.currentTimeMillis()
                                        val result = verifyNoirProof(
                                            circuitFile,
                                            proof,
                                            onChain,
                                            vk,
                                            lowMemoryMode
                                        )
                                        val endTime = System.currentTimeMillis()
                                        verificationTime = "${endTime - startTime} ms"
                                        verificationResult = result.toString()
                                        statusMessage = if (result) "Verified successfully" else "Verification failed"
                                    } ?: run {
                                        verificationResult = "No verification key"
                                        statusMessage = "Generate a proof first"
                                    }
                                } ?: run {
                                    verificationResult = "No proof"
                                    statusMessage = "Generate a proof first"
                                }
                            } catch (e: Exception) {
                                verificationTime = "Failed"
                                verificationResult = "Error: ${e.message}"
                                statusMessage = "Verification error"
                                e.printStackTrace()
                            } finally {
                                isVerifyingProof = false
                            }
                        }.start()
                    },
                    modifier = Modifier.fillMaxWidth().testTag("noirVerifyProofButton"),
                    enabled = !isBusy && proofBytes != null
                ) { Text("Verify proof") }
            }
        }

        if (provingTime != null || proofResult != null || verificationTime != null || verificationResult != null) {
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
                    proofResult?.let { Text(it, style = MaterialTheme.typography.bodyMedium) }
                    verificationTime?.let { Text("Verifying: $it", style = MaterialTheme.typography.bodyMedium) }
                    verificationResult?.let {
                        Text(
                            "Valid: $it",
                            style = MaterialTheme.typography.bodyMedium,
                            fontWeight = if (it == "true") FontWeight.Bold else FontWeight.Normal
                        )
                    }
                }
            }
        }

        Spacer(modifier = Modifier.height(24.dp))
    }
}
