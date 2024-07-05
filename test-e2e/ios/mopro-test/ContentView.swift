//
//  ContentView.swift
//  mopro-test
//
//  Created by Chance on 6/25/24.
//
import SwiftUI

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
    @State private var isProveButtonEnabled = true
    @State private var isVerifyButtonEnabled = false
    @State private var generatedProof: Data?
    @State private var publicInputs: Data?
    private let zkeyPath: String
    private let circuit: Multiplier2CircomCircuit

    init() {
        self.zkeyPath = Bundle.main.path(forResource: "multiplier2_final", ofType: "zkey")!
        self.circuit = Multiplier2CircomCircuit(circuitPath: zkeyPath)
    }
    
    var body: some View {
        VStack(spacing: 10) {
            Image(systemName: "globe")
                .imageScale(.large)
                .foregroundStyle(.tint)
            Button("Prove", action: runProveAction).disabled(!isProveButtonEnabled).accessibilityIdentifier("prove")
            Button("Verify", action: runVerifyAction).disabled(!isVerifyButtonEnabled).accessibilityIdentifier("verify")
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
    func runProveAction() {
        textViewText += "Generating proof... "
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
            let generateProofResult = try circuit.prove(circuitInputs: inputs)
            assert(!generateProofResult.proof.isEmpty, "Proof should not be empty")
            assert(Data(expectedOutput) == generateProofResult.inputs, "Circuit outputs mismatch the expected outputs")
            
            let end = CFAbsoluteTimeGetCurrent()
            let timeTaken = end - start
            
            // Store the generated proof and public inputs for later verification
            generatedProof = generateProofResult.proof
            publicInputs = generateProofResult.inputs
            
            textViewText += "\(String(format: "%.3f", timeTaken))s 1️⃣\n"
            
            isVerifyButtonEnabled = true
        } catch {
            textViewText += "\nProof generation failed: \(error.localizedDescription)\n"
        }
    }
    
    func runVerifyAction() {
        guard let proof = generatedProof,
              let inputs = publicInputs else {
            textViewText += "Proof has not been generated yet.\n"
            return
        }
        
        textViewText += "Verifying proof... "
        do {
            let start = CFAbsoluteTimeGetCurrent()
            
            let isValid = try circuit.verify(proof: proof, publicInput: inputs)
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
            isVerifyButtonEnabled = false
        } catch let error as MoproErrorExternal {
            print("\nMoproError: \(error)")
        } catch {
            print("\nUnexpected error: \(error)")
        }
    }
}

