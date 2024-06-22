import uniffi.mopro.*

try {
    // Setup
    var moproCircom = MoproCircom()

    // Prepare inputs
    val inputs = mutableMapOf<String, List<String>>()
    inputs["a"] = listOf("3")
    inputs["b"] = listOf("5")

    // Generate proof
    var generateProofResult = moproCircom.generateProof("multiplier2", inputs)
    assert(generateProofResult.proof.size > 0) { "Proof is empty" }

    // Verify proof
    var isValid = moproCircom.verifyProof("multiplier2", generateProofResult.proof, generateProofResult.inputs)
    assert(isValid) { "Proof is invalid" }

    // Convert proof to Ethereum compatible proof
    var convertProofResult = toEthereumProof(generateProofResult.proof)
    var convertInputsResult = toEthereumInputs(generateProofResult.inputs)
    assert(convertProofResult.a.x.isNotEmpty()) { "Proof is empty" }
    assert(convertInputsResult.size > 0) { "Inputs are empty" }


} catch (e: Exception) {
    println(e)
}
