import java.io.File
import uniffi.mopro_bindings.*

fun testNoirMultiplier2() {
    try {
        // Test inputs
        val inputs = listOf("3", "5")
        val onChain = true  // Use Keccak for Solidity compatibility
        val lowMemoryMode = false
        
        // Circuit and SRS paths (these would be test vectors in a real scenario)
        val circuitPath = "noir_multiplier2.json"
        val srsPath = "noir_multiplier2.srs"
        
        // Get verification key
        val vk = getNoirVerificationKey(
            circuitPath = circuitPath,
            srsPath = srsPath,
            onChain = onChain,
            lowMemoryMode = lowMemoryMode
        )
        
        require(vk.isNotEmpty()) { "Verification key should not be empty" }
        
        // Generate proof
        val proof = generateNoirProof(
            circuitPath = circuitPath,
            srsPath = srsPath,
            inputs = inputs,
            onChain = onChain,
            vk = vk,
            lowMemoryMode = lowMemoryMode
        )
        
        require(proof.isNotEmpty()) { "Proof should not be empty" }
        
        // Verify proof
        val isValid = verifyNoirProof(
            circuitPath = circuitPath,
            proof = proof,
            onChain = onChain,
            vk = vk,
            lowMemoryMode = lowMemoryMode
        )
        
        require(isValid) { "Proof should be valid" }
        
        println("✅ Noir multiplier2 test passed!")
        
    } catch (e: Exception) {
        println("❌ Noir multiplier2 test failed: ${e.message}")
        throw AssertionError("Test should not fail", e)
    }
}

fun testNoirMultiplier2WithExistingVk() {
    try {
        // Test inputs
        val inputs = listOf("3", "5")
        val onChain = true  // Use Keccak for Solidity compatibility
        val lowMemoryMode = false
        
        // Circuit and file paths
        val circuitPath = "noir_multiplier2.json"
        val srsPath = "noir_multiplier2.srs"
        val vkPath = "noir_multiplier2.vk"
        
        // Read existing verification key from file
        val vkData = File(vkPath).readBytes()
        
        require(vkData.isNotEmpty()) { "Verification key data should not be empty" }
        
        // Generate proof with existing verification key
        val proof = generateNoirProof(
            circuitPath = circuitPath,
            srsPath = srsPath,
            inputs = inputs,
            onChain = onChain,
            vk = vkData,
            lowMemoryMode = lowMemoryMode
        )
        
        require(proof.isNotEmpty()) { "Proof should not be empty" }
        
        // Verify proof with existing verification key
        val isValid = verifyNoirProof(
            circuitPath = circuitPath,
            proof = proof,
            onChain = onChain,
            vk = vkData,
            lowMemoryMode = lowMemoryMode
        )
        
        require(isValid) { "Proof should be valid" }
        
        println("✅ Noir multiplier2 with existing VK test passed!")
        
    } catch (e: Exception) {
        println("❌ Noir multiplier2 with existing VK test failed: ${e.message}")
        throw AssertionError("Test should not fail", e)
    }
}

// Run the tests
testNoirMultiplier2()
testNoirMultiplier2WithExistingVk()