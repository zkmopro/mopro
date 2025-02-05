import uniffi.mopro.*

try {
    val vkPath = "../test-vectors/plonky2/plonky2_fibonacci_vk.bin"
    val pkPath = "../test-vectors/plonky2/plonky2_fibonacci_pk.bin"

    // Prepare inputs
    val inputs = mutableMapOf<String, List<String>>()
    inputs["a"] = listOf("0")
    inputs["b"] = listOf("1")

    // Generate proof
    var generateProofResult = generatePlonky2Proof(pkPath, inputs)
    assert(generateProofResult.size > 0) { "Proof is empty" }

    // Verify proof
    var isValid = verifyPlonky2Proof(vkPath, generateProofResult)
    assert(isValid) { "Proof is invalid" }


} catch (e: Exception) {
    println(e)
    throw e
}
