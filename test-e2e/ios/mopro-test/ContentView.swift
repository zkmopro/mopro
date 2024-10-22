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


struct ContentView: View {
    @State private var textViewText = ""
    @State private var isCircomProveButtonEnabled = true
    @State private var isCircomVerifyButtonEnabled = false
    @State private var isHalo2roveButtonEnabled = true
    @State private var isHalo2VerifyButtonEnabled = false
    @State private var isAshlangroveButtonEnabled = true
    @State private var isAshlangVerifyButtonEnabled = false
    @State private var generatedCircomProof: Data?
    @State private var circomPublicInputs: Data?
    @State private var generatedHalo2Proof: Data?
    @State private var halo2PublicInputs: Data?
    @State private var generatedAshlangProof: Data?
    @State private var ashlangPublicInputs: Data?
    private let zkeyPath = Bundle.main.path(forResource: "multiplier2_final", ofType: "zkey")!
    private let srsPath = Bundle.main.path(forResource: "fibonacci_srs.bin", ofType: "")!
    private let vkPath = Bundle.main.path(forResource: "fibonacci_vk.bin", ofType: "")!
    private let pkPath = Bundle.main.path(forResource: "fibonacci_pk.bin", ofType: "")!
    private let ar1csPath = Bundle.main.path(forResource: "example.ar1cs", ofType: "")!
    
    var body: some View {
        VStack(spacing: 10) {
            Image(systemName: "globe")
                .imageScale(.large)
                .foregroundStyle(.tint)
            Button("Prove Circom", action: runCircomProveAction).disabled(!isCircomProveButtonEnabled).accessibilityIdentifier("proveCircom")
            Button("Verify Circom", action: runCircomVerifyAction).disabled(!isCircomVerifyButtonEnabled).accessibilityIdentifier("verifyCircom")
            Button("Prove Halo2", action: runHalo2ProveAction).disabled(!isHalo2roveButtonEnabled).accessibilityIdentifier("proveHalo2")
            Button("Verify Halo2", action: runHalo2VerifyAction).disabled(!isHalo2VerifyButtonEnabled).accessibilityIdentifier("verifyHalo2")
            Button("Prove Ashlang", action: runAshlangProveAction).disabled(!isAshlangroveButtonEnabled).accessibilityIdentifier("proveAshlang")
            Button("Verify Ashlang", action: runAshlangVerifyAction).disabled(!isAshlangVerifyButtonEnabled).accessibilityIdentifier("verifyAshlang")

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
            var inputs = [String: [String]]()
            let a = 3
            let b = 5
            let c = a*b
            inputs["a"] = [String(a)]
            inputs["b"] = [String(b)]
            
            // Expected outputs
            let outputs: [String] = [String(c), String(a)]
            let expectedOutput: [UInt8] = serializeOutputs(outputs)
            
            let start = CFAbsoluteTimeGetCurrent()
            
            // Generate Proof
            let generateProofResult = try generateCircomProof(zkeyPath: zkeyPath, circuitInputs: inputs)
            assert(!generateProofResult.proof.isEmpty, "Proof should not be empty")
            assert(Data(expectedOutput) == generateProofResult.inputs, "Circuit outputs mismatch the expected outputs")
            
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
            
            let isValid = try verifyCircomProof(zkeyPath: zkeyPath, proof: proof, publicInput: inputs)
            let end = CFAbsoluteTimeGetCurrent()
            let timeTaken = end - start
            
            // Convert proof to Ethereum compatible proof
            let ethereumProof = toEthereumProof(proof: proof)
            let ethereumInputs = toEthereumInputs(inputs: inputs)
            assert(ethereumProof.a.x.count > 0, "Proof should not be empty")
            assert(ethereumInputs.count > 0, "Inputs should not be empty")
            
            print("Ethereum Proof: \(ethereumProof)\n")
            print("Ethereum Inputs: \(ethereumInputs)\n")
            
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

    func runAshlangProveAction() {
        textViewText += "Generating Ashlang proof... "
        do {
            // Prepare inputs
            var inputs = [String(55)]
            
            let start = CFAbsoluteTimeGetCurrent()
            
            // Generate Proof
            let generateProofResult = try generateAshlangSpartanProof( ar1csPath: ar1csPath, inputs: inputs)
            assert(!generateProofResult.proof.isEmpty, "Proof should not be empty")
            assert(!generateProofResult.inputs.isEmpty, "Inputs should not be empty")

            
            let end = CFAbsoluteTimeGetCurrent()
            let timeTaken = end - start
            
            // Store the generated proof and public inputs for later verification
            generatedAshlangProof = generateProofResult.proof
            ashlangPublicInputs = generateProofResult.inputs
            
            textViewText += "\(String(format: "%.3f", timeTaken))s 1️⃣\n"
            
            isAshlangVerifyButtonEnabled = true
        } catch {
            textViewText += "\nProof generation failed: \(error.localizedDescription)\n"
        }
    }
    
    func runAshlangVerifyAction() {
        guard let proof = generatedAshlangProof,
              let inputs = ashlangPublicInputs else {
            textViewText += "Proof has not been generated yet.\n"
            return
        }
        
        textViewText += "Verifying Ashlang proof... "
        do {
            let start = CFAbsoluteTimeGetCurrent()
            
            let isValid = try verifyAshlangSpartanProof(
                ar1csPath: ar1csPath, proof: generatedAshlangProof!)
            let end = CFAbsoluteTimeGetCurrent()
            let timeTaken = end - start

            
            if isValid {
                textViewText += "\(String(format: "%.3f", timeTaken))s 2️⃣\n"
            } else {
                textViewText += "\nProof verification failed.\n"
            }
            isAshlangVerifyButtonEnabled = false
        } catch let error as MoproError {
            print("\nMoproError: \(error)")
        } catch {
            print("\nUnexpected error: \(error)")
        }
    }
}

