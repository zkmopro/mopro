//
//  ViewController.swift
//  MoproKit
//
//  Created by 1552237 on 09/16/2023.
//  Copyright (c) 2023 1552237. All rights reserved.
//

import UIKit
import MoproKit

class ViewController: UIViewController {

    override func viewDidLoad() {
        super.viewDidLoad()

        let helloLabel = UILabel(frame: CGRect(x: 0, y: 0, width: 200, height: 21))

        // Trying some MoproKit functions

        let result = MoproKit.add(a: 10, b: 20)
        print("Result of add: \(result)")

        let greeting = MoproKit.hello()
        NSLog(greeting)

        let moproCircom = MoproKit.MoproCircom()

        print("Loading circuit assets")
        if let wasmPath = Bundle.main.path(forResource: "multiplier2", ofType: "wasm"),
           let r1csPath = Bundle.main.path(forResource: "multiplier2", ofType: "r1cs") {

            do {
                // Setup
                NSLog("Setup")
                let setupResult = try moproCircom.setup(wasmPath: wasmPath, r1csPath: r1csPath)
                assert(!setupResult.provingKey.isEmpty, "Proving key should not be empty")
                assert(!setupResult.inputs.isEmpty, "Inputs should not be empty")

                // Generate Proof
                NSLog("Generate proof")
                let generateProofResult = try moproCircom.generateProof()
                assert(!generateProofResult.proof.isEmpty, "Proof should not be empty")

                print(generateProofResult.proof)

                // Verify Proof
                NSLog("Verify proof")
                let isValid = try moproCircom.verifyProof(proof: generateProofResult.proof, publicInput: setupResult.inputs)
                assert(isValid, "Proof verification should succeed")

            } catch let error as MoproError {
                print("MoproError: \(error)")
            } catch {
                print("Unexpected error: \(error)")
            }

        } else {
            print("Error getting paths for resources")
        }

        // Set the label's properties
        helloLabel.center = view.center
        helloLabel.textAlignment = .center
        helloLabel.text = greeting

        // Add the label to the main view
        view.addSubview(helloLabel)
    }

    override func didReceiveMemoryWarning() {
        super.didReceiveMemoryWarning()
        // Dispose of any resources that can be recreated.
    }

}
