// copied from: https://github.com/arkworks-rs/circom-compat/blob/967add46da8ece5216f1838233043ccc9c511330/src/ethereum.rs
//! Helpers for converting Arkworks types to U256-tuples as expected by the
//! Solidity Groth16 Verifier smart contracts
use ark_ff::{BigInteger, PrimeField};
use ethers_core::types::U256;
use num_traits::Zero;

use ark_bn254::{Bn254, Fq, Fq2, Fr, G1Affine, G2Affine};
use ark_serialize::CanonicalDeserialize;

pub struct Inputs(pub Vec<U256>);

impl From<&[Fr]> for Inputs {
    fn from(src: &[Fr]) -> Self {
        let els = src.iter().map(|point| point_to_u256(*point)).collect();

        Self(els)
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct G1 {
    pub x: U256,
    pub y: U256,
}

impl From<G1> for G1Affine {
    fn from(src: G1) -> G1Affine {
        let x: Fq = u256_to_point(src.x);
        let y: Fq = u256_to_point(src.y);
        if x.is_zero() && y.is_zero() {
            G1Affine::identity()
        } else {
            G1Affine::new(x, y)
        }
    }
}

type G1Tup = (U256, U256);

impl G1 {
    pub fn as_tuple(&self) -> (U256, U256) {
        (self.x, self.y)
    }
}

impl From<&G1Affine> for G1 {
    fn from(p: &G1Affine) -> Self {
        Self {
            x: point_to_u256(p.x),
            y: point_to_u256(p.y),
        }
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct G2 {
    pub x: [U256; 2],
    pub y: [U256; 2],
}

impl From<G2> for G2Affine {
    fn from(src: G2) -> G2Affine {
        let c0 = u256_to_point(src.x[0]);
        let c1 = u256_to_point(src.x[1]);
        let x = Fq2::new(c0, c1);

        let c0 = u256_to_point(src.y[0]);
        let c1 = u256_to_point(src.y[1]);
        let y = Fq2::new(c0, c1);

        if x.is_zero() && y.is_zero() {
            G2Affine::identity()
        } else {
            G2Affine::new(x, y)
        }
    }
}

type G2Tup = ([U256; 2], [U256; 2]);

impl G2 {
    // NB: Serialize the c1 limb first.
    pub fn as_tuple(&self) -> G2Tup {
        ([self.x[1], self.x[0]], [self.y[1], self.y[0]])
    }
}

impl From<&G2Affine> for G2 {
    fn from(p: &G2Affine) -> Self {
        Self {
            x: [point_to_u256(p.x.c0), point_to_u256(p.x.c1)],
            y: [point_to_u256(p.y.c0), point_to_u256(p.y.c1)],
        }
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Proof {
    pub a: G1,
    pub b: G2,
    pub c: G1,
}

impl Proof {
    pub fn as_tuple(&self) -> (G1Tup, G2Tup, G1Tup) {
        (self.a.as_tuple(), self.b.as_tuple(), self.c.as_tuple())
    }
}

impl From<ark_groth16::Proof<Bn254>> for Proof {
    fn from(proof: ark_groth16::Proof<Bn254>) -> Self {
        Self {
            a: G1::from(&proof.a),
            b: G2::from(&proof.b),
            c: G1::from(&proof.c),
        }
    }
}

impl From<Proof> for ark_groth16::Proof<Bn254> {
    fn from(src: Proof) -> ark_groth16::Proof<Bn254> {
        ark_groth16::Proof {
            a: src.a.into(),
            b: src.b.into(),
            c: src.c.into(),
        }
    }
}

#[derive(Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct VerifyingKey {
    pub alpha1: G1,
    pub beta2: G2,
    pub gamma2: G2,
    pub delta2: G2,
    pub ic: Vec<G1>,
}

impl VerifyingKey {
    pub fn as_tuple(&self) -> (G1Tup, G2Tup, G2Tup, G2Tup, Vec<G1Tup>) {
        (
            self.alpha1.as_tuple(),
            self.beta2.as_tuple(),
            self.gamma2.as_tuple(),
            self.delta2.as_tuple(),
            self.ic.iter().map(|i| i.as_tuple()).collect(),
        )
    }
}

impl From<ark_groth16::VerifyingKey<Bn254>> for VerifyingKey {
    fn from(vk: ark_groth16::VerifyingKey<Bn254>) -> Self {
        Self {
            alpha1: G1::from(&vk.alpha_g1),
            beta2: G2::from(&vk.beta_g2),
            gamma2: G2::from(&vk.gamma_g2),
            delta2: G2::from(&vk.delta_g2),
            ic: vk.gamma_abc_g1.iter().map(G1::from).collect(),
        }
    }
}

impl From<VerifyingKey> for ark_groth16::VerifyingKey<Bn254> {
    fn from(src: VerifyingKey) -> ark_groth16::VerifyingKey<Bn254> {
        ark_groth16::VerifyingKey {
            alpha_g1: src.alpha1.into(),
            beta_g2: src.beta2.into(),
            gamma_g2: src.gamma2.into(),
            delta_g2: src.delta2.into(),
            gamma_abc_g1: src.ic.into_iter().map(Into::into).collect(),
        }
    }
}

// Helper for converting a PrimeField to its U256 representation for Ethereum compatibility
fn u256_to_point<F: PrimeField>(point: U256) -> F {
    let mut buf = [0; 32];
    point.to_little_endian(&mut buf);
    let bigint = F::BigInt::deserialize_uncompressed(&buf[..]).expect("always works");
    F::from_bigint(bigint).expect("always works")
}

// Helper for converting a PrimeField to its U256 representation for Ethereum compatibility
// (U256 reads data as big endian)
fn point_to_u256<F: PrimeField>(point: F) -> U256 {
    let point = point.into_bigint();
    let point_bytes = point.to_bytes_be();
    U256::from(&point_bytes[..])
}