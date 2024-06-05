use ark_bn254::{Fq, FqConfig};
use ark_ff::biginteger::{BigInteger, BigInteger256};

use crate::middleware::gpu_explorations::metal::abstraction::mont_reduction;

// implement to_u32_limbs and from_u32_limbs for BigInt<4>
pub trait ToLimbs {
    fn to_u32_limbs(&self) -> Vec<u32>;
}

pub trait FromLimbs {
    fn from_u32_limbs(limbs: &[u32]) -> Self;
    fn from_u128(num: u128) -> Self;
    fn from_u32(num: u32) -> Self;
}

// convert from little endian to big endian
impl ToLimbs for BigInteger256 {
    fn to_u32_limbs(&self) -> Vec<u32> {
        let mut limbs = Vec::new();
        self.to_bytes_be().chunks(8).for_each(|chunk| {
            let high = u32::from_be_bytes(chunk[0..4].try_into().unwrap());
            let low = u32::from_be_bytes(chunk[4..8].try_into().unwrap());
            limbs.push(high);
            limbs.push(low);
        });
        limbs
    }
}

// convert from little endian to big endian
impl ToLimbs for Fq {
    fn to_u32_limbs(&self) -> Vec<u32> {
        let mut limbs = Vec::new();
        self.0.to_bytes_be().chunks(8).for_each(|chunk| {
            let high = u32::from_be_bytes(chunk[0..4].try_into().unwrap());
            let low = u32::from_be_bytes(chunk[4..8].try_into().unwrap());
            limbs.push(high);
            limbs.push(low);
        });
        limbs
    }
}

impl FromLimbs for BigInteger256 {
    // convert from big endian to little endian for metal
    fn from_u32_limbs(limbs: &[u32]) -> Self {
        let mut big_int = [0u64; 4];
        for (i, limb) in limbs.chunks(2).rev().enumerate() {
            let high = u64::from(limb[0]);
            let low = u64::from(limb[1]);
            big_int[i] = (high << 32) | low;
        }
        BigInteger256::new(big_int)
    }
    // provide little endian u128 since arkworks use this value as well
    fn from_u128(num: u128) -> Self {
        let high = (num >> 64) as u64;
        let low = num as u64;
        BigInteger256::new([low, high, 0, 0])
    }
    // provide little endian u32 since arkworks use this value as well
    fn from_u32(num: u32) -> Self {
        BigInteger256::new([num as u64, 0, 0, 0])
    }
}

impl FromLimbs for Fq {
    // convert from big endian to little endian for metal
    fn from_u32_limbs(limbs: &[u32]) -> Self {
        let mut big_int = [0u64; 4];
        for (i, limb) in limbs.chunks(2).rev().enumerate() {
            let high = u64::from(limb[0]);
            let low = u64::from(limb[1]);
            big_int[i] = (high << 32) | low;
        }
        Fq::new(mont_reduction::raw_reduction(BigInteger256::new(big_int)))
    }
    fn from_u128(num: u128) -> Self {
        let high = (num >> 64) as u64;
        let low = num as u64;
        Fq::new(mont_reduction::raw_reduction(BigInteger256::new([
            low, high, 0, 0,
        ])))
    }
    fn from_u32(num: u32) -> Self {
        Fq::new(mont_reduction::raw_reduction(BigInteger256::new([
            num as u64, 0, 0, 0,
        ])))
    }
}
