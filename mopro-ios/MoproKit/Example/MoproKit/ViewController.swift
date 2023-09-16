//
//  ViewController.swift
//  MoproKit
//
//  Created by 1552237 on 09/16/2023.
//  Copyright (c) 2023 1552237. All rights reserved.
//

import UIKit

class ViewController: UIViewController {

    override func viewDidLoad() {
        super.viewDidLoad()

        // Create a UILabel instance
        let helloLabel = UILabel(frame: CGRect(x: 0, y: 0, width: 200, height: 21))

        // Set the label's properties
        helloLabel.center = view.center
        helloLabel.textAlignment = .center
        helloLabel.text = "Hello, World!"

        // Add the label to the main view
        view.addSubview(helloLabel)
    }

    override func didReceiveMemoryWarning() {
        super.didReceiveMemoryWarning()
        // Dispose of any resources that can be recreated.
    }

}

