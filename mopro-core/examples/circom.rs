use mopro_core::middleware::circom::run_example;

fn main() {
    let wasm_path = "./examples/circom/target/multiplier2_js/multiplier2.wasm";
    let r1cs_path = "./examples/circom/target/multiplier2.r1cs";

    match run_example(wasm_path, r1cs_path) {
        Ok(_) => println!("OK"),
        Err(e) => println!("Error: {}", e),
    }
}
