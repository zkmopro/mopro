import test_e2eFFI

import Foundation

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
    let circuit = Multiplier2CircomCircuit(zkeyPath)

    // Prepare inputs
    var inputs = [String: [String]]()
    let a = 3
    let b = 5
    let c = a*b
    inputs["a"] = [String(a)]
    inputs["b"] = [String(b)]

    // Expected outputs
    let outputs: [String] = [String(c), String(a)]
    let expectedOutput: [UInt8] = serializeOutputs(outputs)

    // Generate Proof
    let generateProofResult = try circuit.prove(circuitInputs: inputs)
    assert(!generateProofResult.proof.isEmpty, "Proof should not be empty")

    // Verify Proof
    assert(Data(expectedOutput) == generateProofResult.inputs, "Circuit outputs mismatch the expected outputs")

    let isValid = try circuit.verify(proof: generateProofResult.proof, publicInput: generateProofResult.inputs)
    assert(isValid, "Proof verification should succeed")

    // Convert proof to Ethereum compatible proof
    let convertProofResult = toEthereumProof(proof: generateProofResult.proof)
    let convertInputsResult = toEthereumInputs(inputs: generateProofResult.inputs)
    assert(convertProofResult.a.x.count > 0, "Proof should not be empty")
    assert(convertInputsResult.count > 0, "Inputs should not be empty")

} catch let error as MoproError {
    print("MoproError: \(error)")
    throw error
} catch {
    print("Unexpected error: \(error)")
    throw error
}
