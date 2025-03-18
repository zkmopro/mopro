use crate::{CircomProof, G1, G2};
use ark_bn254::{Bn254, Fq, Fq2, Fr, G1Affine, G2Affine};
use circom_prover::prover::{
    circom::{self, CURVE_BN254, PROTOCOL_GROTH16},
    serialization::{self, deserialize_inputs, SerializableInputs},
};
use num_bigint::BigUint;
use std::str::FromStr;

#[derive(Debug, Clone, Default)]
pub struct ProofCalldata {
    pub a: G1,
    pub b: G2,
    pub c: G1,
}

// Convert proof to U256-tuples as expected by the Solidity Groth16 Verifier
// Only supports bn254 for now
pub fn to_ethereum_proof(proof: CircomProof) -> ProofCalldata {
    let a = G1 {
        x: proof.a.x.to_string(),
        y: proof.a.y.to_string(),
        z: None,
    };
    let b = G2 {
        x: proof.b.x.iter().map(|x| x.to_string()).collect(),
        y: proof.b.y.iter().map(|x| x.to_string()).collect(),
        z: None,
    };
    let c = G1 {
        x: proof.c.x.to_string(),
        y: proof.c.y.to_string(),
        z: None,
    };
    ProofCalldata { a, b, c }
}

// Only supports bn254 for now
pub fn to_ethereum_inputs(inputs: Vec<u8>) -> Vec<String> {
    let deserialized_inputs = deserialize_inputs::<Bn254>(inputs);
    let inputs = circom::Inputs::from(&deserialized_inputs.0[..]);
    let inputs = inputs.0.iter().map(|x| x.to_string()).collect();
    inputs
}

// Only supports bn254 for now
pub fn from_ethereum_inputs(inputs: Vec<String>) -> Vec<u8> {
    let inputs = inputs
        .iter()
        .map(|x| BigUint::from_str(x).unwrap())
        .collect::<Vec<BigUint>>();
    let fr_inputs: Vec<Fr> = circom::Inputs(inputs).into();
    serialization::serialize_inputs(&SerializableInputs::<Bn254>(fr_inputs))
}

// Only supports bn254 for now
pub fn from_ethereum_proof(proof: ProofCalldata) -> CircomProof {
    let a_x = Fq::from_str(&proof.a.x).unwrap();
    let a_y = Fq::from_str(&proof.a.y).unwrap();
    let a = G1Affine::new_unchecked(a_x, a_y);
    let a_biguint = circom::G1::from_bn254(&a);
    let c_x = Fq::from_str(&proof.c.x).unwrap();
    let c_y = Fq::from_str(&proof.c.y).unwrap();
    let c = G1Affine::new_unchecked(c_x, c_y);
    let c_biguint = circom::G1::from_bn254(&c);
    let b1_x = Fq::from_str(&proof.b.x[0]).unwrap();
    let b1_y = Fq::from_str(&proof.b.x[1]).unwrap();
    let b1 = Fq2::new(b1_x, b1_y);
    let b2_x = Fq::from_str(&proof.b.y[0]).unwrap();
    let b2_y = Fq::from_str(&proof.b.y[1]).unwrap();
    let b2 = Fq2::new(b2_x, b2_y);
    let b = G2Affine::new_unchecked(b1, b2);
    let b_biguint = circom::G2::from_bn254(&b);
    CircomProof {
        a: a_biguint.into(),
        b: b_biguint.into(),
        c: c_biguint.into(),
        protocol: PROTOCOL_GROTH16.to_string(),
        curve: CURVE_BN254.to_string(),
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    mod ethereum {
        use super::*;
        use circom_prover::prover::{
            circom::{
                Proof as CircomProverProof, CURVE_BN254, G1 as CircomProverG1,
                G2 as CircomProverG2, PROTOCOL_GROTH16,
            },
            serialization::{deserialize_proof, serialize_proof, SerializableProof},
        };

        #[test]
        fn test_to_ethereum_proof() {
            let raw_proof = vec![
                22, 2, 28, 144, 134, 93, 1, 6, 180, 134, 137, 11, 130, 217, 116, 35, 22, 58, 213,
                215, 39, 9, 76, 99, 93, 46, 166, 183, 200, 20, 234, 26, 86, 182, 126, 104, 167,
                218, 67, 106, 232, 177, 113, 76, 217, 110, 167, 101, 215, 168, 67, 23, 2, 2, 50,
                131, 103, 159, 241, 197, 1, 75, 72, 154, 107, 226, 61, 6, 227, 5, 193, 103, 229,
                40, 232, 183, 170, 218, 136, 73, 194, 166, 135, 22, 128, 83, 94, 84, 179, 66, 38,
                17, 200, 0, 107, 4, 237, 57, 13, 157, 153, 39, 204, 59, 155, 91, 76, 89, 209, 195,
                76, 165, 72, 165, 188, 119, 12, 210, 184, 168, 78, 56, 125, 146, 97, 253, 159, 42,
                16, 203, 73, 47, 174, 29, 163, 124, 34, 156, 218, 243, 97, 226, 65, 123, 95, 132,
                40, 158, 63, 255, 94, 39, 196, 45, 251, 145, 188, 37, 155, 16, 201, 208, 50, 33,
                199, 98, 119, 172, 71, 240, 191, 110, 243, 225, 180, 215, 97, 98, 252, 124, 220,
                169, 163, 130, 43, 114, 242, 40, 46, 60, 6, 5, 51, 186, 24, 73, 62, 221, 213, 61,
                116, 62, 159, 150, 165, 183, 78, 86, 26, 236, 214, 9, 54, 152, 13, 135, 124, 137,
                89, 119, 212, 15, 212, 24, 181, 54, 115, 197, 150, 31, 22, 150, 210, 187, 28, 94,
                109, 138, 22, 234, 67, 58, 115, 199, 93, 121, 182, 221, 62, 212, 88, 84, 103, 215,
                109, 154,
            ];

            let deserialized_proof = deserialize_proof::<Bn254>(raw_proof.clone());
            let circom_proof = CircomProof {
                a: CircomProverG1::from_bn254(&deserialized_proof.0.a).into(),
                b: CircomProverG2::from_bn254(&deserialized_proof.0.b).into(),
                c: CircomProverG1::from_bn254(&deserialized_proof.0.c).into(),
                protocol: PROTOCOL_GROTH16.to_string(),
                curve: CURVE_BN254.to_string(),
            };

            let proof = to_ethereum_proof(circom_proof.clone());
            assert!(!proof.a.x.is_empty());
            assert!(!proof.a.y.is_empty());
            assert!(!proof.b.x.is_empty());
            assert!(!proof.b.y.is_empty());
            assert!(!proof.c.x.is_empty());
            assert!(!proof.c.y.is_empty());

            let converted_proof: CircomProverProof = from_ethereum_proof(proof).into();
            let serialized_proof =
                serialize_proof::<Bn254>(&SerializableProof(converted_proof.into()));
            assert_eq!(raw_proof, serialized_proof);
        }

        #[test]
        fn test_to_ethereum_inputs() {
            let raw_inputs = vec![
                2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 240, 147, 245, 225, 67, 145, 112, 185, 121, 72,
                232, 51, 40, 93, 88, 129, 129, 182, 69, 80, 184, 41, 160, 49, 225, 114, 78, 100,
                48, 0, 0, 0, 240, 147, 245, 225, 67, 145, 112, 185, 121, 72, 232, 51, 40, 93, 88,
                129, 129, 182, 69, 80, 184, 41, 160, 49, 225, 114, 78, 100, 48,
            ];
            let inputs = to_ethereum_inputs(raw_inputs.clone());
            let expected_inputs = vec![
                "21888242871839275222246405745257275088548364400416034343698204186575808495616",
                "21888242871839275222246405745257275088548364400416034343698204186575808495616",
            ];
            assert_eq!(inputs, expected_inputs);

            let converted_inputs = from_ethereum_inputs(inputs);
            assert_eq!(raw_inputs, converted_inputs);
        }

        #[test]
        fn test_to_ethereum_inputs_with_zero() {
            let raw_inputs = vec![
                1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ];
            let inputs = to_ethereum_inputs(raw_inputs.clone());
            let expected_inputs = vec!["0".to_string()];
            assert_eq!(inputs, expected_inputs);

            let converted_inputs = from_ethereum_inputs(expected_inputs);
            assert_eq!(raw_inputs, converted_inputs);
        }
    }
}
