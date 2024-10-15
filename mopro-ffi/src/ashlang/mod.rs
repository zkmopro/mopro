use std::fs;

use ashlang::AshlangProver;
use ashlang::SpartanProver;

use super::GenerateProofResult;
use super::MoproError;

pub fn prove(
    ar1cs_path: String, // path to ar1cs file
    secret_inputs: Vec<String>,
) -> Result<GenerateProofResult, MoproError> {
    let ir_source = fs::read_to_string(&ar1cs_path).unwrap();
    // we pass an empty vec for public inputs because
    // they are not supported in the ashlang spartan prover
    // outputs are public and should be used instead
    let proof = SpartanProver::prove_ir(&ir_source, vec![], secret_inputs.clone()).unwrap();

    Ok(GenerateProofResult {
        proof: bincode::serialize(&proof.snark).unwrap(),
        inputs: bincode::serialize(&secret_inputs).unwrap(),
    })
}

pub fn verify(in0: String, in1: Vec<u8>, in2: Vec<u8>) -> Result<bool, MoproError> {
    panic!("ashlang is not enabled in this build. Please pass `ashlang` feature to `mopro-ffi` to enable it.")
}

#[cfg(test)]
mod tests {
    use ::ashlang::AshlangProver;

    use crate::ashlang;

    #[test]
    fn test_ashlang_prove_verify() -> anyhow::Result<()> {
        let proof = ashlang::prove(
            "../test-vectors/ashlang/example.ar1cs".to_string(),
            vec!["55".to_string()],
        )?;

        // ashlang::SpartanProver::verify(bincode::deserialize(&proof).unwrap())?;

        Ok(())
    }
}
