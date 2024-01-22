import mopro
import Foundation

let path = FileManager.default.currentDirectoryPath + "../benchmarks/gpu_explorations/msm_bench.csv"
let fileURL = URL(fileURLWithPath: path)
var fileContent = ""

// generate trials = [1, 500, 1000, 1500, ... ,10000]
let trials: [UInt32] = (0..<21).map { max($0 * 500, 1)}

for each in trials {
    do {
        let benchData: BenchmarkResult = try runMsmBenchmark(numMsm: each)
        fileContent += "\(benchData.numMsm),\(benchData.avgProcessingTime),\(benchData.totalProcessingTime),\(benchData.allocatedMemory)\n"
    } catch let error as MoproError{
        print("Error running benchmark: \(error)")
    }
}

print(fileContent)

// do {
//     try fileContent.write(to: fileURL, atomically: true, encoding:.utf8)
// } catch {
//         print("Error writing file: \(error)")
// }