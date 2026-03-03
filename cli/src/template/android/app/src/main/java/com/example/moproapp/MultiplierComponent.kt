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
import uniffi.mopro.CircomProof
import uniffi.mopro.CircomProofResult
import uniffi.mopro.G1
import uniffi.mopro.G2
import uniffi.mopro.generateCircomProof
import uniffi.mopro.verifyCircomProof
import uniffi.mopro.ProofLib

@Composable
fun MultiplierComponent() {
    var provingTime by remember { mutableStateOf<String?>(null) }
    var verifyingTime by remember { mutableStateOf<String?>(null) }
    var valid by remember { mutableStateOf<String?>(null) }
    var output by remember { mutableStateOf<String?>(null) }
    var isGeneratingCircomProof by remember { mutableStateOf(false) }
    var isGeneratingRapidsnarkProof by remember { mutableStateOf(false) }
    var isVerifyingProof by remember { mutableStateOf(false) }
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

    val inputStr = "{\"b\":[\"5\"],\"a\":[\"3\"]}"
    val zkeyPath = getFilePathFromAssets("multiplier2_final.zkey")

    val isBusy = isGeneratingCircomProof || isGeneratingRapidsnarkProof || isVerifyingProof

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
            text = "Circom Multiplier",
            style = MaterialTheme.typography.titleLarge,
            fontWeight = FontWeight.Bold,
            fontSize = 22.sp
        )
        Text(
            text = "Proves a × b = c (e.g. 3 × 5) with Arkworks or Rapidsnark.",
            style = MaterialTheme.typography.bodyMedium,
            textAlign = TextAlign.Center,
            modifier = Modifier.padding(bottom = 8.dp)
        )

        // Arkworks
        OutlinedCard(
            modifier = Modifier.fillMaxWidth(),
            colors = CardDefaults.outlinedCardColors()
        ) {
            Column(
                modifier = Modifier.padding(16.dp),
                horizontalAlignment = Alignment.CenterHorizontally,
                verticalArrangement = Arrangement.spacedBy(12.dp)
            ) {
                Text("Arkworks", fontWeight = FontWeight.SemiBold, fontSize = 16.sp)
                if (isGeneratingCircomProof || isVerifyingProof) {
                    CircularProgressIndicator(modifier = Modifier.padding(8.dp))
                }
                Button(
                    onClick = {
                        isGeneratingCircomProof = true
                        provingTime = null
                        valid = null
                        verifyingTime = null
                        output = null
                        Thread {
                            try {
                                val startTime = System.currentTimeMillis()
                                res = generateCircomProof(zkeyPath, inputStr, ProofLib.ARKWORKS)
                                val endTime = System.currentTimeMillis()
                                provingTime = "${endTime - startTime} ms"
                            } finally {
                                isGeneratingCircomProof = false
                            }
                        }.start()
                    },
                    modifier = Modifier.fillMaxWidth().testTag("circomGenerateProofButton"),
                    enabled = !isBusy
                ) { Text("Generate proof") }
                Button(
                    onClick = {
                        isVerifyingProof = true
                        verifyingTime = null
                        Thread {
                            try {
                                val startTime = System.currentTimeMillis()
                                val v = verifyCircomProof(zkeyPath, res, ProofLib.ARKWORKS)
                                val endTime = System.currentTimeMillis()
                                verifyingTime = "${endTime - startTime} ms"
                                valid = v.toString()
                                output = res.inputs.toString()
                            } finally {
                                isVerifyingProof = false
                            }
                        }.start()
                    },
                    modifier = Modifier.fillMaxWidth().testTag("circomVerifyProofButton"),
                    enabled = !isBusy && res.proof.a.x.isNotEmpty()
                ) { Text("Verify proof") }
            }
        }

        // Rapidsnark
        OutlinedCard(
            modifier = Modifier.fillMaxWidth(),
            colors = CardDefaults.outlinedCardColors()
        ) {
            Column(
                modifier = Modifier.padding(16.dp),
                horizontalAlignment = Alignment.CenterHorizontally,
                verticalArrangement = Arrangement.spacedBy(12.dp)
            ) {
                Text("RapidSnark", fontWeight = FontWeight.SemiBold, fontSize = 16.sp)
                if (isGeneratingRapidsnarkProof || isVerifyingProof) {
                    CircularProgressIndicator(modifier = Modifier.padding(8.dp))
                }
                Button(
                    onClick = {
                        isGeneratingRapidsnarkProof = true
                        provingTime = null
                        valid = null
                        verifyingTime = null
                        output = null
                        Thread {
                            try {
                                val startTime = System.currentTimeMillis()
                                res = generateCircomProof(zkeyPath, inputStr, ProofLib.RAPIDSNARK)
                                val endTime = System.currentTimeMillis()
                                provingTime = "${endTime - startTime} ms"
                            } finally {
                                isGeneratingRapidsnarkProof = false
                            }
                        }.start()
                    },
                    modifier = Modifier.fillMaxWidth().testTag("rapidsnarkGenerateProofButton"),
                    enabled = !isBusy
                ) { Text("Generate proof (RapidSnark)") }
                Button(
                    onClick = {
                        isVerifyingProof = true
                        verifyingTime = null
                        Thread {
                            try {
                                val startTime = System.currentTimeMillis()
                                val v = verifyCircomProof(zkeyPath, res, ProofLib.RAPIDSNARK)
                                val endTime = System.currentTimeMillis()
                                verifyingTime = "${endTime - startTime} ms"
                                valid = v.toString()
                                output = res.inputs.toString()
                            } finally {
                                isVerifyingProof = false
                            }
                        }.start()
                    },
                    modifier = Modifier.fillMaxWidth().testTag("rapidsnarkVerifyProofButton"),
                    enabled = !isBusy && res.proof.a.x.isNotEmpty()
                ) { Text("Verify proof (RapidSnark)") }
            }
        }

        if (provingTime != null || verifyingTime != null || valid != null || output != null) {
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
                    valid?.let { Text("Valid: $it", style = MaterialTheme.typography.bodyMedium, fontWeight = if (it == "true") FontWeight.Bold else FontWeight.Normal) }
                    output?.let { Text("Output: $it", style = MaterialTheme.typography.bodyMedium) }
                }
            }
        }

        Spacer(modifier = Modifier.height(24.dp))
    }
}
