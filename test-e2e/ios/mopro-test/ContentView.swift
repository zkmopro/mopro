//
//  ContentView.swift
//  mopro-test
//
//  Created by Chance on 6/25/24.
//
import SwiftUI

struct ContentView: View {
    @State private var textViewText = ""
    @State private var isCircomProveButtonEnabled = true
    @State private var isCircomVerifyButtonEnabled = false
    @State private var isRapidsnarkProveButtonEnabled = true
    @State private var isRapidsnarkVerifyButtonEnabled = false
    @State private var isHalo2ProveButtonEnabled = true
    @State private var isHalo2VerifyButtonEnabled = false
    @State private var isNoirProveButtonEnabled = true
    @State private var isNoirVerifyButtonEnabled = false
    @State private var generatedCircomProof: CircomProof?
    @State private var circomPublicInputs: [String]?
    @State private var generatedRapidsnarkProof: CircomProof?
    @State private var rapidsnarkPublicInputs: [String]?
    @State private var generatedHalo2Proof: Data?
    @State private var halo2PublicInputs: Data?
    @State private var generatedNoirProof: Data?
    @State private var noirVerificationKey: Data?
    private let zkeyPath = Bundle.main.path(forResource: "multiplier2_final", ofType: "zkey")!
    private let witnesscalc_zkeyPath = Bundle.main.path(forResource: "multiplier2_witnesscalc_final", ofType: "zkey")!
    private let srsPath = Bundle.main.path(forResource: "plonk_fibonacci_srs.bin", ofType: "")!
    private let vkPath = Bundle.main.path(forResource: "plonk_fibonacci_vk.bin", ofType: "")!
    private let pkPath = Bundle.main.path(forResource: "plonk_fibonacci_pk.bin", ofType: "")!
    private let noirSrsPath = Bundle.main.path(forResource: "noir_multiplier2", ofType: "srs")!
    private let noirCircuitPath = Bundle.main.path(forResource: "noir_multiplier2", ofType: "json")!
    private let noirVkPath = Bundle.main.path(forResource: "noir_multiplier2", ofType: "vk")!
   
    var body: some View {
        VStack(spacing: 10) {
            Image(systemName: "globe")
                .imageScale(.large)
                .foregroundStyle(.tint)
            Button("Prove Circom", action: runCircomProveAction).disabled(!isCircomProveButtonEnabled).accessibilityIdentifier("proveCircom")
            Button("Verify Circom", action: runCircomVerifyAction).disabled(!isCircomVerifyButtonEnabled).accessibilityIdentifier("verifyCircom")
            Button("Prove Circom (Rapidsnark)", action: runRapidsnarkProveAction).disabled(!isRapidsnarkProveButtonEnabled).accessibilityIdentifier("proveRapidsnark")
            Button("Verify Circom (Rapidsnark)", action: runRapidsnarkVerifyAction).disabled(!isRapidsnarkVerifyButtonEnabled).accessibilityIdentifier("verifyRapidsnark")
            Button("Prove Halo2", action: runHalo2ProveAction).disabled(!isHalo2ProveButtonEnabled).accessibilityIdentifier("proveHalo2")
            Button("Verify Halo2", action: runHalo2VerifyAction).disabled(!isHalo2VerifyButtonEnabled).accessibilityIdentifier("verifyHalo2")
            Button("Prove Noir", action: runNoirProveAction).disabled(!isNoirProveButtonEnabled).accessibilityIdentifier("proveNoir")
            Button("Verify Noir", action: runNoirVerifyAction).disabled(!isNoirVerifyButtonEnabled).accessibilityIdentifier("verifyNoir")


            ScrollView {
                Text(textViewText)
                    .padding()
                    .accessibilityIdentifier("proof_log")
            }
            .frame(height: 200)
        }
        .padding()
    }
}

extension ContentView {
    func runCircomProveAction() {
        textViewText += "Generating Circom proof... "
        do {
            // Prepare inputs
            let a = 3
            let b = 5
            let c = a*b
            let input_str: String = "{\"b\":[\"5\"],\"a\":[\"3\"]}"
            
            // Expected outputs
            let outputs: [String] = [String(c), String(a)]
            
            let start = CFAbsoluteTimeGetCurrent()
            
            // Generate Proof
            let generateProofResult = try generateCircomProof(zkeyPath: zkeyPath, circuitInputs: input_str, proofLib: ProofLib.arkworks)
            assert(!generateProofResult.proof.a.x.isEmpty, "Proof should not be empty")
            assert(outputs == generateProofResult.inputs, "Circuit outputs mismatch the expected outputs")
            
            let end = CFAbsoluteTimeGetCurrent()
            let timeTaken = end - start
            
            // Store the generated proof and public inputs for later verification
            generatedCircomProof = generateProofResult.proof
            circomPublicInputs = generateProofResult.inputs
            
            textViewText += "\(String(format: "%.3f", timeTaken))s 1️⃣\n"
            
            isCircomVerifyButtonEnabled = true
        } catch {
            textViewText += "\nProof generation failed: \(error.localizedDescription)\n"
        }
    }
    
    func runCircomVerifyAction() {
        guard let proof = generatedCircomProof,
              let inputs = circomPublicInputs else {
            textViewText += "Proof has not been generated yet.\n"
            return
        }
        
        textViewText += "Verifying Circom proof... "
        do {
            let start = CFAbsoluteTimeGetCurrent()
            
            let isValid = try verifyCircomProof(zkeyPath: zkeyPath, proofResult: CircomProofResult(proof: proof, inputs: inputs), proofLib: ProofLib.arkworks)
            let end = CFAbsoluteTimeGetCurrent()
            let timeTaken = end - start
            
            assert(proof.a.x.count > 0, "Proof should not be empty")
            assert(inputs.count > 0, "Inputs should not be empty")
            
            print("Ethereum Proof: \(proof)\n")
            print("Ethereum Inputs: \(inputs)\n")
            
            if isValid {
                textViewText += "\(String(format: "%.3f", timeTaken))s 2️⃣\n"
            } else {
                textViewText += "\nProof verification failed.\n"
            }
            isCircomVerifyButtonEnabled = false
        } catch let error as MoproError {
            print("\nMoproError: \(error)")
        } catch {
            print("\nUnexpected error: \(error)")
        }
    }
    
    func runRapidsnarkProveAction() {
        textViewText += "Generating Circom Rapidsnark proof... "
        do {
            // Prepare inputs
            let a = 3
            let b = 5
            let c = a*b
            let input_str: String = "{\"b\":[\"5\"],\"a\":[\"3\"]}"
            
            // Expected outputs
            let outputs: [String] = [String(c), String(a)]
            
            let start = CFAbsoluteTimeGetCurrent()
            
            // Generate Proof
            let generateProofResult = try generateCircomProof(zkeyPath: witnesscalc_zkeyPath, circuitInputs: input_str, proofLib: ProofLib.rapidsnark)
            assert(!generateProofResult.proof.a.x.isEmpty, "Proof should not be empty")
            assert(outputs == generateProofResult.inputs, "Circuit outputs mismatch the expected outputs")
            
            let end = CFAbsoluteTimeGetCurrent()
            let timeTaken = end - start
            
            // Store the generated proof and public inputs for later verification
            generatedCircomProof = generateProofResult.proof
            circomPublicInputs = generateProofResult.inputs
            
            textViewText += "\(String(format: "%.3f", timeTaken))s 1️⃣\n"
            
            isRapidsnarkVerifyButtonEnabled = true
        } catch {
            textViewText += "\nProof generation failed: \(error.localizedDescription)\n"
        }
    }
    
    func runRapidsnarkVerifyAction() {
        guard let proof = generatedCircomProof,
              let inputs = circomPublicInputs else {
            textViewText += "Proof has not been generated yet.\n"
            return
        }
        
        textViewText += "Verifying Circom Rapidsnark proof... "
        do {
            let start = CFAbsoluteTimeGetCurrent()
            
            let isValid = try verifyCircomProof(zkeyPath: witnesscalc_zkeyPath, proofResult: CircomProofResult(proof: proof, inputs: inputs), proofLib: ProofLib.rapidsnark)
            let end = CFAbsoluteTimeGetCurrent()
            let timeTaken = end - start
            
            assert(proof.a.x.count > 0, "Proof should not be empty")
            assert(inputs.count > 0, "Inputs should not be empty")
            
            print("Ethereum Proof: \(proof)\n")
            print("Ethereum Inputs: \(inputs)\n")
            
            if isValid {
                textViewText += "\(String(format: "%.3f", timeTaken))s 2️⃣\n"
            } else {
                textViewText += "\nProof verification failed.\n"
            }
            isCircomVerifyButtonEnabled = false
        } catch let error as MoproError {
            print("\nMoproError: \(error)")
        } catch {
            print("\nUnexpected error: \(error)")
        }
    }
    
    func runHalo2ProveAction() {
        textViewText += "Generating Halo2 proof... "
        do {
            // Prepare inputs
            var inputs = [String: [String]]()
            let out = 55
            inputs["out"] = [String(out)]
            
            let start = CFAbsoluteTimeGetCurrent()
            
            // Generate Proof
            let generateProofResult = try generateHalo2Proof(srsPath: srsPath, pkPath: pkPath, circuitInputs: inputs)
            assert(!generateProofResult.proof.isEmpty, "Proof should not be empty")
            assert(!generateProofResult.inputs.isEmpty, "Inputs should not be empty")

            
            let end = CFAbsoluteTimeGetCurrent()
            let timeTaken = end - start
            
            // Store the generated proof and public inputs for later verification
            generatedHalo2Proof = generateProofResult.proof
            halo2PublicInputs = generateProofResult.inputs
            
            textViewText += "\(String(format: "%.3f", timeTaken))s 1️⃣\n"
            
            isHalo2VerifyButtonEnabled = true
        } catch {
            textViewText += "\nProof generation failed: \(error.localizedDescription)\n"
        }
    }
    
    func runHalo2VerifyAction() {
        guard let proof = generatedHalo2Proof,
              let inputs = halo2PublicInputs else {
            textViewText += "Proof has not been generated yet.\n"
            return
        }
        
        textViewText += "Verifying Halo2 proof... "
        do {
            let start = CFAbsoluteTimeGetCurrent()
            
            let isValid = try verifyHalo2Proof(
              srsPath: srsPath, vkPath: vkPath, proof: proof, publicInput: inputs)
            let end = CFAbsoluteTimeGetCurrent()
            let timeTaken = end - start

            
            if isValid {
                textViewText += "\(String(format: "%.3f", timeTaken))s 2️⃣\n"
            } else {
                textViewText += "\nProof verification failed.\n"
            }
            isHalo2VerifyButtonEnabled = false
        } catch let error as MoproError {
            print("\nMoproError: \(error)")
        } catch {
            print("\nUnexpected error: \(error)")
        }
    }
    
    func runNoirProveAction() {
        textViewText += "Generating Noir proof...\n"

        do {
            let inputs: [String] = ["3", "5"]
            let onChain = true  // Use Keccak for Solidity compatibility
            let lowMemoryMode = false

            DispatchQueue.global(qos: .userInitiated).async {
                do {
                    // First, try to load existing verification key from file, or generate new one
                    let vk: Data
                    if let existingVkData = try? Data(contentsOf: URL(fileURLWithPath: noirVkPath)) {
                        vk = existingVkData
                        DispatchQueue.main.async {
                            textViewText += "Using existing verification key...\n"
                        }
                    } else {
                        DispatchQueue.main.async {
                            textViewText += "Generating verification key...\n"
                        }
                        vk = try getNoirVerificationKey(circuitPath: noirCircuitPath, srsPath: noirSrsPath, onChain: onChain, lowMemoryMode: lowMemoryMode)
                    }
                    noirVerificationKey = vk

                    DispatchQueue.main.async {
                        textViewText += "Generating proof with verification key...\n"
                    }
                    let start = CFAbsoluteTimeGetCurrent()
                    
                    // Generate the proof with all required parameters
                    let proofData = try generateNoirProof(
                        circuitPath: noirCircuitPath, 
                        srsPath: noirSrsPath, 
                        inputs: inputs, 
                        onChain: onChain, 
                        vk: vk, 
                        lowMemoryMode: lowMemoryMode
                    )

                    let end = CFAbsoluteTimeGetCurrent()
                    let timeTaken = end - start

                    DispatchQueue.main.async {
                        generatedNoirProof = proofData
                        textViewText += "\(String(format: "%.3f", timeTaken))s 1️⃣\n"
                        isNoirVerifyButtonEnabled = true
                    }
                } catch {
                    DispatchQueue.main.async {
                        textViewText += "Proof generation failed: \(error.localizedDescription)\n"
                    }
                }
            }

        } catch {
            textViewText += "Error setting up proof generation: \(error.localizedDescription)\n"
        }
    }
    
    func runNoirVerifyAction() {
        guard let proofData = generatedNoirProof else {
            textViewText += "Error: Proof data is not available. Generate proof first.\n"
            return
        }
        
        guard let vk = noirVerificationKey else {
            textViewText += "Error: Verification key is not available. Generate proof first.\n"
            return
        }

        textViewText += "Verifying Noir proof...\n"

        DispatchQueue.global(qos: .userInitiated).async {
            let start = CFAbsoluteTimeGetCurrent()
            do {
                let onChain = true  // Use Keccak for Solidity compatibility
                let lowMemoryMode = false
                
                // Verify the proof with all required parameters
                let isValid = try verifyNoirProof(
                    circuitPath: noirCircuitPath, 
                    proof: proofData, 
                    onChain: onChain, 
                    vk: vk, 
                    lowMemoryMode: lowMemoryMode
                )

                let end = CFAbsoluteTimeGetCurrent()
                let timeTaken = end - start

                DispatchQueue.main.async {
                    if isValid {
                        textViewText += "\(String(format: "%.3f", timeTaken))s 2️⃣\n"
                    } else {
                        textViewText += "\nProof verification failed.\n"
                    }
                    isNoirVerifyButtonEnabled = false
                }
            } catch {
                DispatchQueue.main.async {
                    textViewText += "Verification failed: \(error.localizedDescription)\n"
                }
            }
        }
    }
}

