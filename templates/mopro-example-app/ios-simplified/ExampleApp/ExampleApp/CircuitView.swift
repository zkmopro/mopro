//
//  CircuitView.swift
//  ExampleApp
//
//  Created by User Name on 3/8/24.
//

import SwiftUI
import moproFFI

struct CircuitView: View {
    @State private var textViewText = ""
    @State private var isProveButtonEnabled = true
    @State private var isVerifyButtonEnabled = false
    
    let moproCircom = MoproCircom()
    // var generatedProof: Data?
    // var publicInputs: Data?

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
                        Text("Circuit Example").font(.headline)
                        Text("Circom").font(.subheadline)
                    }
                }
            }
        }
    }
}

extension CircuitView {
    func runInitAction() {
        // Implement the initialization logic
        textViewText += "Initializing library\n"
        // Simulate initialization logic
        DispatchQueue.main.asyncAfter(deadline: .now() + 1) {
            self.textViewText += "Initialization completed\n"
            self.isProveButtonEnabled = true
        }
    }
    
    func runProveAction() {
        // Implement the proof generation logic
        textViewText += "Generating proof...\n"
        // Simulate proof generation
        DispatchQueue.main.asyncAfter(deadline: .now() + 1) {
            self.textViewText += "Proof generated\n"
            self.isVerifyButtonEnabled = true
        }
    }

    func runVerifyAction() {
        // Implement the verification logic
        textViewText += "Verifying proof...\n"
        // Simulate verification
        DispatchQueue.main.asyncAfter(deadline: .now() + 1) {
            self.textViewText += "Proof verified\n"
        }
    }
}

#Preview {
    CircuitView()
}
