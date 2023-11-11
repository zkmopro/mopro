//
//  ViewController.swift
//  MoproKit
//
//  Created by 1552237 on 09/16/2023.
//  Copyright (c) 2023 1552237. All rights reserved.
//

import UIKit
import MoproKit

class RSAViewController: UIViewController {

    var proveButton = UIButton(type: .system)
    var verifyButton = UIButton(type: .system)

   func setupUI() {
        proveButton.setTitle("Prove", for: .normal)
        verifyButton.setTitle("Verify", for: .normal)

        proveButton.isEnabled = false
        verifyButton.isEnabled = false

        let stackView = UIStackView(arrangedSubviews: [proveButton, verifyButton])
        stackView.axis = .vertical
        stackView.spacing = 10
        stackView.translatesAutoresizingMaskIntoConstraints = false
        view.addSubview(stackView)

        NSLayoutConstraint.activate([
            stackView.centerXAnchor.constraint(equalTo: view.centerXAnchor),
            stackView.centerYAnchor.constraint(equalTo: view.centerYAnchor),
            stackView.leadingAnchor.constraint(equalTo: view.leadingAnchor, constant: 20),
            stackView.trailingAnchor.constraint(equalTo: view.trailingAnchor, constant: -20)
        ])
    }

    @objc func runProveAction() {
        // Logic for prove
    }

    @objc func runVerifyAction() {
        // Logic for verify
    }
}
