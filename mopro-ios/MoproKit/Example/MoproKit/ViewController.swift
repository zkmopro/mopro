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

    var proveButton = UIButton(type: .system)
    var verifyButton = UIButton(type: .system)
    var textView = UITextView()
    let moproCircom = MoproKit.MoproCircom()
    var setupResult: SetupResult?
    var generatedProof: Data?
    var publicInputs: Data?


    override func viewDidLoad() {
        super.viewDidLoad()
        setupUI()
        runSetup()
        // Uncomment this line to use zkey logic
        //runGenerateProof2()
    }

// TODO: Run init separately?
    func runGenerateProof2() {
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
            let outputBits: [String] = bytesToBits(bytes: outputVec)
//            let expectedOutput: [UInt8] = serializeOutputs(outputBits)

            // Record start time
            let start = CFAbsoluteTimeGetCurrent()

            // Generate Proof
            let generateProofResult = try generateProof2(circuitInputs: inputs)
            assert(!generateProofResult.proof.isEmpty, "Proof should not be empty")
            //assert(Data(expectedOutput) == generateProofResult.inputs, "Circuit outputs mismatch the expected outputs")

            // Record end time and compute duration
            let end = CFAbsoluteTimeGetCurrent()
            let timeTaken = end - start

            // Store the generated proof and public inputs for later verification
            generatedProof = generateProofResult.proof
            publicInputs = generateProofResult.inputs

            textView.text += "Proof generation took \(timeTaken) seconds.\n"
            verifyButton.isEnabled = true // Enable the Verify button once proof has been generated
        } catch let error as MoproError {
            print("MoproError: \(error)")
        } catch {
            print("Unexpected error: \(error)")
        }
    }

    func runSetup() {
        if let wasmPath = Bundle.main.path(forResource: "keccak256_256_test", ofType: "wasm"),
           let r1csPath = Bundle.main.path(forResource: "keccak256_256_test", ofType: "r1cs") {
        // Multiplier example
        // if let wasmPath = Bundle.main.path(forResource: "multiplier2", ofType: "wasm"),
        //    let r1csPath = Bundle.main.path(forResource: "multiplier2", ofType: "r1cs") {
            do {
                setupResult = try moproCircom.setup(wasmPath: wasmPath, r1csPath: r1csPath)
                proveButton.isEnabled = true // Enable the Prove button upon successful setup
            } catch let error as MoproError {
                print("MoproError: \(error)")
            } catch {
                print("Unexpected error: \(error)")
            }
        } else {
            print("Error getting paths for resources")
        }
    }

    func setupUI() {
        self.title = "MoproKit Demo"
        view.backgroundColor = .white

        proveButton.setTitle("Prove", for: .normal)
        verifyButton.setTitle("Verify", for: .normal)
        textView.isEditable = false

        // Make buttons bigger
        proveButton.contentEdgeInsets = UIEdgeInsets(top: 12, left: 16, bottom: 12, right: 16)
        verifyButton.contentEdgeInsets = UIEdgeInsets(top: 12, left: 16, bottom: 12, right: 16)

        proveButton.addTarget(self, action: #selector(runProveAction), for: .touchUpInside)
        verifyButton.addTarget(self, action: #selector(runVerifyAction), for: .touchUpInside)

        let stackView = UIStackView(arrangedSubviews: [proveButton, verifyButton, textView])
        stackView.axis = .vertical
        stackView.spacing = 10
        stackView.translatesAutoresizingMaskIntoConstraints = false

        // Make text view visibile
        textView.heightAnchor.constraint(equalToConstant: 200).isActive = true

        view.addSubview(stackView)

        NSLayoutConstraint.activate([
            stackView.leadingAnchor.constraint(equalTo: view.leadingAnchor, constant: 20),
            stackView.trailingAnchor.constraint(equalTo: view.trailingAnchor, constant: -20),
            stackView.topAnchor.constraint(equalTo: view.safeAreaLayoutGuide.topAnchor, constant: 20)
        ])
    }

    @objc func runProveAction() {
        guard let setupResult = setupResult else {
            print("Setup is not completed yet.")
            return
        }
        do {

            // Prepare inputs
            let inputVec: [UInt8] = [
                116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0,
            ]
            let bits = bytesToBits(bytes: inputVec)
            var inputs = [String: [String]]()
            inputs["in"] = bits

            // Multiplier example
            // var inputs = [String: [String]]()
            // let a = 3
            // let b = 5
            // inputs["a"] = [String(a)]
            // inputs["b"] = [String(b)]

            // Record start time
            let start = CFAbsoluteTimeGetCurrent()

            // Generate Proof
            let generateProofResult = try moproCircom.generateProof(circuitInputs: inputs)
            assert(!generateProofResult.proof.isEmpty, "Proof should not be empty")

            // Record end time and compute duration
            let end = CFAbsoluteTimeGetCurrent()
            let timeTaken = end - start

            // Store the generated proof and public inputs for later verification
            generatedProof = generateProofResult.proof
            publicInputs = generateProofResult.inputs

            textView.text += "Proof generation took \(timeTaken) seconds.\n"
            verifyButton.isEnabled = true // Enable the Verify button once proof has been generated
        } catch let error as MoproError {
            print("MoproError: \(error)")
        } catch {
            print("Unexpected error: \(error)")
        }
    }

    @objc func runVerifyAction() {
        guard let setupResult = setupResult,
              let proof = generatedProof,
              let publicInputs = publicInputs else {
            print("Setup is not completed or proof has not been generated yet.")
            return
        }
        do {
            // Verify Proof
            let isValid = try moproCircom.verifyProof(proof: proof, publicInput: publicInputs)
            assert(isValid, "Proof verification should succeed")

            textView.text += "Proof verification succeeded.\n"
        } catch let error as MoproError {
            print("MoproError: \(error)")
        } catch {
            print("Unexpected error: \(error)")
        }
    }

    override func didReceiveMemoryWarning() {
        super.didReceiveMemoryWarning()
        // Dispose of any resources that can be recreated.
    }

}

func bytesToBits(bytes: [UInt8]) -> [String] {
    var bits = [String]()
    for byte in bytes {
        for j in 0..<8 {
            let bit = (byte >> j) & 1
            bits.append(String(bit))
        }
    }
    return bits
}
