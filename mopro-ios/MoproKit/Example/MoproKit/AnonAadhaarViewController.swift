//
//  AnonAadhaarViewController.swift
//  MoproKit_Example
//
//  Created by Yanis Meziane on 11/11/2023.
//  Copyright © 2023 CocoaPods. All rights reserved.
//

import UIKit
import WebKit
import MoproKit

class AnonAadhaarViewController: UIViewController, WKScriptMessageHandler, WKNavigationDelegate {
    let webView = WKWebView()
    let moproCircom = MoproKit.MoproCircom()
    //var setupResult: SetupResult?
    var generatedProof: Data?
    var publicInputs: Data?
    
    override func viewDidLoad() {
        super.viewDidLoad()
            
            let contentController = WKUserContentController()
            contentController.add(self, name: "startProvingHandler")
            contentController.add(self, name: "messageHandler")
            
            let configuration = WKWebViewConfiguration()
            configuration.userContentController = contentController
            configuration.preferences.javaScriptEnabled = true
            
            // Assign the configuration to the WKWebView
            let webView = WKWebView(frame: view.bounds, configuration: configuration)
            webView.navigationDelegate = self
            
            view.addSubview(webView)
            
            guard let url = URL(string: "https://webview-anon-adhaar.vercel.app/") else { return }
            webView.load(URLRequest(url: url))
    }
    
    
    
    @objc func runInitAction() {
        // Logic for init
        do {
            //textView.text += "Initializing library\n"
            // Record start time
            let start = CFAbsoluteTimeGetCurrent()

            try initializeMopro()

            // Record end time and compute duration
            let end = CFAbsoluteTimeGetCurrent()
            let timeTaken = end - start

            //textView.text += "Initializing arkzkey took \(timeTaken) seconds.\n"
        } catch let error as MoproError {
            print("MoproError: \(error)")
        } catch {
            print("Unexpected error: \(error)")
        }
    }
    
    @objc func runProveAction(inputs: [String: [String]]) {
        // Logic for prove (generate_proof2)
        do {
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
            
            print("Proof generation took \(timeTaken) seconds.\n")
            
            //textView.text += "Proof generation took \(timeTaken) seconds.\n"
            // TODO: Enable verify
            //verifyButton.isEnabled = false
            //verifyButton.isEnabled = true // Enable the Verify button once proof has been generated
        } catch let error as MoproError {
            print("MoproError: \(error)")
        } catch {
            print("Unexpected error: \(error)")
        }
    }
    
    override func viewDidLayoutSubviews() {
        super.viewDidLayoutSubviews()
        webView.frame = view.bounds
    }
    
    struct Witness {
        let signature: [String]
        let modulus: [String]
        let base_message: [String]
    }
    
    // Implement WKScriptMessageHandler method
        func provingContentController(_ userContentController: WKUserContentController, didReceive message: WKScriptMessage) {
            if message.name == "messageHandler" {
                // Handle messages for "messageHandler"
                print("Received message from JavaScript:", message.body)
            }
        }

    func userContentController(_ userContentController: WKUserContentController, didReceive message: WKScriptMessage) {
        if message.name == "startProvingHandler", let data = message.body as? [String: Any] {
            // Check for the "witness" key in the received data
            if let witnessData = data["witness"] as? [String: [String]] {
                if let signature = witnessData["signature"],
                   let modulus = witnessData["modulus"],
                   let baseMessage = witnessData["base_message"] {
                    
                    let inputs: [String: [String]] = [
                        "signature": signature,
                        "modulus": modulus,
                        "base_message": baseMessage
                    ]
                    
                    // Call your Swift function with the received witness data
                    runProveAction(inputs: inputs)
                }
            } else if let error = data["error"] as? String {
                // Handle error data
                print("Received error value from JavaScript:", error)
            } else {
                print("No valid data keys found in the message data.")
            }
        }
    }
}
