import mopro
import Foundation

let instanceSize: UInt32 = 16;
let numInstance: UInt32 = 10;
let utilsDir = "../mopro-core/src/middleware/gpu_explorations/utils";
let benchmarkDir = "../mopro-core/benchmarks/gpu_explorations";

// test with arkworks pippenger
do {
    let result = try arkworksPippenger(
        instanceSize: instanceSize, 
        numInstance: numInstance, 
        utilsDir: utilsDir, 
        benchmarkDir: benchmarkDir
    );
    print("Benchmark result: \(result)");
} catch let error as MoproError{
    print("Error running benchmark: \(error)")
}

// test with trapdoor zprize msm
do {
    let result = try trapdoortechZprizeMsm(
        instanceSize: instanceSize, 
        numInstance: numInstance, 
        utilsDir: utilsDir, 
        benchmarkDir: benchmarkDir
    );
    print("Benchmark result: \(result)");
} catch let error as MoproError{
    print("Error running benchmark: \(error)")
}