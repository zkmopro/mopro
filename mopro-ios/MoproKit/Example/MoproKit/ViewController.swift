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

        print("Running MoproKit.runExample()")
        do {
            try MoproKit.runExample()
            print("Finished running MoproKit.runExample()")
        } catch {
            print("Error running MoproKit.runExample(): \(error)")
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