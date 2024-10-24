import uniffi.mopro.*

try {
  var ar1csPath = "../test-vectors/ashlang/example.ar1cs"

  // this number is used but not constrained
  var inputs = listOf("55")

  // Generate Proof
  var generateProofResult = generateAshlangSpartanProof( ar1csPath, inputs)

  assert(generateProofResult.proof.size > 0) { "Proof is empty" }

  // Verify Proof
  var isValid = verifyAshlangSpartanProof(
  ar1csPath, generateProofResult.proof)

  assert(isValid) { "Proof is invalid" }
} catch (e: Exception) {
  println(e)
}
