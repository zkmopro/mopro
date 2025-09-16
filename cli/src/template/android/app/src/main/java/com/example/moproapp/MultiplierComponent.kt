package com.example.moproapp

import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.width
import androidx.compose.material3.Button
import androidx.compose.material3.Text
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import com.example.moproapp.getFilePathFromAssets
import uniffi.{{PACKAGE_NAME}}.CircomProofResult
import uniffi.{{PACKAGE_NAME}}.CircomProof
import uniffi.{{PACKAGE_NAME}}.G1
import uniffi.{{PACKAGE_NAME}}.G2
import uniffi.{{PACKAGE_NAME}}.generateCircomProof
import uniffi.{{PACKAGE_NAME}}.verifyCircomProof
import uniffi.{{PACKAGE_NAME}}.ProofLib

@Composable
fun MultiplierComponent() {
    var provingTime by remember { mutableStateOf("proving time:") }
    var verifyingTime by remember { mutableStateOf("verifying time: ") }
    var valid by remember { mutableStateOf("valid:") }
    var output by remember { mutableStateOf("output:") }
    var isGeneratingCircomProof by remember { mutableStateOf(false) }
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

    val input_str: String = "{\"b\":[\"5\"],\"a\":[\"3\"]}"

    val zkeyPath = getFilePathFromAssets("multiplier2_final.zkey")

    Box(modifier = Modifier.fillMaxSize().padding(16.dp), contentAlignment = Alignment.Center) {
        Button(
            onClick = {
                isGeneratingCircomProof = true
                Thread(
                    Runnable {
                        try {
                            val startTime = System.currentTimeMillis()
                            res = generateCircomProof(zkeyPath, input_str, ProofLib.ARKWORKS)
                            val endTime = System.currentTimeMillis()
                            provingTime = "proving time: " + (endTime - startTime).toString() + " ms"
                        } finally {
                            isGeneratingCircomProof = false
                        }
                    }
                ).start()
            },
            modifier = Modifier.padding(top = 20.dp),
            enabled = !isGeneratingCircomProof && !isVerifyingProof
        ) { Text(text = "generate proof") }
        Button(
            onClick = {
                isVerifyingProof = true
                Thread {
                    try {
                        val startTime = System.currentTimeMillis()
                        valid = "valid: " + verifyCircomProof(zkeyPath, res, ProofLib.ARKWORKS).toString()
                        val endTime = System.currentTimeMillis()
                        verifyingTime = "verifying time: " + (endTime - startTime).toString() + " ms"
                        output = "output: " + res.inputs
                    } finally {
                        isVerifyingProof = false
                    }
                }.start()
            },
            modifier = Modifier.padding(top = 120.dp),
            enabled = !isGeneratingCircomProof && !isVerifyingProof && res.proof.a.x.isNotEmpty()
        ) { Text(text = "verify proof") }
        Text(
            text = "Multiplier proof",
            modifier = Modifier.padding(bottom = 180.dp),
            fontWeight = FontWeight.Bold
        )

        Text(text = provingTime, modifier = Modifier.padding(top = 250.dp).width(200.dp))
        Text(text = valid, modifier = Modifier.padding(top = 300.dp).width(200.dp))
        Text(text = verifyingTime, modifier = Modifier.padding(top = 350.dp).width(200.dp))
        Text(text = output, modifier = Modifier.padding(top = 400.dp).width(200.dp))
    }
}