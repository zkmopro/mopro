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
    // Imported from mopro-app
    do {
        // Call the prove function of the FibonacciCircuitHalo2Mopro circuit
        // Fix an error: Errors thrown from here are not handled because the enclosing catch is not exhaustive
        let fibonacciCircuit = FibonacciHalo2Circuit()
        print("Halo2: Initiated the circuit")
        
        let input = ["a": ["1", "0"], "b": ["2"], "out": ["55"]]

        let result = try fibonacciCircuit.prove(circuitInputs: input)
        print("Halo2: Generated the proof")

        let verifies = try fibonacciCircuit.verify(proof: result.proof, publicInput: result.inputs)

        print("Halo2: Verifies", verifies)

    } catch MoproErrorExternal.Halo2Error(let err) {

        print("Halo2: Failed with: ", err)
    } catch {
        print("Halo2: Should not be here: \(error)")
    }

}

func circom() {
    // Imported form mopro-app
    do {
        
        let zkeyPath = Bundle.main.path(forResource: "multiplier2_final", ofType: "zkey")!
        
        let circomCircuit = Multiplier2CircomCircuit(circuitPath: zkeyPath)
        print("Circom: Initiated the circuit")
        
        let input = ["a": ["1", "0"], "b": ["2"]]
        let result = try circomCircuit.prove(circuitInputs: input)
        print("Circom: Generated the proof")
        let verifies = try circomCircuit.verify(proof: result.proof, publicInput: result.inputs)
        
        print("Circom: Verifies", verifies)

        // Imported form mopro-ffi
        let ethereumProof = toEthereumProof(proof: result.proof)
        let ethereumInput = toEthereumInputs(inputs: result.inputs)
        
        print("Circom: Generated Ethereum proof and inputs")
    } catch MoproErrorExternal.CircomError(let err) {
        
        print("Circom: Failed with: ", err)
    } catch {
        print("Circom: Should not be here: \(error)")
    }
}
