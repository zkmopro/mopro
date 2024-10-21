import Foundation
import mopro

func serializeOutputs(_ stringArray: [String]) -> [UInt8] {
  var bytesArray: [UInt8] = []
  let length = stringArray.count
  var littleEndianLength = length.littleEndian
  let targetLength = 32
  withUnsafeBytes(of: &littleEndianLength) {
    bytesArray.append(contentsOf: $0)
  }
  for value in stringArray {
    // TODO: should handle 254-bit input
    var littleEndian = Int32(value)!.littleEndian
    var byteLength = 0
    withUnsafeBytes(of: &littleEndian) {
      bytesArray.append(contentsOf: $0)
      byteLength = byteLength + $0.count
    }
    if byteLength < targetLength {
      let paddingCount = targetLength - byteLength
      let paddingArray = [UInt8](repeating: 0, count: paddingCount)
      bytesArray.append(contentsOf: paddingArray)
    }
  }
  return bytesArray
}

do {
  let ar1csPath = "../../../test-vectors/ashlang/example.ar1cs"

  // Prepare inputs
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
