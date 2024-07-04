//
//  ContentView.swift
//  TestMoproApp
//
//  Created by Artem Grigor on 03/07/2024.
//

import SwiftUI

struct ContentView: View {

    var body: some View {
        VStack {

            Image(systemName: "globe")
                .imageScale(.large)
                .foregroundStyle(.tint)
            Text("Hello to our proving systems!")
            
            Button("Test Circom", action: circom).accessibilityIdentifier("provecircom")
            Button("Test Halo2", action: halo2).accessibilityIdentifier("provehalo2")

        }
        .padding()
    }
}

#Preview {
    ContentView()
}

func halo2() {

    let input = ["a": ["1", "0x"], "b": ["2"]]

    // Imported from mopro-app
    do {
        // Call the prove function of the FibonacciCircuitHalo2Mopro circuit
        // Fix an error: Errors thrown from here are not handled because the enclosing catch is not exhaustive
        let fibonacciCircuit = FibonacciCircuitHalo2Mopro()
        let result = try fibonacciCircuit.prove(in1: input)
        let verifies = try fibonacciCircuit.verify(in1: result.proof, in2: result.inputs)

        print("Verifies", verifies)

    } catch MoproErrorExternal.Halo2Error(let err) {

        print("Failed with: ", err)
    } catch {
        print("Should not be here...")
    }

}

func circom() {
     
    let input = ["a": ["1", "0x"], "b": ["2"]]

    do {
        
        let circomCircuit = Multiplier3CircomMopro(circuitPath: "multiplier3_final.zkey")
        // WARNING - this will always fail because for now the prove implementation is broken
        let result = try circomCircuit.prove(in1: input)
        let verifies = try circomCircuit.verify(in1: result.proof, in2: result.inputs)
        
        print("Verifies", verifies)
    } catch MoproErrorExternal.CircomError(let err) {
        
        print("Failed with: ", err)
    } catch {
        print("Should not be here...")
    }
}
