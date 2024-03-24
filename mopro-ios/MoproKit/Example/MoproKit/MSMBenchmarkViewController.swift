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

    struct AlgorithmBenchmark {
        var algorithm: String
        var avgMsmTime: Double
        var diffWithBaseline: Double    // Baseline is Arkwork Vanilla MSM
    }

    var tableView: UITableView!
    var resultsTableView: UITableView!
    var submitButton: UIButton!

    let algorithms = ["Arkwork (Baseline)", "Example Algo 1", "Example Algo 2"]
    var selectedAlgorithms: Set<Int> = [0]  // Default to select the baseline MSM algorithm

    var benchmarkResults: [AlgorithmBenchmark] = []
    
    typealias BenchmarkClosure = () throws -> BenchmarkResult
    
    // update the mapping with function in the future
    let msmBenchmarkMapping: [String: (UInt32?) throws -> BenchmarkResult] = [
        "Arkwork (Baseline)": arkworksPippenger,
        // "Example Algo 1": ,
        // "Example Algo 2": ,
    ]

    override func viewDidLoad() {
        super.viewDidLoad()

        // Set title
        let title = UILabel()
        title.text = "MSM Benchmark"
        title.textColor = .white
        title.textAlignment = .center
        navigationItem.titleView = title
        navigationController?.navigationBar.isHidden = false
        navigationController?.navigationBar.prefersLargeTitles = true

        setupTableView()
        setupBenchmarkButton()
        setupResultsTableView()
    }
    
    func setupTableView() {
        tableView = UITableView()
        tableView.delegate = self
        tableView.dataSource = self
        tableView.register(UITableViewCell.self, forCellReuseIdentifier: "cell")
        tableView.allowsMultipleSelection = true
        tableView.backgroundColor = .black
        tableView.translatesAutoresizingMaskIntoConstraints = false // Disable autoresizing mask to enable auto layout
        view.addSubview(tableView)

        // Set Auto Layout constraints
        NSLayoutConstraint.activate([
            tableView.topAnchor.constraint(equalTo: view.safeAreaLayoutGuide.topAnchor),
            tableView.leadingAnchor.constraint(equalTo: view.safeAreaLayoutGuide.leadingAnchor),
            tableView.trailingAnchor.constraint(equalTo: view.safeAreaLayoutGuide.trailingAnchor),
            tableView.bottomAnchor.constraint(equalTo: view.safeAreaLayoutGuide.centerYAnchor) // Adjusts to half the view's height
        ])
    }

    func setupResultsTableView() {
        resultsTableView = UITableView()
        resultsTableView.delegate = self
        resultsTableView.dataSource = self
        resultsTableView.register(BenchmarkResultCell.self, forCellReuseIdentifier: "BenchmarkResultCell")
        resultsTableView.allowsSelection = false
        resultsTableView.backgroundColor = .black
        resultsTableView.translatesAutoresizingMaskIntoConstraints = false // Disable autoresizing mask to enable auto layout
        view.addSubview(resultsTableView)

        // Set Auto Layout constraints
        NSLayoutConstraint.activate([
            resultsTableView.topAnchor.constraint(equalTo: view.safeAreaLayoutGuide.centerYAnchor),
            resultsTableView.leadingAnchor.constraint(equalTo: view.safeAreaLayoutGuide.leadingAnchor),
            resultsTableView.trailingAnchor.constraint(equalTo: view.safeAreaLayoutGuide.trailingAnchor),
            resultsTableView.bottomAnchor.constraint(equalTo: submitButton.topAnchor, constant: -20)
        ])
    }
    
    func setupBenchmarkButton() {
        submitButton = UIButton(type: .system)
        submitButton.frame = CGRect(x: 20, y: view.bounds.height - 80, width: view.bounds.width - 60, height: 50)
        submitButton.setTitle("Generating Benchmarks", for: .normal)
        submitButton.addTarget(self, action: #selector(submitAction), for: .touchUpInside)
        submitButton.backgroundColor = .systemBlue
        submitButton.setTitleColor(.white, for: .normal)
        submitButton.layer.cornerRadius = 10
        submitButton.translatesAutoresizingMaskIntoConstraints = false // Disable autoresizing mask to enable auto layout
        view.addSubview(submitButton)

        // Set Auto Layout constraints
        NSLayoutConstraint.activate([
            submitButton.leadingAnchor.constraint(equalTo: view.safeAreaLayoutGuide.leadingAnchor, constant: 20),
            submitButton.trailingAnchor.constraint(equalTo: view.safeAreaLayoutGuide.trailingAnchor, constant: -20),
            submitButton.bottomAnchor.constraint(equalTo: view.safeAreaLayoutGuide.bottomAnchor, constant: -20),
            submitButton.heightAnchor.constraint(equalToConstant: 50)
        ])
    }
    
    // UITableViewDataSource methods
    func tableView(_ tableView: UITableView, numberOfRowsInSection section: Int) -> Int {
        if tableView == self.tableView {
            return algorithms.count
        } else {    // Results table
            return benchmarkResults.count
        }
    }
    
    func tableView(_ tableView: UITableView, cellForRowAt indexPath: IndexPath) -> UITableViewCell {
        if tableView == self.tableView {
            let cell = tableView.dequeueReusableCell(withIdentifier: "cell", for: indexPath)
            cell.textLabel?.text = "\(indexPath.row + 1). \(algorithms[indexPath.row])"
            cell.backgroundColor = .black
            cell.textLabel?.textColor = .white
            
            if selectedAlgorithms.contains(indexPath.row) {
                tableView.selectRow(at: indexPath, animated: false, scrollPosition: .none)
                cell.accessoryType = .checkmark
            } else {
                cell.accessoryType = .none
            }
            return cell
        } else {   // Results table
            guard let cell = tableView.dequeueReusableCell(withIdentifier: "BenchmarkResultCell", for: indexPath) as? BenchmarkResultCell else {
                return UITableViewCell()
            }
            
            let result = benchmarkResults[indexPath.row]
            cell.nameLabel.text = result.algorithm
            cell.avgTimeLabel.text = String(format: "%.2f ms", result.avgMsmTime)
            cell.diffLabel.text = String(format: "%.2f %%", result.diffWithBaseline)
            
            // Adjust the font size or other properties as needed
            cell.nameLabel.font = UIFont.systemFont(ofSize: 14)
            cell.avgTimeLabel.font = UIFont.systemFont(ofSize: 14)
            cell.diffLabel.font = UIFont.systemFont(ofSize: 14)
            
            return cell
        }
    }
    
    // UITableViewDelegate methods for algorithm selection
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

    // UITableViewDelegate methods for header view
    func tableView(_ tableView: UITableView, viewForHeaderInSection section: Int) -> UIView? {
        let headerView = UIView()
        headerView.backgroundColor = UIColor.black
        
        // Create the header label
        let headerLabel = UILabel()
        headerLabel.textColor = .white
        headerLabel.font = UIFont.systemFont(ofSize: 16, weight: .bold)
        
        // Create the horizontal white line view
        let lineView = UIView()
        lineView.backgroundColor = .white // Set the line color
        headerView.addSubview(lineView) // Add the line view to the header view
        lineView.translatesAutoresizingMaskIntoConstraints = false
        
        if tableView == self.tableView {
            headerLabel.text = "Select MSM Algorithms"
        } else {    // Results table
            // Define additional labels for each column if it's the results table
            let nameLabel = UILabel()
            configureHeaderLabel(label: nameLabel, text: "Algo Name")
            
            let avgTimingLabel = UILabel()
            configureHeaderLabel(label: avgTimingLabel, text: "Avg Timing")
            
            let diffLabel = UILabel()
            configureHeaderLabel(label: diffLabel, text: "Diff with Baseline")
            
            // Add additional labels to the header view
            headerView.addSubview(nameLabel)
            headerView.addSubview(avgTimingLabel)
            headerView.addSubview(diffLabel)
            
            // Disable autoresizing mask to enable auto layout
            nameLabel.translatesAutoresizingMaskIntoConstraints = false
            avgTimingLabel.translatesAutoresizingMaskIntoConstraints = false
            diffLabel.translatesAutoresizingMaskIntoConstraints = false
            
            // Set Auto Layout constraints
            NSLayoutConstraint.activate([
                nameLabel.leadingAnchor.constraint(equalTo: headerView.leadingAnchor, constant: 8),
                nameLabel.topAnchor.constraint(equalTo: headerView.topAnchor),
                nameLabel.bottomAnchor.constraint(equalTo: lineView.topAnchor, constant: -8),
                
                avgTimingLabel.centerXAnchor.constraint(equalTo: headerView.centerXAnchor),
                avgTimingLabel.topAnchor.constraint(equalTo: headerView.topAnchor),
                avgTimingLabel.bottomAnchor.constraint(equalTo: lineView.topAnchor, constant: -8),
                
                diffLabel.trailingAnchor.constraint(equalTo: headerView.trailingAnchor, constant: -8),
                diffLabel.topAnchor.constraint(equalTo: headerView.topAnchor),
                diffLabel.bottomAnchor.constraint(equalTo: lineView.topAnchor, constant: -8),
            ])
        }

        headerView.addSubview(headerLabel)
        headerLabel.translatesAutoresizingMaskIntoConstraints = false
        
        // Set Auto Layout constraints
        NSLayoutConstraint.activate([
            headerLabel.leadingAnchor.constraint(equalTo: headerView.leadingAnchor, constant: 16),
            headerLabel.topAnchor.constraint(equalTo: headerView.topAnchor, constant: 8),
            headerLabel.trailingAnchor.constraint(equalTo: headerView.trailingAnchor, constant: -16),
            lineView.heightAnchor.constraint(equalToConstant: 1), // Line thickness
            lineView.leadingAnchor.constraint(equalTo: headerView.leadingAnchor),
            lineView.trailingAnchor.constraint(equalTo: headerView.trailingAnchor),
            lineView.bottomAnchor.constraint(equalTo: headerView.bottomAnchor), // Position at the bottom
            lineView.topAnchor.constraint(equalTo: headerLabel.bottomAnchor, constant: 8) // Space between label and line
        ])
        
        return headerView
    }

    func configureHeaderLabel(label: UILabel, text: String) {
        label.text = text
        label.textColor = .white
        label.font = UIFont.systemFont(ofSize: 14, weight: .bold)
    }

    @objc func submitAction() {
        print("Selected algorithms: \(selectedAlgorithms.map { algorithms[$0] })")

        // offload heavy computation of benchmarking in background
        DispatchQueue.global(qos: .userInitiated).async { [weak self] in
            guard let self = self else { return }

            var tempResults: [AlgorithmBenchmark] = []
            var baselineTiming: Double = 0.0

            for index in self.selectedAlgorithms.sorted() {
                let algorithm = self.algorithms[index]

                if let benchmarkFunction = self.msmBenchmarkMapping[algorithm] {
                    do {
                        print("Running MSM in algorithm: \(algorithm)...")
                        let benchData: BenchmarkResult = try benchmarkFunction(10000)
                        if algorithm == "Arkwork (Baseline)" {
                            baselineTiming = benchData.avgProcessingTime
                        }

                        let algorithmBenchmark = AlgorithmBenchmark(
                            algorithm: algorithm,
                            avgMsmTime: benchData.avgProcessingTime,
                            // Calculate the percentage difference with baseline
                            diffWithBaseline: (benchData.avgProcessingTime - baselineTiming) / baselineTiming * 100
                        )
                        tempResults.append(algorithmBenchmark)
                        print("Result of \(algorithmBenchmark.algorithm): \(algorithmBenchmark.avgMsmTime) ms (diff: \(algorithmBenchmark.diffWithBaseline) %)")

                    } catch {
                        print("Error running benchmark: \(error)")
                    }
                } else {
                    print("No benchmark function found for \(algorithm)")
                    tempResults.append(AlgorithmBenchmark(algorithm: algorithm, avgMsmTime: Double.nan, diffWithBaseline: Double.nan))
                }
            }

            DispatchQueue.main.async {
                self.benchmarkResults = tempResults
                self.resultsTableView.reloadData()
            }
        }
    }

}

class BenchmarkResultCell: UITableViewCell {
    let nameLabel = UILabel()
    let avgTimeLabel = UILabel()
    let diffLabel = UILabel()

    override init(style: UITableViewCell.CellStyle, reuseIdentifier: String?) {
        super.init(style: style, reuseIdentifier: reuseIdentifier)
        setupViews()
        self.backgroundColor = .black
    }
    
    required init?(coder aDecoder: NSCoder) {
        fatalError("init(coder:) has not been implemented")
    }
    
    private func setupViews() {
        // Add labels to the cell's contentView
        [nameLabel, avgTimeLabel, diffLabel].forEach {
            $0.translatesAutoresizingMaskIntoConstraints = false
            contentView.addSubview($0)
            $0.textColor = .white
        }
        
        // Set Auto Layout constraints
        NSLayoutConstraint.activate([
            nameLabel.leadingAnchor.constraint(equalTo: contentView.leadingAnchor, constant: 8),
            nameLabel.topAnchor.constraint(equalTo: contentView.topAnchor, constant: 8), // Add top constraint
            nameLabel.bottomAnchor.constraint(equalTo: contentView.bottomAnchor, constant: -8), // Add bottom constraint to ensure vertical spacing is defined
            
            avgTimeLabel.centerXAnchor.constraint(equalTo: contentView.centerXAnchor),
            avgTimeLabel.centerYAnchor.constraint(equalTo: contentView.centerYAnchor),
            
            diffLabel.trailingAnchor.constraint(equalTo: contentView.trailingAnchor, constant: -8),
            diffLabel.centerYAnchor.constraint(equalTo: contentView.centerYAnchor),
        ])
    }
}
