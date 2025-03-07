// This file is copied from https://github.dev/zkmopro/circom-compat/tree/wasm-delete

//! Helpers for converting Arkworks types to BigUint-tuples as expected by the
//! Solidity Groth16 Verifier smart contracts
use ark_bls12_381::{
    Bls12_381, Fq as bls12_381_fq, Fq2 as bls12_381_Fq2, G1Affine as bls12_381_G1Affine,
    G2Affine as bls12_381_G2Affine,
};
use ark_ff::{BigInteger, PrimeField};
use num::BigUint;
use num_traits::Zero;

use ark_bn254::{
    Bn254, Fq as bn254_Fq, Fq2 as bn254_Fq2, Fr, G1Affine as bn254_G1Affine,
    G2Affine as bn254_G2Affine,
};
use ark_serialize::CanonicalDeserialize;

const BN254_BUF_SIZE: usize = 32;
const BLS12_381_BUF_SIZE: usize = 48;
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

type G1Tup = (BigUint, BigUint);

impl G1 {
    pub fn as_tuple(&self) -> (BigUint, BigUint) {
        (self.x.clone(), self.y.clone())
    }

    // BN254
    pub fn to_bn254(self) -> bn254_G1Affine {
        let x: bn254_Fq = biguint_to_point(self.x, BN254_BUF_SIZE);
        let y: bn254_Fq = biguint_to_point(self.y, BN254_BUF_SIZE);
        if x.is_zero() && y.is_zero() {
            bn254_G1Affine::identity()
        } else {
            bn254_G1Affine::new(x, y)
        }
    }

    pub fn from_bn254(p: &bn254_G1Affine) -> Self {
        Self {
            x: point_to_biguint(p.x),
            y: point_to_biguint(p.y),
        }
    }

    // BLS12-381
    pub fn to_bls12_381(self) -> bls12_381_G1Affine {
        let x: bls12_381_fq = biguint_to_point(self.x, BLS12_381_BUF_SIZE);
        let y: bls12_381_fq = biguint_to_point(self.y, BLS12_381_BUF_SIZE);
        if x.is_zero() && y.is_zero() {
            bls12_381_G1Affine::identity()
        } else {
            bls12_381_G1Affine::new(x, y)
        }
    }

    pub fn from_bls12_381(p: &bls12_381_G1Affine) -> Self {
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

type G2Tup = ([BigUint; 2], [BigUint; 2]);

impl G2 {
    // NB: Serialize the c1 limb first.
    pub fn as_tuple(&self) -> G2Tup {
        (
            [self.x[1].clone(), self.x[0].clone()],
            [self.y[1].clone(), self.y[0].clone()],
        )
    }

    // BN254
    pub fn to_bn254(self) -> bn254_G2Affine {
        let c0 = biguint_to_point(self.x[0].clone(), BN254_BUF_SIZE);
        let c1 = biguint_to_point(self.x[1].clone(), BN254_BUF_SIZE);
        let x = bn254_Fq2::new(c0, c1);

        let c0 = biguint_to_point(self.y[0].clone(), BN254_BUF_SIZE);
        let c1 = biguint_to_point(self.y[1].clone(), BN254_BUF_SIZE);
        let y = bn254_Fq2::new(c0, c1);

        if x.is_zero() && y.is_zero() {
            bn254_G2Affine::identity()
        } else {
            bn254_G2Affine::new(x, y)
        }
    }

    pub fn from_bn254(p: &bn254_G2Affine) -> Self {
        Self {
            x: [point_to_biguint(p.x.c0), point_to_biguint(p.x.c1)],
            y: [point_to_biguint(p.y.c0), point_to_biguint(p.y.c1)],
        }
    }

    // BLS12-381
    pub fn to_bls12_381(self) -> bls12_381_G2Affine {
        let c0 = biguint_to_point(self.x[0].clone(), BLS12_381_BUF_SIZE);
        let c1 = biguint_to_point(self.x[1].clone(), BLS12_381_BUF_SIZE);
        let x = bls12_381_Fq2::new(c0, c1);

        let c0 = biguint_to_point(self.y[0].clone(), BLS12_381_BUF_SIZE);
        let c1 = biguint_to_point(self.y[1].clone(), BLS12_381_BUF_SIZE);
        let y = bls12_381_Fq2::new(c0, c1);

        if x.is_zero() && y.is_zero() {
            bls12_381_G2Affine::identity()
        } else {
            bls12_381_G2Affine::new(x, y)
        }
    }

    pub fn from_bls12_381(p: &bls12_381_G2Affine) -> Self {
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
            a: G1::from_bn254(&proof.a),
            b: G2::from_bn254(&proof.b),
            c: G1::from_bn254(&proof.c),
        }
    }
}

impl From<ark_groth16::Proof<Bls12_381>> for Proof {
    fn from(proof: ark_groth16::Proof<Bls12_381>) -> Self {
        Self {
            a: G1::from_bls12_381(&proof.a),
            b: G2::from_bls12_381(&proof.b),
            c: G1::from_bls12_381(&proof.c),
        }
    }
}

impl From<Proof> for ark_groth16::Proof<Bn254> {
    fn from(src: Proof) -> ark_groth16::Proof<Bn254> {
        ark_groth16::Proof {
            a: src.a.to_bn254(),
            b: src.b.to_bn254(),
            c: src.c.to_bn254(),
        }
    }
}

impl From<Proof> for ark_groth16::Proof<Bls12_381> {
    fn from(src: Proof) -> ark_groth16::Proof<Bls12_381> {
        ark_groth16::Proof {
            a: src.a.to_bls12_381(),
            b: src.b.to_bls12_381(),
            c: src.c.to_bls12_381(),
        }
    }
}

// Helper for converting a PrimeField to its BigUint representation for Ethereum compatibility
fn biguint_to_point<F: PrimeField>(point: BigUint, buf_size: usize) -> F {
    let mut buf = point.to_bytes_le();
    buf.resize(buf_size, 0u8);
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
    use crate::prover::ethereum::{
        biguint_to_point, point_to_biguint, Proof, BLS12_381_BUF_SIZE, BN254_BUF_SIZE, G1, G2,
    };

    mod bn254 {
        use super::*;
        use ark_bn254::{Bn254, Fq, Fr, G1Affine, G2Affine};
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
            let el3: Fq = biguint_to_point(el2.clone(), BN254_BUF_SIZE);
            let el4 = point_to_biguint(el3);
            assert_eq!(el, el3);
            assert_eq!(el2, el4);
        }

        #[test]
        fn convert_fr() {
            let el = fr();
            let el2 = point_to_biguint(el);
            let el3: Fr = biguint_to_point(el2.clone(), BN254_BUF_SIZE);
            let el4 = point_to_biguint(el3);
            assert_eq!(el, el3);
            assert_eq!(el2, el4);
        }

        #[test]
        fn convert_g1() {
            let el = g1();
            let el2 = G1::from_bn254(&el);
            let el3: G1Affine = el2.clone().to_bn254();
            let el4 = G1::from_bn254(&el3);
            assert_eq!(el, el3);
            assert_eq!(el2, el4);
        }

        #[test]
        fn convert_g2() {
            let el = g2();
            let el2 = G2::from_bn254(&el);
            let el3: G2Affine = el2.clone().to_bn254();
            let el4 = G2::from_bn254(&el3);
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

    mod bls12_381 {
        use super::*;
        use ark_bls12_381::{Bls12_381, Fq, Fr, G1Affine, G2Affine};
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
            let el3: Fq = biguint_to_point(el2.clone(), BLS12_381_BUF_SIZE);
            let el4 = point_to_biguint(el3);
            assert_eq!(el, el3);
            assert_eq!(el2, el4);
        }

        #[test]
        fn convert_fr() {
            let el = fr();
            let el2 = point_to_biguint(el);
            let el3: Fr = biguint_to_point(el2.clone(), BLS12_381_BUF_SIZE);
            let el4 = point_to_biguint(el3);
            assert_eq!(el, el3);
            assert_eq!(el2, el4);
        }

        #[test]
        fn convert_g1() {
            let el = g1();
            let el2 = G1::from_bls12_381(&el);
            let el3: G1Affine = el2.clone().to_bls12_381();
            let el4 = G1::from_bls12_381(&el3);
            assert_eq!(el, el3);
            assert_eq!(el2, el4);
        }

        #[test]
        fn convert_g2() {
            let el = g2();
            let el2 = G2::from_bls12_381(&el);
            let el3: G2Affine = el2.clone().to_bls12_381();
            let el4 = G2::from_bls12_381(&el3);
            assert_eq!(el, el3);
            assert_eq!(el2, el4);
        }

        #[test]
        fn convert_proof() {
            let p = ark_groth16::Proof::<Bls12_381> {
                a: g1(),
                b: g2(),
                c: g1(),
            };
            let p2 = Proof::from(p.clone());
            let p3 = ark_groth16::Proof::from(p2);
            assert_eq!(p, p3);
        }
    }
}
