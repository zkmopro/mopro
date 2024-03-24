import mopro
import Foundation

do {
    let benchData: BenchmarkResult = try arkworksPippenger(numMsm: 1000)
    print("\nBenchmarking \(1000) msm on BN254 curve")
    print("└─ Average msm time: \(benchData.avgProcessingTime) ms")
    print("└─ Overall processing time: \(benchData.totalProcessingTime) ms")
} catch let error as MoproError{
    print("Error running benchmark: \(error)")
}