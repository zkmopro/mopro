import uniffi.mopro.*

try {
    val srsPath = "../test-vectors/halo2/keccak256_srs.bin"
    val vkPath = "../test-vectors/halo2/keccak256_vk.bin"
    val pkPath = "../test-vectors/halo2/keccak256_pk.bin"

    // Prepare inputs
    val inputs = mutableMapOf<String, List<String>>()
    // Sample byte inputs to the circuit
    inputs["in"] = listOf("10", "10", "1", "0")

    // Generate proof
    var generateProofResult = generateHalo2Proof(srsPath, pkPath, inputs)
    assert(generateProofResult.proof.size > 0) { "Proof is empty" }
    assert(generateProofResult.inputs.size > 0) { "Inputs are empty" }

    // Verify proof
    var isValid = verifyHalo2Proof(srsPath, vkPath, generateProofResult.proof, generateProofResult.inputs)
    assert(isValid) { "Proof is invalid" }


} catch (e: Exception) {
    println(e)
    throw e
}
