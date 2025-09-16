import Foundation
import mopro_bindings

// Test the generateNoirProof and verifyNoirProof functions
func testNoirMultiplier2() {
    do {
        // Test inputs
        let inputs: [String] = ["3", "5"]
        let onChain = true  // Use Keccak for Solidity compatibility
        let lowMemoryMode = false
        
        // Circuit and SRS paths (these would be test vectors in a real scenario)
        let circuitPath = "noir_multiplier2.json"
        let srsPath = "noir_multiplier2.srs"
        
        // Get verification key
        let vk = try getNoirVerificationKey(
            circuitPath: circuitPath,
            srsPath: srsPath, 
            onChain: onChain,
            lowMemoryMode: lowMemoryMode
        )
        
        assert(!vk.isEmpty, "Verification key should not be empty")
        
        // Generate proof
        let proof = try generateNoirProof(
            circuitPath: circuitPath,
            srsPath: srsPath,
            inputs: inputs,
            onChain: onChain,
            vk: vk,
            lowMemoryMode: lowMemoryMode
        )
        
        assert(!proof.isEmpty, "Proof should not be empty")
        
        // Verify proof
        let isValid = try verifyNoirProof(
            circuitPath: circuitPath,
            proof: proof,
            onChain: onChain,
            vk: vk,
            lowMemoryMode: lowMemoryMode
        )
        
        assert(isValid, "Proof should be valid")
        
        print("✅ Noir multiplier2 test passed!")
        
    } catch {
        print("❌ Noir multiplier2 test failed: \(error)")
        assert(false, "Test should not fail")
    }
}

// Test with existing verification key
func testNoirMultiplier2WithExistingVk() {
    do {
        // Test inputs
        let inputs: [String] = ["3", "5"]
        let onChain = true  // Use Keccak for Solidity compatibility
        let lowMemoryMode = false
        
        // Circuit and file paths
        let circuitPath = "noir_multiplier2.json"
        let srsPath = "noir_multiplier2.srs"
        let vkPath = "noir_multiplier2.vk"
        
        // Read existing verification key from file
        let vkData = try Data(contentsOf: URL(fileURLWithPath: vkPath))
        
        assert(!vkData.isEmpty, "Verification key data should not be empty")
        
        // Generate proof with existing verification key
        let proof = try generateNoirProof(
            circuitPath: circuitPath,
            srsPath: srsPath,
            inputs: inputs,
            onChain: onChain,
            vk: vkData,
            lowMemoryMode: lowMemoryMode
        )
        
        assert(!proof.isEmpty, "Proof should not be empty")
        
        // Verify proof with existing verification key
        let isValid = try verifyNoirProof(
            circuitPath: circuitPath,
            proof: proof,
            onChain: onChain,
            vk: vkData,
            lowMemoryMode: lowMemoryMode
        )
        
        assert(isValid, "Proof should be valid")
        
        print("✅ Noir multiplier2 with existing VK test passed!")
        
    } catch {
        print("❌ Noir multiplier2 with existing VK test failed: \(error)")
        assert(false, "Test should not fail")
    }
}

// Run the tests
testNoirMultiplier2()
testNoirMultiplier2WithExistingVk()