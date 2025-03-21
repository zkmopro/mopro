use crate::{CircomProof, G1, G2};
use anyhow::{bail, Result};
use ark_bls12_381::{
    Bls12_381, Fq as Bls12_381_Fq, Fq2 as Bls12_381_Fq2, G1Affine as Bls12_381_G1Affine,
    G2Affine as Bls12_381_G2Affine,
};
use ark_bn254::{
    Bn254, Fq as Bn254_Fq, Fq2 as Bn254_Fq2, G1Affine as Bn254_G1Affine, G2Affine as Bn254_G2Affine,
};
use circom_prover::prover::{
    circom::{self, CURVE_BLS12_381, CURVE_BN254, PROTOCOL_GROTH16},
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

// Convert CircomProof to ProofCalldata as expected by the Solidity Groth16 Verifier
pub fn to_ethereum_proof(proof: CircomProof) -> Result<ProofCalldata> {
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
    Ok(ProofCalldata { a, b, c })
}

pub fn to_ethereum_inputs(inputs: Vec<u8>, curve: String) -> Result<Vec<String>> {
    let inputs = match curve.as_ref() {
        CURVE_BN254 => {
            let deserialized_inputs = deserialize_inputs::<Bn254>(inputs);
            circom::Inputs::from(deserialized_inputs.0.as_slice())
        }
        CURVE_BLS12_381 => {
            let deserialized_inputs = deserialize_inputs::<Bls12_381>(inputs);
            circom::Inputs::from(deserialized_inputs.0.as_slice())
        }
        _ => bail!("Not uspported curve"),
    };
    Ok(inputs.0.iter().map(|x| x.to_string()).collect())
}

pub fn from_ethereum_inputs(inputs: Vec<String>, curve: String) -> Result<Vec<u8>> {
    let inputs = inputs
        .iter()
        .map(|x| BigUint::from_str(x).unwrap())
        .collect::<Vec<BigUint>>();

    match curve.as_ref() {
        CURVE_BN254 => {
            let serializable_inputs = SerializableInputs::<Bn254>(circom::Inputs(inputs).into());
            Ok(serialization::serialize_inputs(&serializable_inputs))
        }
        CURVE_BLS12_381 => {
            let serializable_inputs =
                SerializableInputs::<Bls12_381>(circom::Inputs(inputs).into());
            Ok(serialization::serialize_inputs(&serializable_inputs))
        }
        _ => bail!("Not uspported curve"),
    }
}

// Convert ProofCalldata to CircomProof
pub fn from_ethereum_proof(proof: ProofCalldata, curve: String) -> Result<CircomProof> {
    match curve.as_ref() {
        CURVE_BN254 => {
            let a = Bn254_G1Affine::new_unchecked(
                Bn254_Fq::from_str(&proof.a.x).unwrap(),
                Bn254_Fq::from_str(&proof.a.y).unwrap(),
            );
            let c = Bn254_G1Affine::new_unchecked(
                Bn254_Fq::from_str(&proof.c.x).unwrap(),
                Bn254_Fq::from_str(&proof.c.y).unwrap(),
            );
            let b1_x = Bn254_Fq::from_str(&proof.b.x[0]).unwrap();
            let b1_y = Bn254_Fq::from_str(&proof.b.x[1]).unwrap();
            let b2_x = Bn254_Fq::from_str(&proof.b.y[0]).unwrap();
            let b2_y = Bn254_Fq::from_str(&proof.b.y[1]).unwrap();
            let b = Bn254_G2Affine::new_unchecked(
                Bn254_Fq2::new(b1_x, b1_y),
                Bn254_Fq2::new(b2_x, b2_y),
            );
            Ok(CircomProof {
                a: circom::G1::from_bn254(&a).into(),
                b: circom::G2::from_bn254(&b).into(),
                c: circom::G1::from_bn254(&c).into(),
                protocol: PROTOCOL_GROTH16.to_string(),
                curve: CURVE_BN254.to_string(),
            })
        }
        CURVE_BLS12_381 => {
            let a = Bls12_381_G1Affine::new_unchecked(
                Bls12_381_Fq::from_str(&proof.a.x).unwrap(),
                Bls12_381_Fq::from_str(&proof.a.y).unwrap(),
            );
            let c = Bls12_381_G1Affine::new_unchecked(
                Bls12_381_Fq::from_str(&proof.c.x).unwrap(),
                Bls12_381_Fq::from_str(&proof.c.y).unwrap(),
            );
            let b1_x = Bls12_381_Fq::from_str(&proof.b.x[0]).unwrap();
            let b1_y = Bls12_381_Fq::from_str(&proof.b.x[1]).unwrap();
            let b2_x = Bls12_381_Fq::from_str(&proof.b.y[0]).unwrap();
            let b2_y = Bls12_381_Fq::from_str(&proof.b.y[1]).unwrap();
            let b = Bls12_381_G2Affine::new_unchecked(
                Bls12_381_Fq2::new(b1_x, b1_y),
                Bls12_381_Fq2::new(b2_x, b2_y),
            );
            Ok(CircomProof {
                a: circom::G1::from_bls12_381(&a).into(),
                b: circom::G2::from_bls12_381(&b).into(),
                c: circom::G1::from_bls12_381(&c).into(),
                protocol: PROTOCOL_GROTH16.to_string(),
                curve: CURVE_BLS12_381.to_string(),
            })
        }
        _ => bail!("Not uspported curve"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use circom_prover::prover::{
        circom::{
            Proof as CircomProverProof, CURVE_BN254, G1 as CircomProverG1, G2 as CircomProverG2,
            PROTOCOL_GROTH16,
        },
        serialization::{deserialize_proof, serialize_proof, SerializableProof},
    };

    #[test]
    fn test_not_supported_curve() {
        let raw_proof = vec![
            22, 2, 28, 144, 134, 93, 1, 6, 180, 134, 137, 11, 130, 217, 116, 35, 22, 58, 213, 215,
            39, 9, 76, 99, 93, 46, 166, 183, 200, 20, 234, 26, 86, 182, 126, 104, 167, 218, 67,
            106, 232, 177, 113, 76, 217, 110, 167, 101, 215, 168, 67, 23, 2, 2, 50, 131, 103, 159,
            241, 197, 1, 75, 72, 154, 107, 226, 61, 6, 227, 5, 193, 103, 229, 40, 232, 183, 170,
            218, 136, 73, 194, 166, 135, 22, 128, 83, 94, 84, 179, 66, 38, 17, 200, 0, 107, 4, 237,
            57, 13, 157, 153, 39, 204, 59, 155, 91, 76, 89, 209, 195, 76, 165, 72, 165, 188, 119,
            12, 210, 184, 168, 78, 56, 125, 146, 97, 253, 159, 42, 16, 203, 73, 47, 174, 29, 163,
            124, 34, 156, 218, 243, 97, 226, 65, 123, 95, 132, 40, 158, 63, 255, 94, 39, 196, 45,
            251, 145, 188, 37, 155, 16, 201, 208, 50, 33, 199, 98, 119, 172, 71, 240, 191, 110,
            243, 225, 180, 215, 97, 98, 252, 124, 220, 169, 163, 130, 43, 114, 242, 40, 46, 60, 6,
            5, 51, 186, 24, 73, 62, 221, 213, 61, 116, 62, 159, 150, 165, 183, 78, 86, 26, 236,
            214, 9, 54, 152, 13, 135, 124, 137, 89, 119, 212, 15, 212, 24, 181, 54, 115, 197, 150,
            31, 22, 150, 210, 187, 28, 94, 109, 138, 22, 234, 67, 58, 115, 199, 93, 121, 182, 221,
            62, 212, 88, 84, 103, 215, 109, 154,
        ];
        let deserialized_proof = deserialize_proof::<Bn254>(raw_proof.clone());
        let p: CircomProverProof = deserialized_proof.0.into();
        let proof: ProofCalldata = to_ethereum_proof(p.into()).unwrap();
        let result = from_ethereum_proof(proof, "bls12377".to_string());
        assert!(result.is_err());

        let raw_inputs = vec![
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        let result = to_ethereum_inputs(raw_inputs.clone(), "bls12377".to_string());
        assert!(result.is_err());

        let result = from_ethereum_inputs(vec!["1".to_string()], "bls12377".to_string());
        assert!(result.is_err());
    }

    mod bn254 {
        use super::*;

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

            let proof = to_ethereum_proof(circom_proof.clone()).unwrap();
            assert!(!proof.a.x.is_empty());
            assert!(!proof.a.y.is_empty());
            assert!(!proof.b.x.is_empty());
            assert!(!proof.b.y.is_empty());
            assert!(!proof.c.x.is_empty());
            assert!(!proof.c.y.is_empty());

            let converted_proof: CircomProverProof =
                from_ethereum_proof(proof, CURVE_BN254.to_string())
                    .unwrap()
                    .into();
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
            let inputs = to_ethereum_inputs(raw_inputs.clone(), CURVE_BN254.to_string()).unwrap();
            let expected_inputs = vec![
                "21888242871839275222246405745257275088548364400416034343698204186575808495616",
                "21888242871839275222246405745257275088548364400416034343698204186575808495616",
            ];
            assert_eq!(inputs, expected_inputs);

            let converted_inputs = from_ethereum_inputs(inputs, CURVE_BN254.to_string()).unwrap();
            assert_eq!(raw_inputs, converted_inputs);
        }

        #[test]
        fn test_to_ethereum_inputs_with_zero() {
            let raw_inputs = vec![
                1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ];
            let inputs = to_ethereum_inputs(raw_inputs.clone(), CURVE_BN254.to_string()).unwrap();
            let expected_inputs = vec!["0".to_string()];
            assert_eq!(inputs, expected_inputs);

            let converted_inputs =
                from_ethereum_inputs(expected_inputs, CURVE_BN254.to_string()).unwrap();
            assert_eq!(raw_inputs, converted_inputs);
        }
    }

    mod bls12381 {
        use super::*;

        #[test]
        fn test_to_ethereum_proof() {
            let raw_proof = vec![
                16, 28, 122, 199, 113, 31, 65, 38, 244, 34, 43, 54, 217, 128, 117, 94, 46, 251,
                124, 92, 238, 21, 116, 67, 76, 253, 25, 14, 245, 71, 165, 245, 77, 128, 31, 74,
                244, 168, 231, 74, 230, 29, 0, 205, 114, 129, 181, 0, 10, 185, 151, 173, 216, 44,
                240, 213, 252, 154, 21, 139, 128, 65, 12, 63, 186, 95, 93, 228, 224, 88, 210, 206,
                124, 0, 62, 59, 33, 172, 160, 48, 18, 229, 162, 90, 156, 80, 167, 160, 50, 94, 218,
                143, 249, 97, 83, 87, 18, 146, 222, 45, 247, 74, 121, 222, 241, 6, 178, 104, 230,
                92, 178, 152, 36, 26, 5, 250, 252, 129, 220, 63, 200, 89, 51, 109, 10, 136, 77, 38,
                73, 214, 24, 44, 248, 159, 246, 202, 68, 57, 190, 126, 154, 57, 44, 140, 23, 189,
                98, 18, 30, 4, 123, 229, 206, 30, 54, 47, 96, 142, 197, 69, 147, 171, 129, 153,
                239, 53, 185, 254, 17, 101, 204, 193, 222, 75, 106, 243, 69, 157, 201, 227, 245,
                179, 110, 227, 98, 223, 44, 15, 155, 26, 135, 89, 9, 236, 135, 147, 116, 202, 150,
                158, 72, 34, 160, 39, 140, 34, 127, 249, 86, 205, 208, 198, 45, 122, 121, 208, 114,
                191, 124, 9, 79, 158, 83, 163, 19, 240, 8, 105, 160, 120, 63, 66, 118, 52, 78, 74,
                124, 86, 83, 215, 4, 59, 74, 78, 102, 255, 112, 228, 243, 134, 174, 65, 230, 214,
                2, 205, 16, 134, 64, 198, 131, 204, 199, 224, 180, 132, 5, 107, 233, 51, 155, 131,
                184, 35, 58, 53, 203, 231, 187, 151, 172, 243, 108, 170, 223, 46, 99, 32, 12, 139,
                70, 189, 230, 72, 37, 173, 24, 159, 136, 212, 111, 126, 203, 65, 195, 186, 25, 105,
                160, 44, 154, 229, 134, 128, 183, 187, 173, 246, 240, 98, 181, 151, 124, 128, 146,
                91, 126, 203, 153, 7, 97, 123, 46, 169, 23, 99, 13, 26, 5, 75, 129, 102, 25, 29,
                223, 111, 164, 134, 237, 117, 13, 227, 152, 53, 33, 8, 232, 65, 97, 52, 71, 154,
                156, 43, 49, 34, 143, 66, 4, 9, 3, 245, 230, 175, 97, 172, 245, 106, 33, 234, 179,
                82, 146, 129,
            ];

            let deserialized_proof = deserialize_proof::<Bls12_381>(raw_proof.clone());
            let circom_proof = CircomProof {
                a: CircomProverG1::from_bls12_381(&deserialized_proof.0.a).into(),
                b: CircomProverG2::from_bls12_381(&deserialized_proof.0.b).into(),
                c: CircomProverG1::from_bls12_381(&deserialized_proof.0.c).into(),
                protocol: PROTOCOL_GROTH16.to_string(),
                curve: CURVE_BN254.to_string(),
            };

            let proof = to_ethereum_proof(circom_proof.clone()).unwrap();
            assert!(!proof.a.x.is_empty());
            assert!(!proof.a.y.is_empty());
            assert!(!proof.b.x.is_empty());
            assert!(!proof.b.y.is_empty());
            assert!(!proof.c.x.is_empty());
            assert!(!proof.c.y.is_empty());

            let converted_proof: CircomProverProof =
                from_ethereum_proof(proof, CURVE_BLS12_381.to_string())
                    .unwrap()
                    .into();
            let serialized_proof =
                serialize_proof::<Bls12_381>(&SerializableProof(converted_proof.into()));
            assert_eq!(raw_proof, serialized_proof);
        }

        #[test]
        fn test_to_ethereum_inputs() {
            let raw_inputs = vec![
                2, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ];

            let inputs =
                to_ethereum_inputs(raw_inputs.clone(), CURVE_BLS12_381.to_string()).unwrap();
            let expected_inputs = vec!["3", "11"];
            assert_eq!(inputs, expected_inputs);

            let converted_inputs =
                from_ethereum_inputs(inputs, CURVE_BLS12_381.to_string()).unwrap();
            assert_eq!(raw_inputs, converted_inputs);
        }

        #[test]
        fn test_to_ethereum_inputs_with_zero() {
            let raw_inputs = vec![
                1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ];
            let inputs =
                to_ethereum_inputs(raw_inputs.clone(), CURVE_BLS12_381.to_string()).unwrap();
            let expected_inputs = vec!["0".to_string()];
            assert_eq!(inputs, expected_inputs);

            let converted_inputs =
                from_ethereum_inputs(expected_inputs, CURVE_BLS12_381.to_string()).unwrap();
            assert_eq!(raw_inputs, converted_inputs);
        }
    }
}
