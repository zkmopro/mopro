import Foundation
// GENERATED LIB IMPORT PLACEHOLDER

do {
    let r1csPath = "../../../test-vectors/gnark/cubic_circuit.r1cs"
    let groth16PkPath = "../../../test-vectors/gnark/cubic_circuit.pk"
    let groth16VkPath = "../../../test-vectors/gnark/cubic_circuit.vk"

    let scsPath = "../../../test-vectors/gnark/cubic_circuit_plonk.scs"
    let plonkPkPath = "../../../test-vectors/gnark/cubic_circuit_plonk.pk"
    let plonkVkPath = "../../../test-vectors/gnark/cubic_circuit_plonk.vk"

    // x=3: x^3 + x + 5 = 35
    let witnessJson = "{\"X\": \"3\", \"Y\": \"35\"}"

    // --- Groth16 ---
    let groth16Result = try generateGnarkProof(
        r1csPath: r1csPath, pkPath: groth16PkPath, witnessJson: witnessJson)
    assert(!groth16Result.proof.isEmpty, "Groth16 proof should not be empty")
    assert(!groth16Result.publicInputs.isEmpty, "Groth16 public inputs should not be empty")

    let groth16Valid = try verifyGnarkProof(
        r1csPath: r1csPath, vkPath: groth16VkPath, proofResult: groth16Result)
    assert(groth16Valid, "Groth16 proof verification should succeed")

    // --- PLONK ---
    let plonkResult = try generateGnarkPlonkProof(
        scsPath: scsPath, pkPath: plonkPkPath, witnessJson: witnessJson)
    assert(!plonkResult.proof.isEmpty, "PLONK proof should not be empty")
    assert(!plonkResult.publicInputs.isEmpty, "PLONK public inputs should not be empty")

    let plonkValid = try verifyGnarkPlonkProof(
        scsPath: scsPath, vkPath: plonkVkPath, proofResult: plonkResult)
    assert(plonkValid, "PLONK proof verification should succeed")

} catch let error as MoproError {
    print("MoproError: \(error)")
    throw error
} catch {
    print("Unexpected error: \(error)")
    throw error
}
