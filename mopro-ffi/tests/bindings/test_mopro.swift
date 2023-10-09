import mopro

let moproCircom = MoproCircom()

let wasmPath = "./../mopro-core/examples/circom/target/multiplier2_js/multiplier2.wasm"
let r1csPath = "./../mopro-core/examples/circom/target/multiplier2.r1cs"

do {
    // Setup
    let setupResult = try moproCircom.setup(wasmPath: wasmPath, r1csPath: r1csPath)
    assert(!setupResult.provingKey.isEmpty, "Proving key should not be empty")

    // Prepare inputs
    var inputs = [String: [Int32]]()
    inputs["a"] = [3]
    inputs["b"] = [5]

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
