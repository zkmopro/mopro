import mopro
import Foundation

let instanceSize: UInt32 = 16;
let numInstance: UInt32 = 10;
let utilsDir = "../../../mopro-core/src/middleware/gpu_explorations/utils/vectors/16x10";

// test with arkworks pippenger
do {
    let result = try arkworksPippenger(
        instanceSize: instanceSize, 
        numInstance: numInstance, 
        utilsDir: utilsDir 
    );
    print("Benchmark result: \(result)");
} catch let error as MoproError{
    print("Error running benchmark: \(error)")
}

// test with trapdoor zprize msm
// do {
//     let result = try trapdoortechZprizeMsm(
//         instanceSize: instanceSize, 
//         numInstance: numInstance, 
//         utilsDir: utilsDir
//     );
//     print("Benchmark result: \(result)");
// } catch let error as MoproError{
//     print("Error running benchmark: \(error)")
// }

// test with metal_msm
do {
    let result = try metalMsm(
        instanceSize: 10, 
        numInstance: 10, 
        utilsDir: "../../../mopro-core/src/middleware/gpu_explorations/utils/vectors/10x10"
    );
    print("Benchmark result: \(result)");
} catch let error as MoproError{
    print("Error running benchmark: \(error)")
}