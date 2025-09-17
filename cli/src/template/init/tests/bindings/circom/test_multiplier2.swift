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
    let zkeyPath = "../../../test-vectors/circom/multiplier2_final.zkey"

    // Prepare inputs
    var inputs = [String: [String]]()
    let a = 3
    let b = 5
    let c = a * b
    inputs["a"] = [String(a)]
    inputs["b"] = [String(b)]
    let input_str: String = (try? JSONSerialization.data(withJSONObject: inputs, options: .prettyPrinted)).flatMap {
        String(data: $0, encoding: .utf8)
    } ?? ""

    // Expected outputs
    let outputs: [String] = [String(c), String(a)]

    // Generate Proof
    let generateProofResult = try generateCircomProof(
        zkeyPath: zkeyPath, circuitInputs: input_str, proofLib: ProofLib.arkworks)
    assert(!generateProofResult.proof.a.x.isEmpty, "Proof should not be empty")

    // Verify Proof
    assert(
        outputs == generateProofResult.inputs,
        "Circuit outputs mismatch the expected outputs")

    let isValid = try verifyCircomProof(zkeyPath: zkeyPath, proofResult: generateProofResult, proofLib: ProofLib.arkworks)
    assert(isValid, "Proof verification should succeed")

    assert(generateProofResult.proof.a.x.count > 0, "Proof should not be empty")
    assert(generateProofResult.inputs.count > 0, "Inputs should not be empty")

} catch let error as MoproError {
    print("MoproError: \(error)")
    throw error
} catch {
    print("Unexpected error: \(error)")
    throw error
}
