use ark_bn254::{Fr as ScalarField, G1Affine as GAffine, G1Projective as G};
use ark_ec::{AffineRepr, VariableBaseMSM};
use ark_ff::{BigInteger, PrimeField, UniformRand};
use ark_std::{borrow::Borrow, cfg_into_iter, iterable::Iterable, rand, vec::Vec, One, Zero};

use crate::middleware::gpu_explorations::metal::abstraction::{errors::MetalError, state::*};

use metal::*;
use objc::{rc::autoreleasepool, runtime::YES};

#[cfg(feature = "parallel")]
use rayon::prelude::*;

// Helper function for getting the windows size
fn ln_without_floats(a: usize) -> usize {
    // log2(a) * ln(2)
    (ark_std::log2(a) * 69 / 100) as usize
}

/// Optimized implementation of multi-scalar multiplication.
fn metal_msm<V: VariableBaseMSM>(
    bases: &[V::MulBase],
    _scalars: &[V::ScalarField],
    window_size: usize,
    state: &MetalState,
) -> Result<V, MetalError> {
    let bigints = cfg_into_iter!(_scalars)
        .map(|s| s.into_bigint())
        .collect::<Vec<_>>();
    let size = ark_std::cmp::min(bases.len(), bigints.len());
    let scalars = &bigints[..size];
    let bases = &bases[..size];
    let scalars_and_bases_iter = scalars.iter().zip(bases).filter(|(s, _)| !s.is_zero());

    let c = if size < 32 {
        3
    } else {
        ln_without_floats(size) + 2
    };

    let num_bits = V::ScalarField::MODULUS_BIT_SIZE as usize;
    let one = V::ScalarField::one().into_bigint();

    let zero = V::zero();
    let window_starts: Vec<_> = (0..num_bits).step_by(c).collect();

    // flatten scalar and base to Vec<u32>
    // let scalars = scalars.iter().map(|s| s.as_ref().to_vec()).flatten().collect::<Vec<_>>();
    // let bases = bases.iter().map(|b| b..into_uncompressed().to_vec()).flatten().collect::<Vec<_>>();
    // println!("scalars: {:?}", scalars);

    // store params to GPU shared memory
    let scalar_buffer = state.alloc_buffer_data(&scalars);
    let base_buffer = state.alloc_buffer_data(&bases);
    let window_size_buffer = state.alloc_buffer_data(&window_starts);

    let calc_bucket_pipe = state.setup_pipeline("calculate_buckets")?;

    // TODO: integrate `calculate_buckets` functionality into parallel part of pippenger

    /* Part to add wrapper logic of metal shader */

    // Each window is of size `c`.
    // We divide up the bits 0..num_bits into windows of size `c`, and
    // in parallel process each such window.
    let window_sums: Vec<_> = ark_std::cfg_into_iter!(window_starts)
        .map(|w_start| {
            let mut res = zero;
            // We don't need the "zero" bucket, so we only have 2^c - 1 buckets.
            let mut buckets = vec![zero; (1 << c) - 1];
            // This clone is cheap, because the iterator contains just a
            // pointer and an index into the original vectors.
            scalars_and_bases_iter.clone().for_each(|(&scalar, base)| {
                if scalar == one {
                    // We only process unit scalars once in the first window.
                    if w_start == 0 {
                        res += base;
                    }
                } else {
                    let mut scalar = scalar;

                    // We right-shift by w_start, thus getting rid of the
                    // lower bits.
                    scalar.divn(w_start as u32);

                    // We mod the remaining bits by 2^{window size}, thus taking `c` bits.
                    let scalar = scalar.as_ref()[0] % (1 << c);

                    // If the scalar is non-zero, we update the corresponding
                    // bucket.
                    // (Recall that `buckets` doesn't have a zero bucket.)
                    if scalar != 0 {
                        buckets[(scalar - 1) as usize] += base;
                    }
                }
            });

            // Compute sum_{i in 0..num_buckets} (sum_{j in i..num_buckets} bucket[j])
            // This is computed below for b buckets, using 2b curve additions.
            //
            // We could first normalize `buckets` and then use mixed-addition
            // here, but that's slower for the kinds of groups we care about
            // (Short Weierstrass curves and Twisted Edwards curves).
            // In the case of Short Weierstrass curves,
            // mixed addition saves ~4 field multiplications per addition.
            // However normalization (with the inversion batched) takes ~6
            // field multiplications per element,
            // hence batch normalization is a slowdown.

            // `running_sum` = sum_{j in i..num_buckets} bucket[j],
            // where we iterate backward from i = num_buckets to 0.
            let mut running_sum = V::zero();
            buckets.into_iter().rev().for_each(|b| {
                running_sum += &b;
                res += &running_sum;
            });
            res
        })
        .collect();

    // We store the sum for the lowest window.
    let lowest = *window_sums.first().unwrap();

    // We're traversing windows from high to low.
    Ok(lowest
        + &window_sums[1..]
            .iter()
            .rev()
            .fold(zero, |mut total, sum_i| {
                total += sum_i;
                for _ in 0..c {
                    total.double_in_place();
                }
                total
            }))
}

#[cfg(test)]
mod tests {
    use ark_bn254::Config;
    use ark_ec::{short_weierstrass::Projective, CurveGroup, Group};

    use super::*;

    #[test]
    fn test_msm_bigint() {
        let state = MetalState::new(None).unwrap();

        let num_points = 1000;
        let window_size = 4;
        let num_bits = ScalarField::MODULUS_BIT_SIZE as usize;
        let num_scalars = 1000;

        let mut rng = rand::thread_rng();
        let points: Vec<GAffine> = (0..num_points)
            .map(|_| G::rand(&mut rng).into_affine())
            .collect();

        let scalars: Vec<ScalarField> = (0..num_scalars)
            .map(|_| ScalarField::rand(&mut rng))
            .collect();

        let msm = <G as VariableBaseMSM>::msm(&points, &scalars).unwrap();
        let msm_bigint = metal_msm::<G>(&points, &scalars, window_size, &state).unwrap();

        // println!("msm: {:?}", msm);
        // println!("msm_bigint: {:?}", msm_bigint);

        // if (msm - msm_bigint).is_zero() {
        //     println!("msm and msm_bigint are equal");
        // }
        // else {
        //     println!("msm and msm_bigint are not equal");
        // }

        assert_eq!(msm, msm_bigint);
    }
}
