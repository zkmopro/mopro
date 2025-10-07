package com.example.moproapp

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
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import uniffi.mopro.Halo2ProofResult
import uniffi.mopro.generateHalo2Proof
import uniffi.mopro.verifyHalo2Proof

@Composable
fun FibonacciComponent() {
    var provingTime by remember { mutableStateOf("proving time:") }
    var verifyingTime by remember { mutableStateOf("verifying time: ") }
    var valid by remember { mutableStateOf("valid:") }
    var isGeneratingProof by remember { mutableStateOf(false) }
    var isVerifyingProof by remember { mutableStateOf(false) }
    var res by remember {
        mutableStateOf<Halo2ProofResult>(
            Halo2ProofResult(proof = ByteArray(size = 0), inputs = ByteArray(size = 0))
        )
    }

    val srsPath = getFilePathFromAssets("plonk_fibonacci_srs.bin")
    val provingKeyPath = getFilePathFromAssets("plonk_fibonacci_pk.bin")
    val verifyingKeyPath = getFilePathFromAssets("plonk_fibonacci_vk.bin")


    val inputs = mutableMapOf<String, List<String>>()
    inputs["out"] = listOf("55")

    Box(modifier = Modifier.fillMaxSize().padding(16.dp), contentAlignment = Alignment.Center) {
        Button(
            onClick = {
                isGeneratingProof = true
                Thread {
                    try {
                        val startTime = System.currentTimeMillis()
                        res = generateHalo2Proof(srsPath, provingKeyPath, inputs)
                        val endTime = System.currentTimeMillis()
                        provingTime =
                            "proving time: " +
                                    (endTime - startTime).toString() +
                                    " ms"
                    } finally {
                        isGeneratingProof = false
                    }
                }
                    .start()
            },
            modifier = Modifier.padding(top = 20.dp).testTag("halo2GenerateProofButton"),
            enabled = !isGeneratingProof && !isVerifyingProof
        ) { Text(text = "generate proof") }
        Button(
            onClick = {
                isVerifyingProof = true
                Thread {
                    try {
                        val startTime = System.currentTimeMillis()
                        valid = "valid: " + verifyHalo2Proof(srsPath, verifyingKeyPath, res.proof, res.inputs).toString()
                        val endTime = System.currentTimeMillis()
                        verifyingTime = "verifying time: " + (endTime - startTime).toString() + " ms"
                    } finally {
                        isVerifyingProof = false
                    }
                }.start()
            },
            modifier = Modifier.padding(top = 120.dp).testTag("halo2VerifyProofButton"),
            enabled = !isGeneratingProof && !isVerifyingProof && res.proof.isNotEmpty()
        ) { Text(text = "verify proof") }
        Text(
            text = "Halo2 Fibonacci proof",
            modifier = Modifier.padding(bottom = 180.dp),
            fontWeight = FontWeight.Bold
        )

        Text(text = provingTime, modifier = Modifier.padding(top = 250.dp).width(200.dp))
        Text(text = valid, modifier = Modifier.padding(top = 300.dp).width(200.dp))
        Text(text = verifyingTime, modifier = Modifier.padding(top = 350.dp).width(200.dp))
    }
}