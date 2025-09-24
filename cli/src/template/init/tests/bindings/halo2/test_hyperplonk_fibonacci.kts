// GENERATED LIB IMPORT PLACEHOLDER

try {
    val srsPath = "./test-vectors/halo2/hyperplonk_fibonacci_srs.bin"
    val vkPath = "./test-vectors/halo2/hyperplonk_fibonacci_vk.bin"
    val pkPath = "./test-vectors/halo2/hyperplonk_fibonacci_pk.bin"

    // Prepare inputs
    val inputs = mutableMapOf<String, List<String>>()
    inputs["out"] = listOf("55")

    // Generate proof
    var generateProofResult = generateHalo2Proof(srsPath, pkPath, inputs)
    assert(generateProofResult.inputs.size > 0) { "Inputs are empty" }

    // Verify proof
    var isValid = verifyHalo2Proof(srsPath, vkPath, generateProofResult.proof, generateProofResult.inputs)
    assert(isValid) { "Proof is invalid" }


} catch (e: Exception) {
    println(e)
    throw e
}
