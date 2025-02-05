import Foundation
import mopro

do {
  let vkPath = "../../../test-vectors/plonky2/plonky2_fibonacci_vk.bin"
  let pkPath = "../../../test-vectors/plonky2/plonky2_fibonacci_pk.bin"

  // Prepare inputs
  var inputs = [String: [String]]()
  let a = 0
  let b = 1
  inputs["a"] = [String(a)]
  inputs["b"] = [String(b)]

  // Generate Proof
  let generateProofResult = try generatePlonky2Proof(pkPath: pkPath, circuitInputs: inputs)
  assert(!generateProofResult.isEmpty, "Proof should not be empty")

  let isValid = try verifyPlonky2Proof(
    vkPath: vkPath, proof: generateProofResult)
  assert(isValid, "Proof verification should succeed")


} catch let error as MoproError {
  print("MoproError: \(error)")
  throw error
} catch {
  print("Unexpected error: \(error)")
  throw error
}
