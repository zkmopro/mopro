//
//  ContentView.swift
//  mopro-test
//
//  Created by Chance on 6/25/24.
//
import SwiftUI
import moproFFI

func serializeOutputs(_ stringArray: [String]) -> [UInt8] {
    var bytesArray: [UInt8] = []
    let length = stringArray.count
    var littleEndianLength = length.littleEndian
    let targetLength = 32
    withUnsafeBytes(of: &littleEndianLength) {
        bytesArray.append(contentsOf: $0)
    }
    for value in stringArray {
        // TODO: should handle 254-bit input
        var littleEndian = Int32(value)!.littleEndian
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

struct HeaderTest: Decodable {
    let storage: [UInt8]
    let len: UInt32
}

struct PubKeyTest: Decodable {
    let modulus: [String]
    let redc: [String]
}

struct SequenceTest: Decodable {
    let index: UInt32
    let length: UInt32
}

// Structs for decoding zkemail_input.json
struct ZkEmailInputTest: Decodable {
    let header: HeaderTest
    let pubkey: PubKeyTest
    let signature: [String]
    let date_index: UInt32
    let subject_sequence: SequenceTest
    let from_header_sequence: SequenceTest
    let from_address_sequence: SequenceTest
}


struct ContentView: View {
    @State private var textViewText = ""
    @State private var isCircomProveButtonEnabled = true
    @State private var isCircomVerifyButtonEnabled = false
    @State private var isHalo2roveButtonEnabled = true
    @State private var isHalo2VerifyButtonEnabled = false
    @State private var isNoirProveButtonEnabled = true
    @State private var isNoirVerifyButtonEnabled = false
    @State private var generatedCircomProof: CircomProof?
    @State private var circomPublicInputs: [String]?
    @State private var generatedHalo2Proof: Data?
    @State private var halo2PublicInputs: Data?
    @State private var generatedNoirProof: Data?
    private let zkeyPath = Bundle.main.path(forResource: "multiplier2_final", ofType: "zkey")!
    private let srsPath = Bundle.main.path(forResource: "plonk_fibonacci_srs.bin", ofType: "")!
    private let vkPath = Bundle.main.path(forResource: "plonk_fibonacci_vk.bin", ofType: "")!
    private let pkPath = Bundle.main.path(forResource: "plonk_fibonacci_pk.bin", ofType: "")!
    private let zkemailSrsPath = Bundle.main.path(forResource: "zkemail_srs", ofType: "local")!
    private let zkemailCircuitPath = Bundle.main.path(forResource: "zkemail", ofType: "json")!
    
    var body: some View {
        VStack(spacing: 10) {
            Image(systemName: "globe")
                .imageScale(.large)
                .foregroundStyle(.tint)
            Button("Prove Circom", action: runCircomProveAction).disabled(!isCircomProveButtonEnabled).accessibilityIdentifier("proveCircom")
            Button("Verify Circom", action: runCircomVerifyAction).disabled(!isCircomVerifyButtonEnabled).accessibilityIdentifier("verifyCircom")
            Button("Prove Halo2", action: runHalo2ProveAction).disabled(!isHalo2roveButtonEnabled).accessibilityIdentifier("proveHalo2")
            Button("Verify Halo2", action: runHalo2VerifyAction).disabled(!isHalo2VerifyButtonEnabled).accessibilityIdentifier("verifyHalo2")
            Button("Prove Noir(zkemail)", action: runNoirProveAction).disabled(!isNoirProveButtonEnabled).accessibilityIdentifier("proveNoir")
            Button("Verify Noir(zkemail)", action: runNoirVerifyAction).disabled(!isNoirVerifyButtonEnabled).accessibilityIdentifier("verifyNoir")

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
        textViewText += "Generating zkEmail proof...\n"
        
        // Get the path to the SRS file in the app bundle
        guard let srsPath = Bundle.main.path(forResource: "zkemail_srs", ofType: "local") else {
            textViewText += "Error: Could not find SRS file in app bundle\n"
            return
        }
        
        // Get the path to the input JSON file
        guard let inputJsonPath = Bundle.main.path(forResource: "zkemail_input", ofType: "json") else {
            textViewText += "Error: Could not find zkemail_input.json in app bundle\n"
            return
        }
        
        // Load and parse JSON
        do {
            let jsonData = try Data(contentsOf: URL(fileURLWithPath: inputJsonPath))
            let decoder = JSONDecoder()
            let inputData = try decoder.decode(ZkEmailInputTest.self, from: jsonData)
            
            // Convert to the format expected by proveZkemail
            let inputs: [String] =
                inputData.header.storage.map { String($0) } +
                [String(inputData.header.len)] +
                inputData.pubkey.modulus +
                inputData.pubkey.redc +
                inputData.signature +
                [String(inputData.date_index)] +
                [String(inputData.subject_sequence.index)] +
                [String(inputData.subject_sequence.length)] +
                [String(inputData.from_header_sequence.index)] +
                [String(inputData.from_header_sequence.length)] +
                [String(inputData.from_address_sequence.index)] +
                [String(inputData.from_address_sequence.length)]
            
            
            // Run in background thread
            DispatchQueue.global(qos: .userInitiated).async {
                let start = CFAbsoluteTimeGetCurrent()
                do {
                    // Generate the proof
                    let proofData = try! generateNoirProof(circuitPath: zkemailCircuitPath, srsPath: zkemailSrsPath, inputs: inputs)
                    
                    let end = CFAbsoluteTimeGetCurrent()
                    let timeTaken = end - start
                    
                    generatedNoirProof = proofData
                    textViewText += "\(String(format: "%.3f", timeTaken))s 1️⃣\n"
                    
                    isNoirVerifyButtonEnabled = true
                } catch {
                    textViewText += "Proof generation failed: \(error.localizedDescription)\n"
                }
            }
            
        } catch {
            textViewText += "Error loading or parsing input JSON: \(error.localizedDescription)\n"
        }
    }

    func runNoirVerifyAction() {
        guard let proofData = generatedNoirProof else {
            textViewText += "Error: Proof data is not available. Generate proof first.\n"
            return
        }

        textViewText += "Verifying zkEmail proof...\n"
        
        // Get the path to the SRS file in the app bundle
        guard let srsPath = Bundle.main.path(forResource: "zkemail_srs", ofType: "local") else {
            textViewText += "Error: Could not find SRS file in app bundle\n"
            return
        }

        DispatchQueue.global(qos: .userInitiated).async {
            let start = CFAbsoluteTimeGetCurrent()
            do {
                // Verify the proof
                let isValid = try! verifyNoirProof(circuitPath: zkemailCircuitPath, proof: proofData)

                let end = CFAbsoluteTimeGetCurrent()
                let timeTaken = end - start

                if isValid {
                    textViewText += "\(String(format: "%.3f", timeTaken))s 2️⃣\n"
                } else {
                    textViewText += "\nProof verification failed.\n"
                }

                isNoirVerifyButtonEnabled = false
            } catch {
                self.textViewText += "Verification failed: \(error.localizedDescription)\n"
            }
        }
    }
}

