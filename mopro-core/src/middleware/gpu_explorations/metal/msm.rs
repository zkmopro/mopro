use ark_bn254::{Fr as ScalarField, G1Affine as GAffine, G1Projective as G};
use ark_ec::{AffineRepr, CurveGroup, Group, VariableBaseMSM};
use ark_ff::{
    biginteger::{BigInteger, BigInteger256},
    BigInt, Field, PrimeField, UniformRand,
};
use ark_std::{borrow::Borrow, cfg_into_iter, iterable::Iterable, rand, vec::Vec, One, Zero};

use crate::middleware::gpu_explorations::metal::abstraction::{
    errors::MetalError, state::*, utils::*,
};

use metal::*;
use objc::rc::autoreleasepool;

#[cfg(feature = "parallel")]
use rayon::prelude::*;

// Helper function for getting the windows size
fn ln_without_floats(a: usize) -> usize {
    // log2(a) * ln(2)
    (ark_std::log2(a) * 69 / 100) as usize
}

/// Optimized implementation of multi-scalar multiplication.
fn metal_msm(
    // bases: MulBase],
    // _scalars: &[V::ScalarField],
    bases: &[GAffine],
    scalars: &[ScalarField],
    window_size: usize,
    state: &MetalState,
) -> Result<G, MetalError> {
    // let bigints = cfg_into_iter!(_scalars)
    //     .map(|s| s.into_bigint())
    //     .collect::<Vec<_>>();
    // let size = ark_std::cmp::min(bases.len(), bigints.len());
    // let scalars = &bigints[..size];
    let instances_size = ark_std::cmp::min(bases.len(), scalars.len());
    let scalars = &scalars[..instances_size];
    let bases = &bases[..instances_size];

    // set window size
    let c = if instances_size < 32 {
        3
    } else {
        ln_without_floats(instances_size) + 2
    };
    let buckets_size = (1 << c) - 1;

    let num_bits = ScalarField::MODULUS_BIT_SIZE as usize;
    // let one = ScalarField::one().into_bigint();

    let zero = GAffine::zero().into_group(); // In group form, [x, y, z] = [1, 1, 0]
    let window_starts: Vec<_> = (0..num_bits).step_by(c).collect();

    // flatten scalar and base to Vec<u32>
    let scalars = cfg_into_iter!(scalars)
        .map(|s| s.0.to_u32_limbs())
        .flatten()
        .collect::<Vec<u32>>();
    let bases = cfg_into_iter!(bases)
        .map(|b| {
            let b = b.into_group();
            b.x.0
                .to_u32_limbs()
                .into_iter()
                .chain(b.y.0.to_u32_limbs())
                .chain(b.z.0.to_u32_limbs())
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<Vec<u32>>();

    // store params to GPU shared memory
    let window_size_buffer = state.alloc_buffer_data(&[c as u32]);
    let scalar_buffer = state.alloc_buffer_data(&scalars);
    let base_buffer = state.alloc_buffer_data(&bases);
    let buckets_buffer = state.alloc_buffer::<u32>(buckets_size * instances_size * 3 * 8); // x, y, z coordinates each has 8 limbs

    let calc_bucket_pipe = state.setup_pipeline("calculate_buckets")?;

    // TODO: integrate `calculate_buckets` functionality into parallel part of pippenger
    let window_sums: Vec<_> = (0..window_starts.len())
        .step_by(c)
        .map(|w_start| {
            let window_idx_buffer = state.alloc_buffer_data(&[w_start as u32]);
            objc::rc::autoreleasepool(|| {
                let (command_buffer, command_encoder) = state.setup_command(
                    &calc_bucket_pipe,
                    Some(&[
                        (0, &window_idx_buffer),
                        (1, &window_size_buffer),
                        (2, &scalar_buffer),
                        (3, &base_buffer),
                        (4, &buckets_buffer),
                    ]),
                );

                MetalState::set_bytes(0, &[w_start], command_encoder);

                command_encoder.dispatch_thread_groups(
                    MTLSize::new(1, 1, 1),
                    MTLSize::new(instances_size as u64, 1, 1),
                );
                command_encoder.end_encoding();
                command_buffer.commit();
                command_buffer.wait_until_completed();
            });

            // recover the points from the buckets
            let buckets_matrix = {
                let raw_limbs = MetalState::retrieve_contents::<u32>(&buckets_buffer);
                let limbs = raw_limbs
                    .chunks(8)
                    .map(BigInteger256::from_u32_limbs)
                    .collect::<Vec<_>>();
                limbs
                    .chunks(3)
                    .map(|chunk| GAffine::new_unchecked(chunk[0].into(), chunk[1].into()))
                    .collect::<Vec<_>>()
            };
            // println!("buckets_matrix: {:?}", buckets_matrix);

            let mut res = zero;
            let mut running_sum = zero;
            buckets_matrix.into_iter().rev().for_each(|b| {
                // println!("running_sum: {:?}", running_sum);
                // println!("res: {:?}", res);
                running_sum += &b;
                res += &running_sum;
            });
            res
        })
        .collect();

    println!("window_sums: {:?}", window_sums);

    // We store the sum for the lowest window.
    let lowest = *window_sums.first().unwrap();

    // We're traversing windows from high to low.
    let result = lowest
        + &window_sums[1..]
            .iter()
            .rev()
            .fold(zero, |mut total, sum_i| {
                total += sum_i;
                for _ in 0..c {
                    total.double_in_place();
                }
                total
            });
    Ok(result)
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
        let msm_bigint = metal_msm(&points, &scalars, window_size, &state).unwrap();

        // now will fail, since msm shader is not implemented in BN254
        assert_eq!(msm, msm_bigint);
    }
}
