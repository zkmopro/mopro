//
//  ViewController.swift
//  MoproKit
//
//  Created by 1552237 on 09/16/2023.
//  Copyright (c) 2023 1552237. All rights reserved.
//

import UIKit
import MoproKit


// Main ViewController
class ViewController: UIViewController {

    let keccakSetupButton = UIButton(type: .system)
    let keccakZkeyButton = UIButton(type: .system)
    let rsaButton = UIButton(type: .system)
    let kimchiButton = UIButton(type: .system)

    override func viewDidLoad() {
        super.viewDidLoad()
        // TODO: Improve style

        // Set title
        let title = UILabel()
        title.text = "Mopro Examples"
        title.textColor = .white
        title.textAlignment = .center
        navigationItem.titleView = title
        navigationController?.navigationBar.isHidden = false
        navigationController?.navigationBar.prefersLargeTitles = true

        setupMainUI()
    }

    func setupMainUI() {
        keccakSetupButton.setTitle("Keccak (Setup)", for: .normal)
        keccakSetupButton.addTarget(self, action: #selector(openKeccakSetup), for: .touchUpInside)

        keccakZkeyButton.setTitle("Keccak (Zkey)", for: .normal)
        keccakZkeyButton.addTarget(self, action: #selector(openKeccakZkey), for: .touchUpInside)

        rsaButton.setTitle("RSA", for: .normal)
        rsaButton.addTarget(self, action: #selector(openRSA), for: .touchUpInside)

        kimchiButton.setTitle("Kimchi", for: .normal)
        kimchiButton.addTarget(self, action: #selector(openKimchi), for: .touchUpInside)

        keccakSetupButton.contentEdgeInsets = UIEdgeInsets(top: 12, left: 16, bottom: 12, right: 16)
        keccakZkeyButton.contentEdgeInsets = UIEdgeInsets(top: 12, left: 16, bottom: 12, right: 16)
        rsaButton.contentEdgeInsets = UIEdgeInsets(top: 12, left: 16, bottom: 12, right: 16)
        kimchiButton.contentEdgeInsets = UIEdgeInsets(top: 12, left: 16, bottom: 12, right: 16)

//        self.title = "Mopro Examples"
//        navigationController?.navigationBar.prefersLargeTitles = true

        let stackView = UIStackView(arrangedSubviews: [keccakSetupButton, keccakZkeyButton, rsaButton, kimchiButton])
        stackView.axis = .vertical
        stackView.spacing = 20
        stackView.translatesAutoresizingMaskIntoConstraints = false
        view.addSubview(stackView)

        NSLayoutConstraint.activate([
            stackView.centerXAnchor.constraint(equalTo: view.centerXAnchor),
            stackView.centerYAnchor.constraint(equalTo: view.centerYAnchor)
        ])
    }

    @objc func openKeccakSetup() {
        let keccakSetupVC = KeccakSetupViewController()
        navigationController?.pushViewController(keccakSetupVC, animated: true)
    }

    @objc func openKeccakZkey() {
        let keccakZkeyVC = KeccakZkeyViewController()
        navigationController?.pushViewController(keccakZkeyVC, animated: true)
    }

    @objc func openRSA() {
        let rsaVC = RSAViewController()
        navigationController?.pushViewController(rsaVC, animated: true)
    }

    @objc func openKimchi() {
        let kimchiVC = KimchiViewController()
        navigationController?.pushViewController(kimchiVC, animated: true)
    }
}