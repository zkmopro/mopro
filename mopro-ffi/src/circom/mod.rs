use crate::{MoproError, ProofCalldata, G1, G2};
mod serialization;

use serialization::{SerializableInputs, SerializableProof};

use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::str::FromStr;
use std::time::Instant;

use crate::GenerateProofResult;
use ark_bn254::Bn254;
use ark_circom::{read_zkey, CircomReduction};

use ark_crypto_primitives::snark::SNARK;
use ark_groth16::{prepare_verifying_key, Groth16};
use ark_std::UniformRand;

use ark_std::rand::thread_rng;
use color_eyre::Result;

use num_bigint::BigInt;

pub type WtnsFn = fn(HashMap<String, Vec<BigInt>>) -> Vec<BigInt>;
type GrothBn = Groth16<Bn254>;

rust_witness::witness!(multiplier2);
rust_witness::witness!(keccak256256test);

// This should be defined by a file that the mopro package consumer authors
// then we reference it in our build somehow
pub fn circuit_data(zkey_path: &str) -> Result<WtnsFn, MoproError> {
    let name = Path::new(zkey_path).file_stem().unwrap();
    match name.to_str().unwrap() {
        "multiplier2_final" => Ok(multiplier2_witness),
        "keccak256_256_test_final" => Ok(keccak256256test_witness),
        _ => Err(MoproError::CircomError("Unknown circuit name".to_string())),
    }
}

pub fn generate_circom_proof(
    zkey_path: String,
    inputs: HashMap<String, Vec<String>>,
) -> Result<GenerateProofResult, MoproError> {
    let witness_fn = circuit_data(zkey_path.as_str())?;
    let mut file = File::open(zkey_path).map_err(|e| MoproError::CircomError(e.to_string()))?;
    let zkey = read_zkey(&mut file).map_err(|e| MoproError::CircomError(e.to_string()))?;

    // Form the inputs
    let bigint_inputs = inputs
        .into_iter()
        .map(|(k, v)| {
            (
                k,
                v.into_iter()
                    .map(|i| BigInt::from_str(&i).unwrap())
                    .collect(),
            )
        })
        .collect();

    // build the proof
    let mut rng = thread_rng();
    let rng = &mut rng;

    let r = ark_bn254::Fr::rand(rng);
    let s = ark_bn254::Fr::rand(rng);

    let full_assignment = witness_fn(bigint_inputs)
        .into_iter()
        .map(|w| ark_bn254::Fr::from(w.to_biguint().unwrap()))
        .collect::<Vec<_>>();

    let public_inputs = full_assignment.as_slice()[1..zkey.1.num_instance_variables].to_vec();

    let now = std::time::Instant::now();
    let ark_proof = Groth16::<_, CircomReduction>::create_proof_with_reduction_and_matrices(
        &zkey.0,
        r,
        s,
        &zkey.1,
        zkey.1.num_instance_variables,
        zkey.1.num_constraints,
        full_assignment.as_slice(),
    );

    let proof = ark_proof.map_err(|e| MoproError::CircomError(e.to_string()))?;
    // Ok((SerializableProof(proof), SerializableInputs(public_inputs)))

    println!("proof generation took: {:.2?}", now.elapsed());
    // let (proof, inputs) = prover.generate_proof(bigint_inputs)?;

    Ok(GenerateProofResult {
        proof: serialization::serialize_proof(&SerializableProof(proof)),
        inputs: serialization::serialize_inputs(&SerializableInputs(public_inputs)),
    })
}

pub fn verify_circom_proof(
    zkey_path: String,
    proof: Vec<u8>,
    public_input: Vec<u8>,
) -> Result<bool, MoproError> {
    let deserialized_proof = serialization::deserialize_proof(proof);
    let deserialized_public_input = serialization::deserialize_inputs(public_input);
    let mut file = File::open(zkey_path).map_err(|e| MoproError::CircomError(e.to_string()))?;
    let zkey = read_zkey(&mut file).map_err(|e| MoproError::CircomError(e.to_string()))?;
    let start = Instant::now();
    let pvk = prepare_verifying_key(&zkey.0.vk);

    let proof_verified = GrothBn::verify_with_processed_vk(
        &pvk,
        &deserialized_public_input.0,
        &deserialized_proof.0,
    )
    .map_err(|e| MoproError::CircomError(e.to_string()))?;

    let verification_duration = start.elapsed();
    println!("Verification time: {:?}", verification_duration);
    Ok(proof_verified)
}

// Convert proof to String-tuples as expected by the Solidity Groth16 Verifier
pub fn to_ethereum_proof(proof: Vec<u8>) -> ProofCalldata {
    let deserialized_proof = serialization::deserialize_proof(proof);
    let proof = serialization::to_ethereum_proof(&deserialized_proof);
    let a = G1 {
        x: proof.a.x.to_string(),
        y: proof.a.y.to_string(),
    };
    let b = G2 {
        x: proof.b.x.iter().map(|x| x.to_string()).collect(),
        y: proof.b.y.iter().map(|x| x.to_string()).collect(),
    };
    let c = G1 {
        x: proof.c.x.to_string(),
        y: proof.c.y.to_string(),
    };
    ProofCalldata { a, b, c }
}

pub fn to_ethereum_inputs(inputs: Vec<u8>) -> Vec<String> {
    let deserialized_inputs = serialization::deserialize_inputs(inputs);
    let inputs = deserialized_inputs
        .0
        .iter()
        .map(|x| x.to_string())
        .collect();
    inputs
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::str::FromStr;

    use crate::circom::{generate_circom_proof, serialization, verify_circom_proof, MoproError};
    use ark_bn254::Fr;
    use num_bigint::BigInt;

    use crate::circom::{to_ethereum_inputs, to_ethereum_proof};

    fn bytes_to_bits(bytes: &[u8]) -> Vec<bool> {
        let mut bits = Vec::new();
        for &byte in bytes {
            for j in 0..8 {
                let bit = (byte >> j) & 1;
                bits.push(bit == 1);
            }
        }
        bits
    }

    fn bytes_to_circuit_inputs(input_vec: &Vec<u8>) -> HashMap<String, Vec<String>> {
        let bits = bytes_to_bits(&input_vec);
        let converted_vec: Vec<String> = bits
            .into_iter()
            .map(|bit| (bit as i32).to_string())
            .collect();
        let mut inputs = HashMap::new();
        inputs.insert("in".to_string(), converted_vec);
        inputs
    }

    fn bytes_to_circuit_outputs(bytes: &[u8]) -> Vec<u8> {
        let bits = bytes_to_bits(bytes);
        let field_bits = bits.into_iter().map(|bit| Fr::from(bit as u8)).collect();
        let circom_outputs = serialization::SerializableInputs(field_bits);
        serialization::serialize_inputs(&circom_outputs)
    }

    #[test]
    fn test_end_to_end() -> Result<(), MoproError> {
        // Create a new MoproCircom instance
        let zkey_path = "./test-vectors/circom/multiplier2_final.zkey".to_string();

        let mut inputs = HashMap::new();
        let a = BigInt::from_str(
            "21888242871839275222246405745257275088548364400416034343698204186575808495616",
        )
        .unwrap();
        let b = BigInt::from(1u8);
        let c = a.clone() * b.clone();
        inputs.insert("a".to_string(), vec![a.to_string()]);
        inputs.insert("b".to_string(), vec![b.to_string()]);
        // output = [public output c, public input a]
        let expected_output = vec![
            Fr::from(c.clone().to_biguint().unwrap()),
            Fr::from(a.clone().to_biguint().unwrap()),
        ];
        let circom_outputs = serialization::SerializableInputs(expected_output);
        let serialized_outputs = serialization::serialize_inputs(&circom_outputs);

        // Generate Proof
        let p = generate_circom_proof(zkey_path.clone(), inputs)?;
        let serialized_proof = p.proof;
        let serialized_inputs = p.inputs;

        assert!(serialized_proof.len() > 0);
        assert_eq!(serialized_inputs, serialized_outputs);

        // Step 3: Verify Proof
        let is_valid = verify_circom_proof(
            zkey_path,
            serialized_proof.clone(),
            serialized_inputs.clone(),
        )?;
        assert!(is_valid);

        // Step 4: Convert Proof to Ethereum compatible proof
        let proof_calldata = to_ethereum_proof(serialized_proof);
        let inputs_calldata = to_ethereum_inputs(serialized_inputs);
        assert!(proof_calldata.a.x.len() > 0);
        assert!(inputs_calldata.len() > 0);

        Ok(())
    }

    #[test]
    fn test_end_to_end_keccak() -> Result<(), MoproError> {
        // Create a new MoproCircom instance
        let zkey_path = "./test-vectors/circom/keccak256_256_test_final.zkey".to_string();
        // Prepare inputs
        let input_vec = vec![
            116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0,
        ];

        // Expected output
        let expected_output_vec = vec![
            37, 17, 98, 135, 161, 178, 88, 97, 125, 150, 143, 65, 228, 211, 170, 133, 153, 9, 88,
            212, 4, 212, 175, 238, 249, 210, 214, 116, 170, 85, 45, 21,
        ];

        let inputs = bytes_to_circuit_inputs(&input_vec);
        let serialized_outputs = bytes_to_circuit_outputs(&expected_output_vec);

        // Generate Proof
        let p = generate_circom_proof(zkey_path.clone(), inputs)?;
        let serialized_proof = p.proof;
        let serialized_inputs = p.inputs;

        assert!(serialized_proof.len() > 0);
        assert_eq!(serialized_inputs, serialized_outputs);

        // Verify Proof

        let is_valid = verify_circom_proof(
            zkey_path,
            serialized_proof.clone(),
            serialized_inputs.clone(),
        )?;
        assert!(is_valid);

        // Step 4: Convert Proof to Ethereum compatible proof
        let proof_calldata = to_ethereum_proof(serialized_proof);
        let inputs_calldata = to_ethereum_inputs(serialized_inputs);
        assert!(proof_calldata.a.x.len() > 0);
        assert!(inputs_calldata.len() > 0);

        Ok(())
    }
}
