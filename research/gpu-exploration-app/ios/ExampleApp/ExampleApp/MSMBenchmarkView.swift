//
//  MSMBenchmarkView.swift
//  Example GPU Exploration
//
//  Created by Fuchuan Chung on 2024/4/21.
//  Copyright © 2024 CocoaPods. All rights reserved.
//

import SwiftUI
import moproFFI


struct AlgorithmBenchmark {
    var algorithm: String
    var avgMsmTime: Double
    var diffWithBaseline: Double // Baseline is Arkwork Vanilla MSM
}

struct MSMBenchmarkView: View {
    @State private var selectedAlgorithms: Set<Int> = [0] // Default to select the baseline MSM algorithm
    let algorithms = ["Arkwork (Baseline)", "Metal Msm (our work)"]
    @State private var benchmarkResults: [AlgorithmBenchmark] = []
    @State private var isSubmitting: Bool = false
    
    // setting up msm algorithm mapping
    let msmBenchmarkMapping:
    [ String: (
        UInt32,
        UInt32,
        String
    ) throws -> BenchmarkResult] = [
        "Arkwork (Baseline)": arkworksPippenger,
        "Metal Msm (our work)": metalMsm,
//        "TrapdoorTech Zprize": trapdoortechZprizeMsm,
    ]

    var body: some View {
        NavigationView {
            VStack {
                // The MSM algorithm Lists
                List {
                    Section(header: Text("Select MSM Algorithms")) {
                        ForEach(algorithms.indices, id: \.self) { index in
                            HStack {
                                Text("\(index + 1). \(algorithms[index])")
                                Spacer()
                                if selectedAlgorithms.contains(index) {
                                    Image(systemName: "checkmark")
                                }
                            }
                            .onTapGesture {
                                // select the algorithms
                                if selectedAlgorithms.contains(index) {
                                    selectedAlgorithms.remove(index)
                                } else {
                                    selectedAlgorithms.insert(index)
                                }
                            }
                            .foregroundColor(.black)
                            .listRowBackground(Color.white)
                        }
                    }

                    // result lists
                    Section(header: Text("Benchmark Results")) {
                        // Adding titles to the table-like structure
                        HStack {
                            Text("Algorithm")
                                .bold()
                                .frame(width: 120, alignment: .leading)
                            Spacer()
                            Text("Avg Time (ms)")
                                .bold()
                                .frame(width: 120, alignment: .trailing)
                            Text("Diff(%)")
                                .bold()
                                .frame(width: 80, alignment: .trailing)
                        }
                        .foregroundColor(.white)
                        .listRowBackground(Color.gray)
                        
                        // List of results
                        ForEach(benchmarkResults, id: \.algorithm) { result in
                            HStack {
                                Text(result.algorithm)
                                    .frame(width: 120, alignment: .leading)
                                Spacer()
                                Text("\(String(format: "%.2f", result.avgMsmTime))")
                                    .frame(width: 120, alignment: .trailing)
                                Text("\(String(format: result.diffWithBaseline > 0 ? "+%.2f" : "%.2f", result.diffWithBaseline))")
                                    .frame(width: 80, alignment: .trailing)
                            }
                            .foregroundColor(.black)
                            .listRowBackground(Color.white)
                        }
                    }
                }
                .listStyle(DefaultListStyle())
                .background(Color.black.edgesIgnoringSafeArea(.all))

                Button("Generate Benchmarks") {
                    submitAction()
                }
                
                .padding()
                .background(isSubmitting ? Color.gray : Color.blue)
                .foregroundColor(.white)
                .cornerRadius(5)
                .disabled(isSubmitting)
            }
            .navigationBarTitle("MSM Benchmark", displayMode: .inline)
            .navigationBarHidden(false)
        }
    }

    func submitAction() {
        isSubmitting = true
        print("Selected algorithms: \(selectedAlgorithms.map { algorithms[$0] })")
        // print("Downloading Scalars and Points...")
        DispatchQueue.global(qos: .userInitiated).async {
            var tempResults: [AlgorithmBenchmark] = []
            var baselineTiming: Double = 0.0
            
            for index in self.selectedAlgorithms.sorted() {
                let algorithm = self.algorithms[index]
                
                if let benchmarkFunction = self.msmBenchmarkMapping[algorithm] {
                    do {
                        let documentsUrl = FileManager.default.urls(for: .documentDirectory, in: .userDomainMask).first!
                        let documentsPath = documentsUrl.path
                        let instanceSize: UInt32 = 10;
                        let numInstance: UInt32 = 10
                        print("Running MSM in algorithm: \(algorithm)...")
                        let benchData: BenchmarkResult =
                            try benchmarkFunction(
                                10,
                                numInstance,
                                documentsPath
                            )
                        
                        if algorithm == "Arkwork (Baseline)" {
                            baselineTiming = benchData.avgProcessingTime
                        }
                        
                        let algorithmBenchmark = AlgorithmBenchmark(algorithm: algorithm, avgMsmTime: benchData.avgProcessingTime, diffWithBaseline: (baselineTiming - benchData.avgProcessingTime) / baselineTiming * 100
                        )
                        tempResults.append(algorithmBenchmark)
                        print("Result of \(algorithmBenchmark.algorithm): \n \(algorithmBenchmark.avgMsmTime) ms (diff: \(algorithmBenchmark.diffWithBaseline) %)"
                        )
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
                self.isSubmitting = false
            }
        }
    }
}

