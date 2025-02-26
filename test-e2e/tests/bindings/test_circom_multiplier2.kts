import uniffi.mopro.*

try {
    var zkeyPath = "../test-vectors/circom/multiplier2_final.zkey"

    // Prepare inputs
    val inputs = mutableMapOf<String, List<String>>()
    inputs["a"] = listOf("3")
    inputs["b"] = listOf("5")

    // Generate proof
    var generateProofResult = generateCircomProof(zkeyPath, inputs, ProofLib.ARKWORKS)
    assert(generateProofResult.proof.size > 0) { "Proof is empty" }

    // Verify proof
    var isValid = verifyCircomProof(zkeyPath, generateProofResult.proof, generateProofResult.inputs, ProofLib.ARKWORKS)
    assert(isValid) { "Proof is invalid" }

    // Convert proof to Ethereum compatible proof
    var convertProofResult = toEthereumProof(generateProofResult.proof)
    var convertInputsResult = toEthereumInputs(generateProofResult.inputs)
    assert(convertProofResult.a.x.isNotEmpty()) { "Proof is empty" }
    assert(convertInputsResult.size > 0) { "Inputs are empty" }


} catch (e: Exception) {
    println(e)
    throw e
}
