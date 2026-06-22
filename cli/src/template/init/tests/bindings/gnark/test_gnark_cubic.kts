// GENERATED LIB IMPORT PLACEHOLDER

try {
    val r1csPath = "./test-vectors/gnark/cubic_circuit.r1cs"
    val groth16PkPath = "./test-vectors/gnark/cubic_circuit.pk"
    val groth16VkPath = "./test-vectors/gnark/cubic_circuit.vk"

    val scsPath = "./test-vectors/gnark/cubic_circuit_plonk.scs"
    val plonkPkPath = "./test-vectors/gnark/cubic_circuit_plonk.pk"
    val plonkVkPath = "./test-vectors/gnark/cubic_circuit_plonk.vk"

    // x=3: x^3 + x + 5 = 35
    val witnessJson = "{\"X\": \"3\", \"Y\": \"35\"}"

    // --- Groth16 ---
    val groth16Result = generateGnarkProof(r1csPath, groth16PkPath, witnessJson)
    assert(groth16Result.proof.isNotEmpty()) { "Groth16 proof should not be empty" }
    assert(groth16Result.publicInputs.isNotEmpty()) { "Groth16 public inputs should not be empty" }

    val groth16Valid = verifyGnarkProof(r1csPath, groth16VkPath, groth16Result)
    assert(groth16Valid) { "Groth16 proof is invalid" }

    // --- PLONK ---
    val plonkResult = generateGnarkPlonkProof(scsPath, plonkPkPath, witnessJson)
    assert(plonkResult.proof.isNotEmpty()) { "PLONK proof should not be empty" }
    assert(plonkResult.publicInputs.isNotEmpty()) { "PLONK public inputs should not be empty" }

    val plonkValid = verifyGnarkPlonkProof(scsPath, plonkVkPath, plonkResult)
    assert(plonkValid) { "PLONK proof is invalid" }

} catch (e: Exception) {
    println(e)
    throw e
}
