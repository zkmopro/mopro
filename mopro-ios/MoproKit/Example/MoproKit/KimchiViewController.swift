//
//  ViewController.swift
//  MoproKit
//
//  Created by 1552237 on 09/16/2023.
//  Copyright (c) 2023 1552237. All rights reserved.
//

import UIKit
import MoproKit

class KimchiViewController: UIViewController {

    var benchButton = UIButton(type: .system)
    //var initButton = UIButton(type: .system)
    var proveButton = UIButton(type: .system)
    var verifyButton = UIButton(type: .system)
    var textView = UITextView()

    // TODO: Put this in init
    let moproKimchi = MoproKit.MoproKimchi()

    var generatedProof: Data?
    // var publicInputs: Data?

    override func viewDidLoad() {
        super.viewDidLoad()

        // Set title
        let title = UILabel()
        title.text = "Kimchi"
        title.textColor = .white
        title.textAlignment = .center
        navigationItem.titleView = title
        navigationController?.navigationBar.isHidden = false
        navigationController?.navigationBar.prefersLargeTitles = true

        setupUI()
    }

   func setupUI() {
        benchButton.setTitle("Bench", for: .normal)
        // initButton.setTitle("Init", for: .normal)
        proveButton.setTitle("Prove", for: .normal)
        verifyButton.setTitle("Verify", for: .normal)

        proveButton.isEnabled = true
        verifyButton.isEnabled = false
        textView.isEditable = false

        // Setup actions for buttons
        benchButton.addTarget(self, action: #selector(runBenchAction), for: .touchUpInside)
        // initButton.addTarget(self, action: #selector(runInitAction), for: .touchUpInside)
        proveButton.addTarget(self, action: #selector(runProveAction), for: .touchUpInside)
        verifyButton.addTarget(self, action: #selector(runVerifyAction), for: .touchUpInside)

        benchButton.contentEdgeInsets = UIEdgeInsets(top: 12, left: 16, bottom: 12, right: 16)
        // initButton.contentEdgeInsets = UIEdgeInsets(top: 12, left: 16, bottom: 12, right: 16)
        proveButton.contentEdgeInsets = UIEdgeInsets(top: 12, left: 16, bottom: 12, right: 16)
        verifyButton.contentEdgeInsets = UIEdgeInsets(top: 12, left: 16, bottom: 12, right: 16)

        //let stackView = UIStackView(arrangedSubviews: [initButton, proveButton, verifyButton, textView])
        let stackView = UIStackView(arrangedSubviews: [benchButton, proveButton, verifyButton, textView])
        stackView.axis = .vertical
        stackView.spacing = 10
        stackView.translatesAutoresizingMaskIntoConstraints = false
        view.addSubview(stackView)

        // Make text view visible
        textView.heightAnchor.constraint(equalToConstant: 200).isActive = true

        NSLayoutConstraint.activate([
            stackView.centerXAnchor.constraint(equalTo: view.centerXAnchor),
            stackView.centerYAnchor.constraint(equalTo: view.centerYAnchor),
            stackView.leadingAnchor.constraint(equalTo: view.leadingAnchor, constant: 20),
            stackView.trailingAnchor.constraint(equalTo: view.trailingAnchor, constant: -20)
        ])
    }

    @objc func runBenchAction() {
        // Update the textView on the main thread
        DispatchQueue.main.async {
            self.textView.text += "Running bench library\n"
        }

        // Execute long-running tasks in the background
        DispatchQueue.global(qos: .userInitiated).async {
            // Record start time
            let start = CFAbsoluteTimeGetCurrent()

            do {
                //try initializeMopro()
                try kimchiBench()

                // Record end time and compute duration
                let end = CFAbsoluteTimeGetCurrent()
                let timeTaken = end - start

                // Again, update the UI on the main thread
                DispatchQueue.main.async {
                    self.textView.text += "Running bench took \(timeTaken) seconds.\n"
                }
            } catch {
                // Handle errors - update UI on main thread
                DispatchQueue.main.async {
                    self.textView.text += "An error occurred during initialization: \(error)\n"
                }
            }
        }
    }

    @objc func runInitAction() {
        // Logic for init
    }

    @objc func runProveAction() {
        // Logic for prove
        // Create a proof
        do {
            let start = CFAbsoluteTimeGetCurrent()
            generatedProof = try moproKimchi.createProof()
            let end = CFAbsoluteTimeGetCurrent()
            let timeTaken = end - start
            print("Proof creation took \(timeTaken) seconds.")
            textView.text += "Proof creation took \(timeTaken) seconds.\n"
            verifyButton.isEnabled = true
        } catch let error as MoproError {
            print("MoproError: \(error)")
            textView.text += "MoproError: \(error)\n"
        } catch {
            print("Unexpected error: \(error)")
            textView.text += "Unexpected error: \(error)\n"
        }
    }

    @objc func runVerifyAction() {
         // Verify the proof

         guard let proof = generatedProof else {
            print("Proof has not been generated yet.")
            return
        }
         do {
            let start = CFAbsoluteTimeGetCurrent()
            let isProofValid = try moproKimchi.verifyProof(proof: proof)
            let end = CFAbsoluteTimeGetCurrent()
            let timeTaken = end - start
            textView.text += "Proof verification took: \(timeTaken)\n"
        } catch let error as MoproError {
            print("MoproError: \(error)")
            textView.text += "MoproError: \(error)\n"
        } catch {
            print("Unexpected error: \(error)")
            textView.text += "Unexpected error: \(error)\n"
        }
    }
}
