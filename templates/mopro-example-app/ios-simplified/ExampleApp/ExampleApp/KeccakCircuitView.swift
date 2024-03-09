//
//  KeccakCircuitView.swift
//  ExampleApp
//
//  Created by User Name on 3/8/24.
//

import SwiftUI
import moproFFI

struct KeccakCircuitView: View {
    @State private var textViewText = ""
    @State private var isProveButtonEnabled = true
    @State private var isVerifyButtonEnabled = false
    @State private var generatedProof: Data?
    @State private var publicInputs: Data?

    //let moproCircom = MoproCircom()

    var body: some View {
        NavigationView {
            VStack(spacing: 10) {
                Button("Init", action: runInitAction)
                Button("Prove", action: runProveAction).disabled(!isProveButtonEnabled)
                Button("Verify", action: runVerifyAction).disabled(!isVerifyButtonEnabled)
                ScrollView {
                    Text(textViewText)
                        .padding()
                }
                .frame(height: 200)
            }
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .principal) {
                    VStack {
                        Text("Keccak Example").font(.headline)
                        Text("Circom Circuit").font(.subheadline)
                    }
                }
            }
        }
    }
}

extension KeccakCircuitView {
    func runInitAction() {
        textViewText += "Initializing library... "
        Task {
            do {
                let start = CFAbsoluteTimeGetCurrent()
                try initializeMopro()
                let end = CFAbsoluteTimeGetCurrent()
                let timeTaken = end - start
                textViewText += "\(String(format: "%.3f", timeTaken))s\n"
                isProveButtonEnabled = true
            } catch {
                textViewText += "\nInitialization failed: \(error.localizedDescription)\n"
            }
        }
    }

    func runProveAction() {
         textViewText += "Generating proof... "
         Task {
             do {
                 // Prepare inputs
                 let inputVec: [UInt8] = [
                     116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0, 0, 0, 0, 0, 0,
                 ]
                 let bits = bytesToBits(bytes: inputVec)
                 var inputs = [String: [String]]()
                 inputs["in"] = bits

                 // Expected outputs
                 let outputVec: [UInt8] = [
                     37, 17, 98, 135, 161, 178, 88, 97, 125, 150, 143, 65, 228, 211, 170, 133, 153, 9, 88,
                     212, 4, 212, 175, 238, 249, 210, 214, 116, 170, 85, 45, 21,
                 ]
                 let _: [String] = bytesToBits(bytes: outputVec) // outputBits currently not used
                 // let expectedOutput: [UInt8] = serializeOutputs(outputBits)

                 let start = CFAbsoluteTimeGetCurrent()

                 // Generate Proof
                 let generateProofResult = try generateProof2(circuitInputs: inputs)
                 assert(!generateProofResult.proof.isEmpty, "Proof should not be empty")
                 //assert(Data(expectedOutput) == generateProofResult.inputs, "Circuit outputs mismatch the expected outputs")

                 let end = CFAbsoluteTimeGetCurrent()
                 let timeTaken = end - start

                 // Store the generated proof and public inputs for later verification
                 generatedProof = generateProofResult.proof
                 publicInputs = generateProofResult.inputs

                 textViewText += "\(String(format: "%.3f", timeTaken))s\n"

                 isVerifyButtonEnabled = true
             } catch {
                 textViewText += "\nProof generation failed: \(error.localizedDescription)\n"
             }
         }
     }

    func runVerifyAction() {
        guard let proof = generatedProof,
              let inputs = publicInputs else {
            textViewText += "Proof has not been generated yet.\n"
            return
        }

        textViewText += "Verifying proof... "
        Task {
             do {
                 let start = CFAbsoluteTimeGetCurrent()

                 let isValid = try verifyProof2(proof: proof, publicInput: inputs)
                 let end = CFAbsoluteTimeGetCurrent()
                 let timeTaken = end - start

                 if isValid {
                     textViewText += "\(String(format: "%.3f", timeTaken))s\n"

                 } else {
                     textViewText += "\nProof verification failed.\n"
                 }
                 isVerifyButtonEnabled = false // Optionally disable the verify button after verification
             } catch let error as MoproError {
                 print("\nMoproError: \(error)")
             } catch {
                 print("\nUnexpected error: \(error)")
             }
         }
    }
}

//#Preview {
//    CircuitView()
//}
