//
//  MultiplierCircuitView.swift
//  ExampleApp
//

import SwiftUI
import moproFFI

struct Halo2FibonacciCircuitView: View {
    @State private var textViewText = ""
    @State private var isVerifyButtonEnabled = false
    @State private var isProveButtonEnabled = false
    @State private var generatedProof: Data?
    @State private var publicInputs: Data?
    @State private var fibonacciInput: String = ""

    var body: some View {
        NavigationView {
            VStack(spacing: 10) {
                Text("Enter the 9th Fibonacci number:")
                    .font(.headline)
                    .padding(.top)

                TextField("9th Fibonacci number", text: $fibonacciInput)
                    .textFieldStyle(RoundedBorderTextFieldStyle())
                    .frame(maxWidth: UIScreen.main.bounds.width * 0.5)
                    .multilineTextAlignment(.center)
                    .padding()
                    .keyboardType(.numberPad)
                    .onChange(of: fibonacciInput, perform: {newValue in
                        isProveButtonEnabled = isValidFibonacciInput(newValue)
                    })

                Button("Prove", action: runProveAction)
                    .disabled(!isProveButtonEnabled)

                Button("Verify", action: runVerifyAction)
                    .disabled(!isVerifyButtonEnabled)

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
                        Text("Fibonacci Example").font(.headline)
                        Text("Halo2 Circuit").font(.subheadline)
                    }
                }
            }
        }
    }

    func isValidFibonacciInput(_ input: String) -> Bool {
        guard let number = Int(input), number > 0 else {
            return false
        }
        // Here you can add additional validation for Fibonacci numbers if needed.
        return true
    }
}

extension Halo2FibonacciCircuitView {
    func runProveAction() {
        guard let a = Int(fibonacciInput) else {
            textViewText += "\nInvalid Fibonacci number entered.\n"
            return
        }

        textViewText += "Generating Halo2 proof for \(a) "
        Task {
            do {
                // Prepare inputs

                var inputs = [String: [String]]()
                let out = 55

                inputs["out"] = [String(out)]

                // Expected outputs
                let outputs: [String] = [String(1), String(1), String(out)]
                let expectedOutput: [UInt8] = serializeOutputs(outputs)

                let start = CFAbsoluteTimeGetCurrent()

                // Generate Proof
                let generateProofResult = try generateHalo2Proof(circuitInputs: inputs)
                assert(!generateProofResult.proof.isEmpty, "Proof should not be empty")
                assert(Data(expectedOutput) == generateProofResult.inputs, "Circuit outputs mismatch the expected outputs")

                let end = CFAbsoluteTimeGetCurrent()
                let timeTaken = end - start

                // Store the generated proof and public inputs for later verification
                generatedProof = generateProofResult.proof
                publicInputs = generateProofResult.inputs

                textViewText += "(\(String(format: "%.3f", timeTaken))s)\n"

                isVerifyButtonEnabled = true
            } catch {
                textViewText += "\nHalo2 Proof generation failed: \(error.localizedDescription)\n"
            }
        }
    }

    func runVerifyAction() {
        guard let proof = generatedProof,
              let inputs = publicInputs else {
            textViewText += "Proof has not been generated yet.\n"
            return
        }

        textViewText += "Verifying Halo2 proof for \(fibonacciInput) "
        Task {
            do {
                let start = CFAbsoluteTimeGetCurrent()

                let isValid = try verifyHalo2Proof(proof: proof, publicInput: inputs)
                let end = CFAbsoluteTimeGetCurrent()
                let timeTaken = end - start

                if isValid {
                    textViewText += "(\(String(format: "%.3f", timeTaken))s)\n"
                } else {
                    textViewText += "\nHalo2 Proof verification failed.\n"
                }
                isVerifyButtonEnabled = false
            } catch let error as MoproError {
                print("\nMoproError: \(error)")
            } catch {
                print("\nUnexpected error: \(error)")
            }
        }
    }
}

// #Preview {
//     Halo2CircuitView()
// }
