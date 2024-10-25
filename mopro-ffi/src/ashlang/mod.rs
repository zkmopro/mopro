use std::fs;

use ashlang::AshlangProver;
use ashlang::SpartanProver;

use super::GenerateProofResult;

#[macro_export]
macro_rules! ashlang_spartan_app {
    () => {
        fn generate_ashlang_spartan_proof(
            ar1cs_path: String, // path to ar1cs file
            secret_inputs: Vec<String>,
        ) -> Result<mopro_ffi::GenerateProofResult, mopro_ffi::MoproError> {
            mopro_ffi::ashlang::prove(&ar1cs_path, secret_inputs).map_err(|e| {
                mopro_ffi::MoproError::AshlangError(
                    "error generating ashlang spartan proof".to_string(),
                )
            })
        }

        fn verify_ashlang_spartan_proof(
            ar1cs_path: String,
            proof: Vec<u8>,
        ) -> Result<bool, mopro_ffi::MoproError> {
            mopro_ffi::ashlang::verify(&ar1cs_path, proof).map_err(|e| {
                mopro_ffi::MoproError::AshlangError("error verifying proof".to_string())
            })
        }
    };
}

/// Generates a spartan proof from an ar1cs file compiled
/// with the ashlang compiler.
pub fn prove(
    ar1cs_path: &str, // path to ar1cs file
    secret_inputs: Vec<String>,
) -> anyhow::Result<GenerateProofResult> {
    let ir_source = fs::read_to_string(ar1cs_path)?;
    // we pass an empty vec for public inputs because
    // they are not supported in the ashlang spartan prover
    // outputs are public and should be used instead
    let proof = SpartanProver::prove_ir(&ir_source, vec![], secret_inputs.clone())?;

    Ok(GenerateProofResult {
        proof: bincode::serialize(&proof)?,
        inputs: vec![],
    })
}

/// Verifies a spartan proof from an ar1cs file
pub fn verify(ar1cs_path: &str, proof: Vec<u8>) -> anyhow::Result<bool> {
    let ir_source = fs::read_to_string(ar1cs_path)?;
    let p = bincode::deserialize(&proof)?;
    ashlang::SpartanProver::verify(&ir_source, p)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ashlang_spartan_prove_verify() -> anyhow::Result<()> {
        let ar1cs_path = "../test-vectors/ashlang/example.ar1cs".to_string();
        let proof = prove(&ar1cs_path, vec!["55".to_string()])?;
        verify(&ar1cs_path, proof.proof)?;
        Ok(())
    }

    #[test]
    fn test_ashlang_spartan_multiplier2() -> anyhow::Result<()> {
        let ar1cs_path = "../test-vectors/ashlang/multiplier2.ar1cs".to_string();
        let proof = prove(&ar1cs_path, vec!["55".to_string(), "56".to_string()])?;
        verify(&ar1cs_path, proof.proof)?;
        Ok(())
    }
}
