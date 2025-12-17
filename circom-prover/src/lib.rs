pub mod prover;
pub mod witness;

use anyhow::Result;
use prover::{CircomProof, ProofLib};

#[cfg(feature = "rapidsnark")]
pub use prover::rapidsnark;

#[cfg(feature = "rustwitness")]
pub use rust_witness::*;
use witness::WitnessFn;
#[cfg(feature = "witnesscalc")]
pub use witnesscalc_adapter;

#[cfg(feature = "circom-witnesscalc")]
#[doc(hidden)]
pub mod __macro_deps {
    pub use anyhow;
    pub use circom_witnesscalc;
    pub use once_cell;
    pub use once_cell::sync::Lazy;
}

#[derive(Debug, Clone)]
pub struct CircomProver {}

impl CircomProver {
    pub fn prove(
        proof_lib: ProofLib,
        wit_fn: WitnessFn,
        json_input_str: String,
        zkey_path: String,
    ) -> Result<CircomProof> {
        let wit_thread = witness::generate_witness(wit_fn, json_input_str);
        prover::prove(proof_lib, zkey_path, wit_thread)
    }

    pub fn verify(proof_lib: ProofLib, proof: CircomProof, zkey_path: String) -> Result<bool> {
        prover::verify(proof_lib, zkey_path, proof)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    const ZKEY_PATH: &str = "./test-vectors/multiplier2_final.zkey";

    fn generate_proof(witness_fn: WitnessFn, proof_lib: ProofLib) -> CircomProof {
        let inputs = HashMap::from([
            ("a".to_string(), vec!["1".to_string()]),
            ("b".to_string(), vec!["2".to_string()]),
        ]);
        let input_str = serde_json::to_string(&inputs).unwrap();
        CircomProver::prove(proof_lib, witness_fn, input_str, ZKEY_PATH.to_string()).unwrap()
    }

    fn verify_proof(proof: CircomProof, proof_lib: ProofLib) -> bool {
        CircomProver::verify(proof_lib, proof, ZKEY_PATH.to_string()).unwrap()
    }

    #[cfg(all(feature = "rustwitness", feature = "arkworks"))]
    #[test]
    fn test_rustwitness_arkworks_prove_and_verify() {
        rust_witness::witness!(multiplier2);
        let proof = generate_proof(
            WitnessFn::RustWitness(multiplier2_witness),
            ProofLib::Arkworks,
        );
        assert!(verify_proof(proof, ProofLib::Arkworks));
    }

    #[cfg(all(feature = "witnesscalc", feature = "arkworks"))]
    #[test]
    fn test_witnesscalc_arkworks_prove_and_verify() {
        witnesscalc_adapter::witness!(multiplier2);
        let proof = generate_proof(
            WitnessFn::WitnessCalc(multiplier2_witness),
            ProofLib::Arkworks,
        );
        assert!(verify_proof(proof, ProofLib::Arkworks));
    }

    #[cfg(all(feature = "circom-witnesscalc", feature = "arkworks"))]
    #[test]
    fn test_circom_witnesscalc_arkworks_prove_and_verify() {
        graph!(multiplier2, "../test-vectors/multiplier2.bin");
        let proof = generate_proof(
            WitnessFn::CircomWitnessCalc(multiplier2_witness),
            ProofLib::Arkworks,
        );
        assert!(verify_proof(proof, ProofLib::Arkworks));
    }

    #[cfg(all(feature = "rustwitness", feature = "rapidsnark"))]
    #[test]
    fn test_rustwitness_rapidsnark_prove_and_verify() {
        rust_witness::witness!(multiplier2);

        let proof = generate_proof(
            WitnessFn::RustWitness(multiplier2_witness),
            ProofLib::Rapidsnark,
        );
        assert!(verify_proof(proof, ProofLib::Rapidsnark));
    }
}
