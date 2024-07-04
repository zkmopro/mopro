//
//  ContentView.swift
//  TestMoproApp
//
//  Created by Artem Grigor on 03/07/2024.
//

import SwiftUI
import mopro_bindingsFFI
import moproFFI

struct ContentView: View {

    var body: some View {
        VStack {

            Image(systemName: "globe")
                .imageScale(.large)
                .foregroundStyle(.tint)
            Text("Hello, world!")
            Button("Test", action: hello).accessibilityIdentifier("prove")

        }
        .padding()
    }
}

#Preview {
    ContentView()
}

func hello() {
    // Imported from mopro-app
    MoproError1.CircomError("Hello")
    // Imported from mopro-ffi
    GenerateProofResult.init(proof: Data(), inputs: Data())

    let input = ["a": ["1", "0x"], "b": ["2"]]

    // Imported from mopro-app
    do {
        // Call the prove function of the FibonacciCircuitHalo2Mopro circuit
        // Fix an error: Errors thrown from here are not handled because the enclosing catch is not exhaustive
        let fibonacciCircuit = FibonacciCircuitHalo2Mopro()
        let result = try fibonacciCircuit.prove(in0: input)
        let verifies = try fibonacciCircuit.verify(in0: result.proof, in1: result.inputs)

        print("Verifies", verifies)

    } catch {

        print("Failed")
    }

}
