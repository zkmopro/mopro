use crate::{MoproError, WtnsFn};
pub mod serialization;
mod zkey;
mod zkey_header;

use ark_bls12_381::Bls12_381;
use ark_bn254::Bn254;
use ark_ec::pairing::Pairing;
use ark_ff::PrimeField;
use ark_relations::r1cs::ConstraintMatrices;
use serialization::{SerializableInputs, SerializableProof};

use zkey::{BinFile, FieldSerialization};
use zkey_header::ZkeyHeaderReader;

use std::collections::HashMap;
use std::fs::File;
use std::str::FromStr;

use crate::GenerateProofResult;
use ark_circom::CircomReduction;

use ark_crypto_primitives::snark::SNARK;
use ark_groth16::{prepare_verifying_key, Groth16, ProvingKey, VerifyingKey};
use ark_std::UniformRand;

use ark_std::rand::thread_rng;
use color_eyre::Result;

use num_bigint::{BigInt, BigUint};

// build a proof for a zkey using witness_fn to build
// the witness
pub fn generate_circom_proof_wtns(
    zkey_path: String,
    inputs: HashMap<String, Vec<String>>,
    witness_fn: WtnsFn,
) -> Result<GenerateProofResult, MoproError> {
    // We'll start a background thread building the witness
    let witness_thread = std::thread::spawn(move || {
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

        witness_fn(bigint_inputs)
            .into_iter()
            .map(|w| w.to_biguint().unwrap())
            .collect::<Vec<_>>()
    });

    // here we make a loader just to get the groth16 header
    // this header tells us what curve the zkey was compiled for
    //
    // we specify the BinFile as Bn254 here just to satisfy the compiler
    // we'll make a separate instance after this using the correct type
    let mut header_reader = ZkeyHeaderReader::new(&zkey_path);
    header_reader.read();
    let file = File::open(&zkey_path).map_err(|e| MoproError::CircomError(e.to_string()))?;
    let mut reader = std::io::BufReader::new(file);
    // check the prime in the header
    // println!("{} {} {}", header.q, header.n8q, ark_bls12_381::Fq::MODULUS);
    if header_reader.r == BigUint::from(ark_bn254::Fr::MODULUS) {
        let mut binfile = BinFile::<_, Bn254>::new(&mut reader)
            .map_err(|e| MoproError::CircomError(e.to_string()))?;
        let proving_key = binfile
            .proving_key()
            .map_err(|e| MoproError::CircomError(e.to_string()))?;
        let matrices = binfile
            .matrices()
            .map_err(|e| MoproError::CircomError(e.to_string()))?;
        let full_assignment = witness_thread
            .join()
            .map_err(|_e| MoproError::CircomError("Failed to generate witness".to_string()))?;
        return prove(proving_key, matrices, full_assignment);
    } else if header_reader.r == BigUint::from(ark_bls12_381::Fr::MODULUS) {
        let mut binfile = BinFile::<_, Bls12_381>::new(&mut reader)
            .map_err(|e| MoproError::CircomError(e.to_string()))?;
        let proving_key = binfile
            .proving_key()
            .map_err(|e| MoproError::CircomError(e.to_string()))?;
        let matrices = binfile
            .matrices()
            .map_err(|e| MoproError::CircomError(e.to_string()))?;
        let full_assignment = witness_thread
            .join()
            .map_err(|_e| MoproError::CircomError("Failed to generate witness".to_string()))?;
        return prove(proving_key, matrices, full_assignment);
    } else {
        // unknown curve
        // wait for the witness thread to finish for consistency
        witness_thread
            .join()
            .map_err(|_e| MoproError::CircomError("Failed to generate witness".to_string()))?;
        return Err(MoproError::CircomError(
            "unknown curve detected in zkey".to_string(),
        ));
    }
}

// Prove on a generic curve
fn prove<T: Pairing + FieldSerialization>(
    pkey: ProvingKey<T>,
    matrices: ConstraintMatrices<T::ScalarField>,
    witness: Vec<BigUint>,
) -> Result<GenerateProofResult, MoproError> {
    let witness_fr = witness
        .iter()
        .map(|v| T::ScalarField::from(v.clone()))
        .collect::<Vec<_>>();
    let mut rng = thread_rng();
    let rng = &mut rng;
    let r = T::ScalarField::rand(rng);
    let s = T::ScalarField::rand(rng);
    let public_inputs = witness_fr.as_slice()[1..matrices.num_instance_variables].to_vec();

    // build the proof
    let ark_proof = Groth16::<T, CircomReduction>::create_proof_with_reduction_and_matrices(
        &pkey,
        r,
        s,
        &matrices,
        matrices.num_instance_variables,
        matrices.num_constraints,
        witness_fr.as_slice(),
    );

    let proof = ark_proof.map_err(|e| MoproError::CircomError(e.to_string()))?;
    // println!("{} {} {}", proof.a, proof.b, proof.c);
    // println!("{} {}",public_inputs.len(), public_inputs[0]);

    Ok(GenerateProofResult {
        proof: serialization::serialize_proof(&SerializableProof(proof)),
        inputs: serialization::serialize_inputs(&SerializableInputs::<T>(public_inputs)),
    })
}

// Prove on a generic curve
pub fn verify_circom_proof(
    zkey_path: String,
    proof: Vec<u8>,
    public_input: Vec<u8>,
) -> Result<bool, MoproError> {
    let mut header_reader = ZkeyHeaderReader::new(&zkey_path);
    header_reader.read();
    let file = File::open(&zkey_path).map_err(|e| MoproError::CircomError(e.to_string()))?;
    let mut reader = std::io::BufReader::new(file);
    if header_reader.r == BigUint::from(ark_bn254::Fr::MODULUS) {
        let mut binfile = BinFile::<_, Bn254>::new(&mut reader)
            .map_err(|e| MoproError::CircomError(e.to_string()))?;
        let proving_key = binfile
            .proving_key()
            .map_err(|e| MoproError::CircomError(e.to_string()))?;
        let p = serialization::deserialize_inputs::<Bn254>(public_input);
        return verify(proving_key.vk, p.0, proof);
    } else if header_reader.r == BigUint::from(ark_bls12_381::Fr::MODULUS) {
        let mut binfile = BinFile::<_, Bls12_381>::new(&mut reader)
            .map_err(|e| MoproError::CircomError(e.to_string()))?;
        let proving_key = binfile
            .proving_key()
            .map_err(|e| MoproError::CircomError(e.to_string()))?;
        let p = serialization::deserialize_inputs::<Bls12_381>(public_input);
        return verify(proving_key.vk, p.0, proof);
    } else {
        // unknown curve
        return Err(MoproError::CircomError(
            "unknown curve detected in zkey".to_string(),
        ));
    }
}

fn verify<T: Pairing + FieldSerialization>(
    vk: VerifyingKey<T>,
    public_inputs: Vec<T::ScalarField>,
    proof: Vec<u8>,
) -> Result<bool, MoproError> {
    let pvk = prepare_verifying_key(&vk);
    let public_inputs_fr = public_inputs
        .iter()
        .map(|v| T::ScalarField::from(v.clone()))
        .collect::<Vec<_>>();
    let proof_parsed = serialization::deserialize_proof::<T>(proof);
    let verified = Groth16::<T, CircomReduction>::verify_with_processed_vk(
        &pvk,
        &public_inputs_fr,
        &proof_parsed.0,
    )
    .map_err(|e| MoproError::CircomError(e.to_string()))?;
    Ok(verified)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::ops::{Add, Mul};
    use std::str::FromStr;

    use crate::circom::{generate_circom_proof_wtns, serialization, verify_circom_proof, WtnsFn};
    use crate::{GenerateProofResult, MoproError};
    use ark_bls12_381::Bls12_381;
    use ark_bn254::Bn254;
    use ark_ff::PrimeField;
    use num_bigint::{BigInt, BigUint, ToBigInt};
    use serialization::{to_ethereum_inputs, to_ethereum_proof};

    // Only build the witness functions for tests, don't bundle them into
    // the final library
    rust_witness::witness!(multiplier2);
    rust_witness::witness!(multiplier2bls);
    rust_witness::witness!(keccak256256test);
    rust_witness::witness!(hashbenchbls);

    // This should be defined by a file that the mopro package consumer authors
    // then we reference it in our build somehow
    fn zkey_witness_map(name: &str) -> Result<WtnsFn, MoproError> {
        match name {
            "multiplier2_final.zkey" => Ok(multiplier2_witness),
            "keccak256_256_test_final.zkey" => Ok(keccak256256test_witness),
            "hashbench_bls_final.zkey" => Ok(hashbenchbls_witness),
            "multiplier2_bls_final.zkey" => Ok(multiplier2bls_witness),
            _ => Err(MoproError::CircomError("Unknown circuit name".to_string())),
        }
    }

    fn generate_circom_proof(
        zkey_path: String,
        inputs: HashMap<String, Vec<String>>,
    ) -> Result<GenerateProofResult, MoproError> {
        let name = std::path::Path::new(zkey_path.as_str())
            .file_name()
            .unwrap();
        if let Ok(witness_fn) = zkey_witness_map(&name.to_str().unwrap()) {
            generate_circom_proof_wtns(zkey_path, inputs, witness_fn)
        } else {
            Err(MoproError::CircomError("Unknown ZKEY".to_string()))
        }
    }

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
        let field_bits = bits
            .into_iter()
            .map(|bit| ark_bn254::Fr::from(bit as u8))
            .collect();
        let circom_outputs = serialization::SerializableInputs::<Bn254>(field_bits);
        serialization::serialize_inputs(&circom_outputs)
    }

    #[test]
    fn test_prove() -> Result<(), MoproError> {
        // Create a new MoproCircom instance
        let zkey_path = "../test-vectors/circom/multiplier2_final.zkey".to_string();

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
            ark_bn254::Fr::from(c.clone().to_biguint().unwrap()),
            ark_bn254::Fr::from(a.clone().to_biguint().unwrap()),
        ];
        let circom_outputs = serialization::SerializableInputs::<Bn254>(expected_output);
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
    fn test_prove_keccak() -> Result<(), MoproError> {
        // Create a new MoproCircom instance
        let zkey_path = "../test-vectors/circom/keccak256_256_test_final.zkey".to_string();
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

    #[test]
    #[ignore = "hashbench circuit is having problems"]
    fn test_prove_bls_hashbench() -> Result<(), MoproError> {
        // Create a new MoproCircom instance
        let zkey_path = "../test-vectors/circom/hashbench_bls_final.zkey".to_string();

        let mut inputs = HashMap::new();
        let a = BigInt::from(1);
        let b = BigInt::from(1);
        inputs.insert("inputs".to_string(), vec![a.to_string(), b.to_string()]);
        // output = [public output c, public input a]
        // let expected_output = vec![
        //     Bls12_381::ScalarField::from(BigUint::from_str("").unwrap())
        // ];
        // let circom_outputs = serialization::SerializableInputs::<Bls12_381>(expected_output);
        // let serialized_outputs = serialization::serialize_inputs(&circom_outputs);

        // Generate Proof
        let p = generate_circom_proof(zkey_path.clone(), inputs)?;
        let serialized_proof = p.proof;
        let serialized_inputs = p.inputs;

        assert!(serialized_proof.len() > 0);
        // assert_eq!(serialized_inputs, serialized_outputs);

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
    fn test_prove_bls_multiplier2() -> Result<(), MoproError> {
        // Create a new MoproCircom instance
        let zkey_path = "../test-vectors/circom/multiplier2_bls_final.zkey".to_string();

        let mut inputs = HashMap::new();
        // we're using large numbers to ensure we're in the bls field
        let a = BigInt::from(2).pow(250);
        let b: BigInt = BigInt::from(2).pow(254).add(1240);
        let c = a.clone().mul(b.clone())
            % BigUint::from(ark_bls12_381::Fr::MODULUS)
                .to_bigint()
                .unwrap();
        inputs.insert("a".to_string(), vec![a.to_string()]);
        inputs.insert("b".to_string(), vec![b.to_string()]);
        // output = [public output c, public input a]
        let expected_output = vec![ark_bls12_381::Fr::from(c.to_biguint().unwrap())];
        let circom_outputs = serialization::SerializableInputs::<Bls12_381>(expected_output);
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

        // We don't support formatting for ethereum for the BLS curve.
        // Once the hardfork enables the bls precompile we should
        // revisit this
        //
        // // Step 4: Convert Proof to Ethereum compatible proof
        // let proof_calldata = to_ethereum_proof(serialized_proof);
        // let inputs_calldata = to_ethereum_inputs(serialized_inputs);
        // assert!(proof_calldata.a.x.len() > 0);
        // assert!(inputs_calldata.len() > 0);

        Ok(())
    }
}
