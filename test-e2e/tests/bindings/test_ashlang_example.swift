import Foundation
import mopro

do {
  let ar1csPath = "../../../test-vectors/ashlang/example.ar1cs"

  // this number is used but not constrained
  var inputs = [String(55)]

  // Generate Proof
  let generateProofResult = try generateAshlangSpartanProof(ar1csPath: ar1csPath, inputs: inputs)

  assert(!generateProofResult.proof.isEmpty, "Proof should not be empty")

  // Verify Proof
  let isValid = try verifyAshlangSpartanProof(
    ar1csPath: ar1csPath, proof: generateProofResult.proof)

  assert(isValid, "Proof verification should succeed")

} catch let error as MoproError {
  print("MoproError: \(error)")
  throw error
} catch {
  print("Unexpected error: \(error)")
  throw error
}
