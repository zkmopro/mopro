import Foundation
// GENERATED LIB IMPORT PLACEHOLDER

do {
    let r1csPath = "../../../test-vectors/gnark/cubic_circuit.r1cs"
    let pkPath = "../../../test-vectors/gnark/cubic_circuit.pk"
    let vkPath = "../../../test-vectors/gnark/cubic_circuit.vk"

    // x=3: x^3 + x + 5 = 35
    let witnessJson = "{\"X\": \"3\", \"Y\": \"35\"}"

    // Generate Proof
    let proofResult = try generateGnarkProof(
        r1csPath: r1csPath, pkPath: pkPath, witnessJson: witnessJson)
    assert(!proofResult.proof.isEmpty, "Proof should not be empty")
    assert(!proofResult.publicInputs.isEmpty, "Public inputs should not be empty")

    // Verify Proof
    let isValid = try verifyGnarkProof(
        r1csPath: r1csPath, vkPath: vkPath, proofResult: proofResult)
    assert(isValid, "Proof verification should succeed")

} catch let error as MoproError {
    print("MoproError: \(error)")
    throw error
} catch {
    print("Unexpected error: \(error)")
    throw error
}
