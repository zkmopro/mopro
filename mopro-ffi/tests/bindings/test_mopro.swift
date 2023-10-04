import mopro

let moproCircom = MoproCircom()

let wasmPath = "./../mopro-core/examples/circom/target/multiplier2_js/multiplier2.wasm"
let r1csPath = "./../mopro-core/examples/circom/target/multiplier2.r1cs"

do {
    let setupResult = try moproCircom.setup(wasmPath: wasmPath, r1csPath: r1csPath)
    assert(!setupResult.provingKey.isEmpty, "Proving key should not be empty")
    assert(!setupResult.inputs.isEmpty, "Inputs should not be empty")
} catch let error as MoproError {
    print("MoproError: \(error)")
} catch {
    print("Unexpected error: \(error)")
}