import mopro
import Foundation

let path = FileManager.default.currentDirectoryPath + "/../../../../mopro-core/benchmarks/gpu_explorations/msm_bench_swift_laptop.csv"
let fileURL = URL(fileURLWithPath: path)
var fileContent = "num_msm,avg_processing_time(ms),total_processing_time(ms)\n"

// generate trials = [1, 500, 1000, 1500, ... ,10000]
let trials: [UInt32] = (0..<21).map { max($0 * 500, 1)}

for each in trials {
    do {
        // for tracking the progress
        print("Running benchmark with \(each) MSMs...")
        let benchData: BenchmarkResult = try arkworksPippenger(numMsm: each)
        fileContent += "\(benchData.numMsm),\(benchData.avgProcessingTime),\(benchData.totalProcessingTime)\n"
    } catch let error as MoproError{
        print("Error running benchmark: \(error)")
    }
}

do {
    try fileContent.write(to: fileURL, atomically: true, encoding:.utf8)
} catch {
        print("Error writing file: \(error)")
}