import Foundation
import mopro

do {
    let srsPath = "../../../test-vectors/halo2/hyperplonk_fibonacci_srs.bin"
    let vkPath = "../../../test-vectors/halo2/hyperplonk_fibonacci_vk.bin"
    let pkPath = "../../../test-vectors/halo2/hyperplonk_fibonacci_pk.bin"

    // Prepare inputs
    var inputs = [String: [String]]()
    let out = 55
    inputs["out"] = [String(out)]


    // Generate Proof
    let generateProofResult = try generateHalo2Proof(srsPath: srsPath, pkPath: pkPath, circuitInputs: inputs)
    assert(!generateProofResult.proof.isEmpty, "Proof should not be empty")

    // Verify Proof
    assert(
        !generateProofResult.inputs.isEmpty,
        "Circuit inputs are empty")

    let isValid = try verifyHalo2Proof(
        srsPath: srsPath, vkPath: vkPath, proof: generateProofResult.proof, publicInput: generateProofResult.inputs)
    assert(isValid, "Proof verification should succeed")


} catch let error as MoproError {
    print("MoproError: \(error)")
    throw error
} catch {
    print("Unexpected error: \(error)")
    throw error
}
