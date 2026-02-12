// GENERATED LIB IMPORT PLACEHOLDER

try {
    val r1csPath = "./test-vectors/gnark/cubic_circuit.r1cs"
    val pkPath = "./test-vectors/gnark/cubic_circuit.pk"
    val vkPath = "./test-vectors/gnark/cubic_circuit.vk"

    // x=3: x^3 + x + 5 = 35
    val witnessJson = "{\"X\": \"3\", \"Y\": \"35\"}"

    // Generate proof
    val proofResult = generateGnarkProof(r1csPath, pkPath, witnessJson)
    assert(proofResult.proof.isNotEmpty()) { "Proof should not be empty" }
    assert(proofResult.publicInputs.isNotEmpty()) { "Public inputs should not be empty" }

    // Verify proof
    val isValid = verifyGnarkProof(r1csPath, vkPath, proofResult)
    assert(isValid) { "Proof is invalid" }

} catch (e: Exception) {
    println(e)
    throw e
}
