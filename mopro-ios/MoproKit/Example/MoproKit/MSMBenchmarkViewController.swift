//
//  MSMBenchmarkViewController.swift
//  MoproKit_Example
//
//  Created by Fuchuan Chung on 2024/2/6.
//  Copyright © 2024 CocoaPods. All rights reserved.
//

import UIKit
import MoproKit

class MSMBenchmarkViewController: UIViewController, UITableViewDelegate, UITableViewDataSource {
    
    var tableView: UITableView!
    var submitButton: UIButton!
    
    let algorithms = ["1. Arkwork MSM", "2. ", "3. "]
    var selectedAlgorithms: Set<Int> = []
    
    // for benchmark results
    var resultsView: UIView!
    var resultsTableView: UITableView!
    var benchmarkResults: [BenchmarkResult] = []
    
    typealias BenchmarkClosure = () throws -> BenchmarkResult
    
    let msmBenchmarkMapping: [String: (UInt32?) throws -> BenchmarkResult] = [
        // update the mapping with function in the future
        "1. Arkwork MSM": runMsmBenchmark,
//        "2. ": msmFunction2,
//        "3. ": msmFunction3,
    ]

    override func viewDidLoad() {
        super.viewDidLoad()
        setupTableView()
        setupBenchmarkButton()
    }
    
    func setupTableView() {
        tableView = UITableView(frame: view.bounds)
        tableView.delegate = self
        tableView.dataSource = self
        tableView.register(UITableViewCell.self, forCellReuseIdentifier: "cell")
        tableView.allowsMultipleSelection = true // Enable multiple selection
        view.addSubview(tableView)
    }
    
    func setupBenchmarkButton() {
        submitButton = UIButton(type: .system)
        submitButton.frame = CGRect(x: 20, y: view.bounds.height - 80, width: view.bounds.width - 60, height: 50)
        submitButton.setTitle("Generating Benchmarks", for: .normal)
        submitButton.addTarget(self, action: #selector(submitAction), for: .touchUpInside)
        submitButton.backgroundColor = .systemBlue
        submitButton.setTitleColor(.white, for: .normal)
        submitButton.layer.cornerRadius = 10
        view.addSubview(submitButton)
    }
    
    // UITableViewDataSource methods
    func tableView(_ tableView: UITableView, numberOfRowsInSection section: Int) -> Int {
        return algorithms.count
    }
    
    func tableView(_ tableView: UITableView, cellForRowAt indexPath: IndexPath) -> UITableViewCell {
        
        let cell = tableView.dequeueReusableCell(withIdentifier: "cell", for: indexPath)
        cell.textLabel?.text = algorithms[indexPath.row]
        
        if selectedAlgorithms.contains(indexPath.row) {
            tableView.selectRow(at: indexPath, animated: false, scrollPosition: .none)
            cell.accessoryType = .checkmark
        } else {
            cell.accessoryType = .none
        }
        return cell
    }
    
    // UITableViewDelegate methods
    func tableView(_ tableView: UITableView, didSelectRowAt indexPath: IndexPath) {
        selectedAlgorithms.insert(indexPath.row)
        if let cell = tableView.cellForRow(at: indexPath) {
            cell.accessoryType = .checkmark
        }
        print("Selected: \(algorithms[indexPath.row])")
    }
    
    func tableView(_ tableView: UITableView, didDeselectRowAt indexPath: IndexPath) {
        selectedAlgorithms.remove(indexPath.row)
        if let cell = tableView.cellForRow(at: indexPath) {
            cell.accessoryType = .none
        }
        print("Deselected: \(algorithms[indexPath.row])")
    }
    
    @objc func submitAction() {
        print("Selected algorithms: \(selectedAlgorithms.map { algorithms[$0] })")

        for index in selectedAlgorithms.sorted() {
            let algorithm = algorithms[index]
            do {
                print("Running MSM in algorithm: \(algorithm)...")
                if let benchmarkFunction = msmBenchmarkMapping[algorithm] {
                    let benchData: BenchmarkResult = try benchmarkFunction(10000)
                    print("Result of \(algorithm): \n \(benchData)")
                }
            } catch {
                    print("Error running benchmark: \(error)")
            }
        }
    }
}
