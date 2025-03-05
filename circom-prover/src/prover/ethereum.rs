// This file is copied from https://github.dev/zkmopro/circom-compat/tree/wasm-delete

//! Helpers for converting Arkworks types to BigUint-tuples as expected by the
//! Solidity Groth16 Verifier smart contracts
use ark_ff::{BigInteger, PrimeField};
use num::BigUint;
use num_traits::Zero;

use ark_bn254::{Bn254, Fq, Fq2, Fr, G1Affine, G2Affine};
use ark_serialize::CanonicalDeserialize;

pub struct Inputs(pub Vec<BigUint>);

impl From<&[Fr]> for Inputs {
    fn from(src: &[Fr]) -> Self {
        let els = src.iter().map(|point| point_to_biguint(*point)).collect();

        Self(els)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct G1 {
    pub x: BigUint,
    pub y: BigUint,
}

impl From<G1> for G1Affine {
    fn from(src: G1) -> G1Affine {
        let x: Fq = biguint_to_point(src.x);
        let y: Fq = biguint_to_point(src.y);
        if x.is_zero() && y.is_zero() {
            G1Affine::identity()
        } else {
            G1Affine::new(x, y)
        }
    }
}

type G1Tup = (BigUint, BigUint);

impl G1 {
    pub fn as_tuple(&self) -> (BigUint, BigUint) {
        (self.x.clone(), self.y.clone())
    }
}

impl From<&G1Affine> for G1 {
    fn from(p: &G1Affine) -> Self {
        Self {
            x: point_to_biguint(p.x),
            y: point_to_biguint(p.y),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct G2 {
    pub x: [BigUint; 2],
    pub y: [BigUint; 2],
}

impl From<G2> for G2Affine {
    fn from(src: G2) -> G2Affine {
        let c0 = biguint_to_point(src.x[0].clone());
        let c1 = biguint_to_point(src.x[1].clone());
        let x = Fq2::new(c0, c1);

        let c0 = biguint_to_point(src.y[0].clone());
        let c1 = biguint_to_point(src.y[1].clone());
        let y = Fq2::new(c0, c1);

        if x.is_zero() && y.is_zero() {
            G2Affine::identity()
        } else {
            G2Affine::new(x, y)
        }
    }
}

type G2Tup = ([BigUint; 2], [BigUint; 2]);

impl G2 {
    // NB: Serialize the c1 limb first.
    pub fn as_tuple(&self) -> G2Tup {
        (
            [self.x[1].clone(), self.x[0].clone()],
            [self.y[1].clone(), self.y[0].clone()],
        )
    }
}

impl From<&G2Affine> for G2 {
    fn from(p: &G2Affine) -> Self {
        Self {
            x: [point_to_biguint(p.x.c0), point_to_biguint(p.x.c1)],
            y: [point_to_biguint(p.y.c0), point_to_biguint(p.y.c1)],
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
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

// Helper for converting a PrimeField to its BigUint representation for Ethereum compatibility
fn biguint_to_point<F: PrimeField>(point: BigUint) -> F {
    let mut buf = point.to_bytes_le();
    buf.resize(32, 0u8);
    let bigint = F::BigInt::deserialize_uncompressed(&buf[..]).expect("always works");
    F::from_bigint(bigint).expect("always works")
}

// Helper for converting a PrimeField to its BigUint representation for Ethereum compatibility
// (BigUint reads data as big endian)
fn point_to_biguint<F: PrimeField>(point: F) -> BigUint {
    let point = point.into_bigint();
    let point_bytes = point.to_bytes_be();
    BigUint::from_bytes_be(&point_bytes[..])
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_bn254::Fq;
    use ark_std::UniformRand;

    fn fq() -> Fq {
        Fq::from(2)
    }

    fn fr() -> Fr {
        Fr::from(2)
    }

    fn g1() -> G1Affine {
        let rng = &mut ark_std::test_rng();
        G1Affine::rand(rng)
    }

    fn g2() -> G2Affine {
        let rng = &mut ark_std::test_rng();
        G2Affine::rand(rng)
    }

    #[test]
    fn convert_fq() {
        let el = fq();
        let el2 = point_to_biguint(el);
        let el3: Fq = biguint_to_point(el2.clone());
        let el4 = point_to_biguint(el3);
        assert_eq!(el, el3);
        assert_eq!(el2, el4);
    }

    #[test]
    fn convert_fr() {
        let el = fr();
        let el2 = point_to_biguint(el);
        let el3: Fr = biguint_to_point(el2.clone());
        let el4 = point_to_biguint(el3);
        assert_eq!(el, el3);
        assert_eq!(el2, el4);
    }

    #[test]
    fn convert_g1() {
        let el = g1();
        let el2 = G1::from(&el);
        let el3: G1Affine = el2.clone().into();
        let el4 = G1::from(&el3);
        assert_eq!(el, el3);
        assert_eq!(el2, el4);
    }

    #[test]
    fn convert_g2() {
        let el = g2();
        let el2 = G2::from(&el);
        let el3: G2Affine = el2.clone().into();
        let el4 = G2::from(&el3);
        assert_eq!(el, el3);
        assert_eq!(el2, el4);
    }

    #[test]
    fn convert_proof() {
        let p = ark_groth16::Proof::<Bn254> {
            a: g1(),
            b: g2(),
            c: g1(),
        };
        let p2 = Proof::from(p.clone());
        let p3 = ark_groth16::Proof::from(p2);
        assert_eq!(p, p3);
    }
}
