import uniffi.mopro.*

try {
    var zkeyPath = "../test-vectors/circom/multiplier2_bls_final.zkey"

    val input_str: String = "{\"b\":[\"5\"],\"a\":[\"3\"]}"

    // Generate proof
    var generateProofResult = generateCircomProof(zkeyPath, input_str)
    assert(generateProofResult.proof.size > 0) { "Proof is empty" }

    // Verify proof
    var isValid = verifyCircomProof(zkeyPath, generateProofResult.proof, generateProofResult.inputs)
    assert(isValid) { "Proof is invalid" }


} catch (e: Exception) {
    println(e)
    throw e
}
