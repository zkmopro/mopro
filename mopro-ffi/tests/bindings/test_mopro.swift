import mopro
import Foundation

let moproCircom = MoproCircom()

let wasmPath = "./../../../../mopro-core/examples/circom/target/multiplier2_js/multiplier2.wasm"
let r1csPath = "./../../../../mopro-core/examples/circom/target/multiplier2.r1cs"

// TODO: should handle 254-bit input
func serializeOutputs(_ int32Array: [Int32]) -> [UInt8] {
    var bytesArray: [UInt8] = []
    let length = int32Array.count
    var littleEndianLength = length.littleEndian
    let targetLength = 32
    withUnsafeBytes(of: &littleEndianLength) {
        bytesArray.append(contentsOf: $0)
    }
    for value in int32Array {
        var littleEndian = value.littleEndian
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
    // Setup
    let setupResult = try moproCircom.setup(wasmPath: wasmPath, r1csPath: r1csPath)
    assert(!setupResult.provingKey.isEmpty, "Proving key should not be empty")

    // Prepare inputs
    var inputs = [String: [Int32]]()
    inputs["a"] = [3]
    inputs["b"] = [5]

    // Expected outputs
    let outputs: [Int32] = [15, 3]
    let expectedOutput: [UInt8] = serializeOutputs(outputs)

    // Generate Proof
    let generateProofResult = try moproCircom.generateProof(circuitInputs: inputs)
    assert(!generateProofResult.proof.isEmpty, "Proof should not be empty")

    // Verify Proof
    assert(Data(expectedOutput) == generateProofResult.inputs, "Circuit outputs mismatch the expected outputs")

    let isValid = try moproCircom.verifyProof(proof: generateProofResult.proof, publicInput: generateProofResult.inputs)
    assert(isValid, "Proof verification should succeed")

} catch let error as MoproError {
    print("MoproError: \(error)")
} catch {
    print("Unexpected error: \(error)")
}
