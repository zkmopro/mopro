mod halo2;
mod circom;

#[derive(Debug)]
pub enum FFIError {
    MoproError(mopro_core::MoproError),
    SerializationError(String),
}

#[derive(Debug, Clone)]
pub struct GenerateProofResult {
    pub proof: Vec<u8>,
    pub inputs: Vec<u8>,
}

impl From<MoproError> for FFIError {
    fn from(error: MoproError) -> Self {
        FFIError::MoproError(error)
    }
}

pub use circom::*;
pub use halo2::*;
use mopro_core::MoproError;


fn add(a: u32, b: u32) -> u32 {
    a + b
}

fn hello() -> String {
    "Hello World from Rust".to_string()
}

// TODO: Remove me
// UniFFI expects String type
// See https://mozilla.github.io/uniffi-rs/udl/builtin_types.html
// fn run_example(wasm_path: String, r1cs_path: String) -> Result<(), MoproError> {
//     circom::run_example(wasm_path.as_str(), r1cs_path.as_str())
// }

uniffi::include_scaffolding!("mopro");

#[cfg(test)]
mod tests {
    use core::num;

    use super::*;
    use ark_bn254::Fr;
    use num_bigint::BigUint;

    #[test]
    fn add_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

}
