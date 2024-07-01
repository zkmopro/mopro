use halo2_proofs::halo2curves::ff::{FromUniformBytes, PrimeField};
use num_bigint::BigUint;
use std::hash::Hash;

/// Helper trait to represent a field element that can be converted into [u64] limbs.
///
/// Note: Since the number of bits necessary to represent a field element is larger than the number of bits in a u64, we decompose the integer representation of the field element into multiple [u64] values e.g. `limbs`.
pub trait ScalarField: PrimeField + FromUniformBytes<64> + From<bool> + Hash + Ord {
    /// Returns the base `2<sup>bit_len</sup>` little endian representation of the [ScalarField] element up to `num_limbs` number of limbs (truncates any extra limbs).
    ///
    /// Assumes `bit_len < 64`.
    /// * `num_limbs`: number of limbs to return
    /// * `bit_len`: number of bits in each limb
    fn to_u64_limbs(self, num_limbs: usize, bit_len: usize) -> Vec<u64>;

    /// Returns the little endian byte representation of the element.
    fn to_bytes_le(&self) -> Vec<u8> {
        self.to_repr().as_ref().to_vec()
    }

    /// Creates a field element from a little endian byte representation.
    ///
    /// The default implementation assumes that `PrimeField::from_repr` is implemented for little-endian.
    /// It should be overridden if this is not the case.
    fn from_bytes_le(bytes: &[u8]) -> Self {
        let mut repr = Self::Repr::default();
        repr.as_mut()[..bytes.len()].copy_from_slice(bytes);
        Self::from_repr(repr).unwrap()
    }

    /// Gets the least significant 32 bits of the field element.
    fn get_lower_32(&self) -> u32 {
        let bytes = self.to_bytes_le();
        let mut lower_32 = 0u32;
        for (i, byte) in bytes.into_iter().enumerate().take(4) {
            lower_32 |= (byte as u32) << (i * 8);
        }
        lower_32
    }

    /// Gets the least significant 64 bits of the field element.
    fn get_lower_64(&self) -> u64 {
        let bytes = self.to_bytes_le();
        let mut lower_64 = 0u64;
        for (i, byte) in bytes.into_iter().enumerate().take(8) {
            lower_64 |= (byte as u64) << (i * 8);
        }
        lower_64
    }
}
// See below for implementations

/// Converts an [Iterator] of u64 digits into `number_of_limbs` limbs of `bit_len` bits returned as a [Vec].
///
/// Assumes: `bit_len < 64`.
/// * `e`: Iterator of [u64] digits
/// * `number_of_limbs`: number of limbs to return
/// * `bit_len`: number of bits in each limb
#[inline(always)]
pub(crate) fn decompose_u64_digits_to_limbs(
    e: impl IntoIterator<Item = u64>,
    number_of_limbs: usize,
    bit_len: usize,
) -> Vec<u64> {
    debug_assert!(bit_len < 64);

    let mut e = e.into_iter();
    // Mask to extract the bits from each digit
    let mask: u64 = (1u64 << bit_len) - 1u64;
    let mut u64_digit = e.next().unwrap_or(0);
    let mut rem = 64;

    // For each digit, we extract its individual limbs by repeatedly masking and shifting the digit based on how many bits we have left to extract.
    (0..number_of_limbs)
        .map(|_| match rem.cmp(&bit_len) {
            // If `rem` > `bit_len`, we mask the bits from the `u64_digit` to return the first limb.
            // We shift the digit to the right by `bit_len` bits and subtract `bit_len` from `rem`
            core::cmp::Ordering::Greater => {
                let limb = u64_digit & mask;
                u64_digit >>= bit_len;
                rem -= bit_len;
                limb
            }
            // If `rem` == `bit_len`, then we mask the bits from the `u64_digit` to return the first limb
            // We retrieve the next digit and reset `rem` to 64
            core::cmp::Ordering::Equal => {
                let limb = u64_digit & mask;
                u64_digit = e.next().unwrap_or(0);
                rem = 64;
                limb
            }
            // If `rem` < `bit_len`, we retrieve the next digit, mask it, and shift left `rem` bits from the `u64_digit` to return the first limb.
            // we shift the digit to the right by `bit_len` - `rem` bits to retrieve the start of the next limb and add 64 - bit_len to `rem` to get the remainder.
            core::cmp::Ordering::Less => {
                let mut limb = u64_digit;
                u64_digit = e.next().unwrap_or(0);
                limb |= (u64_digit & ((1u64 << (bit_len - rem)) - 1u64)) << rem;
                u64_digit >>= bit_len - rem;
                rem += 64 - bit_len;
                limb
            }
        })
        .collect()
}

/// We do a blanket implementation in 'community-edition' to make it easier to integrate with other crates.
///
/// ASSUMING F::Repr is little-endian
impl<F> ScalarField for F
where
    F: PrimeField + FromUniformBytes<64> + From<bool> + Hash + Ord,
{
    #[inline(always)]
    fn to_u64_limbs(self, num_limbs: usize, bit_len: usize) -> Vec<u64> {
        let bytes = self.to_repr();
        let uint = BigUint::from_bytes_le(bytes.as_ref());
        let digits = uint.iter_u64_digits();
        decompose_u64_digits_to_limbs(digits, num_limbs, bit_len)
    }
}

// Later: will need to separate BigPrimeField from ScalarField when Goldilocks is introduced

/// [ScalarField] that is ~256 bits long
pub trait BigPrimeField: PrimeField<Repr = [u8; 32]> + ScalarField {}

impl<F> BigPrimeField for F where F: PrimeField<Repr = [u8; 32]> + ScalarField {}
