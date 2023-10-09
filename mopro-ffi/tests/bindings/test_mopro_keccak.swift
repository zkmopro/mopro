import mopro

let moproCircom = MoproCircom()


let wasmPath = "./../../../../mopro-core/examples/circom/keccak256/target/keccak256_256_test_js/keccak256_256_test.wasm"
let r1csPath = "./../../../../mopro-core/examples/circom/keccak256/target/keccak256_256_test.r1cs"

// Helper function to convert bytes to bits
func bytesToBits(bytes: [UInt8]) -> [Int32] {
    var bits = [Int32]()
    for byte in bytes {
        for j in 0..<8 {
            let bit = (byte >> j) & 1
            bits.append(Int32(bit))
        }
    }
    return bits
}

do {
    // Setup
    let setupResult = try moproCircom.setup(wasmPath: wasmPath, r1csPath: r1csPath)
    assert(!setupResult.provingKey.isEmpty, "Proving key should not be empty")

    // Prepare inputs
    let inputVec: [UInt8] = [
        116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0,
    ]
    let bits = bytesToBits(bytes: inputVec)
    var inputs = [String: [Int32]]()
    inputs["in"] = bits

    // Generate Proof
    let generateProofResult = try moproCircom.generateProof(circuitInputs: inputs)
    assert(!generateProofResult.proof.isEmpty, "Proof should not be empty")

    // Verify Proof
    let isValid = try moproCircom.verifyProof(proof: generateProofResult.proof, publicInput: generateProofResult.inputs)
    assert(isValid, "Proof verification should succeed")

} catch let error as MoproError {
    print("MoproError: \(error)")
} catch {
    print("Unexpected error: \(error)")
}
