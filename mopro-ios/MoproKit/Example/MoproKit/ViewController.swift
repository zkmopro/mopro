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


    var proveButton = UIButton()
    var verifyButton = UIButton()
    var textView = UITextView()
    let moproCircom = MoproKit.MoproCircom()
    var setupResult: SetupResult?

    override func viewDidLoad() {
        super.viewDidLoad()
        setupUI()
        runSetup()

        view.addSubview(proveButton)
        view.addSubview(verifyButton)
        view.addSubview(textView)
    }

    func runSetup() {
        if let wasmPath = Bundle.main.path(forResource: "multiplier2", ofType: "wasm"),
           let r1csPath = Bundle.main.path(forResource: "multiplier2", ofType: "r1cs") {
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
        // Setup buttons and text view
        proveButton.setTitle("Prove", for: .normal)
        verifyButton.setTitle("Verify", for: .normal)
        textView.isEditable = false

        // Set button colors for visibility
        proveButton.backgroundColor = .blue
        verifyButton.backgroundColor = .blue

        // Set button action targets
        proveButton.addTarget(self, action: #selector(runProveAction), for: .touchUpInside)
        verifyButton.addTarget(self, action: #selector(runVerifyAction), for: .touchUpInside)

        // Add subviews
        view.addSubview(proveButton)
        view.addSubview(verifyButton)
        view.addSubview(textView)

        let buttonWidth: CGFloat = 200
        let buttonHeight: CGFloat = 50
        let padding: CGFloat = 20
        let verticalOffset: CGFloat = 100  // Adjust this value to position buttons higher or lower

        proveButton.frame = CGRect(
            x: (view.frame.width - buttonWidth) / 2,
            y: verticalOffset,
            width: buttonWidth,
            height: buttonHeight
        )

        verifyButton.frame = CGRect(
            x: (view.frame.width - buttonWidth) / 2,
            y: verticalOffset + buttonHeight + padding,
            width: buttonWidth,
            height: buttonHeight
        )

        textView.frame = CGRect(
            x: padding,
            y: verticalOffset + 2 * (buttonHeight + padding),
            width: view.frame.width - 2 * padding,
            height: view.frame.height / 2 - padding * 2
        )
    }

    @objc func runProveAction() {
        guard let setupResult = setupResult else {
            print("Setup is not completed yet.")
            return
        }
        do {
            let start = CFAbsoluteTimeGetCurrent()
            let generateProofResult = try moproCircom.generateProof()
            let end = CFAbsoluteTimeGetCurrent()
            let timeTaken = end - start
            textView.text += "Proof generation took \(timeTaken) seconds.\n"
            assert(!generateProofResult.proof.isEmpty, "Proof should not be empty")
            verifyButton.isEnabled = true // Enable the Verify button once proof has been generated
        } catch let error as MoproError {
            print("MoproError: \(error)")
        } catch {
            print("Unexpected error: \(error)")
        }
       }

    @objc func runVerifyAction() {
        guard let setupResult = setupResult else {
            print("Setup is not completed yet.")
            return
        }
        do {
             // Get the proof again, ideally this should be stored and reused
            let generateProofResult = try moproCircom.generateProof()
            let isValid = try moproCircom.verifyProof(proof: generateProofResult.proof, publicInput: setupResult.inputs)
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
