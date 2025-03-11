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
        prover::prove(proof_lib, zkey_path.clone(), wit_thread)
    }

    pub fn verify(
        proof_lib: ProofLib,
        proof: Vec<u8>,
        public_inputs: Vec<u8>,
        zkey_path: String,
    ) -> Result<bool> {
        prover::verify(proof_lib, zkey_path, proof, public_inputs)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use ark_bn254::Bn254;

    use crate::prover::serialization::{self, SerializableProof};

    use super::*;
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
        let ark_proof: ark_groth16::Proof<Bn254> = proof.proof.into();
        CircomProver::verify(
            proof_lib,
            serialization::serialize_proof(&SerializableProof(ark_proof)),
            proof.pub_inputs,
            ZKEY_PATH.to_string(),
        )
        .unwrap()
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

    #[cfg(all(feature = "rustwitness", feature = "rapidsnark"))]
    #[test]
    fn test_rustwitness_rapidsnark_prove_and_verify() {
        rust_witness::witness!(multiplier2);

        let proof = generate_proof(
            WitnessFn::RustWitness(multiplier2_witness),
            ProofLib::RapidSnark,
        );
        assert!(verify_proof(proof, ProofLib::RapidSnark));
    }
}
