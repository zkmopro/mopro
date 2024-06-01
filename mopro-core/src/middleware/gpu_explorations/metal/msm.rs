use ark_bn254::{Fq, Fr as ScalarField, G1Affine as GAffine, G1Projective as G};
use ark_ec::{AffineRepr, CurveGroup, Group, VariableBaseMSM};
use ark_ff::{
    biginteger::{BigInteger, BigInteger256},
    BigInt, Field, PrimeField, UniformRand,
};
use ark_std::{borrow::Borrow, cfg_into_iter, iterable::Iterable, rand, vec::Vec, One, Zero};

use crate::middleware::gpu_explorations::metal::abstraction::{
    errors::MetalError,
    limbs_conversion::{FromLimbs, ToLimbs},
    state::*,
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

    let zero = G::zero(); // In group form, [x, y, z] = [1, 1, 0]
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
    let bucket_matrix_limbs = {
        let matrix = vec![zero; buckets_size * instances_size];
        println!("(metal) bucket_size: {:?}", buckets_size);
        println!("(metal) instances_size: {:?}", instances_size);
        println!("(metal) matrix len: {:?}", matrix.len());
        cfg_into_iter!(matrix)
            .map(|b| {
                b.x.0
                    .to_u32_limbs()
                    .into_iter()
                    .chain(b.y.0.to_u32_limbs())
                    .chain(b.z.0.to_u32_limbs())
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect::<Vec<u32>>()
    };

    // store params to GPU shared memory
    let window_size_buffer = state.alloc_buffer_data(&[c as u32]);
    let scalar_buffer = state.alloc_buffer_data(&scalars);
    let base_buffer = state.alloc_buffer_data(&bases);

    let calc_bucket_pipe = state.setup_pipeline("calculate_buckets")?;

    println!("(metal) window_starts: {:?}", window_starts.len());
    let window_sums: Vec<_> = (0..window_starts.len())
        .map(|w_start| {
            let buckets_matrix_buffer = state.alloc_buffer_data(&bucket_matrix_limbs);

            objc::rc::autoreleasepool(|| {
                let (command_buffer, command_encoder) = state.setup_command(
                    &calc_bucket_pipe,
                    Some(&[
                        (1, &window_size_buffer),
                        (2, &scalar_buffer),
                        (3, &base_buffer),
                        (4, &buckets_matrix_buffer),
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
            let mut counter = 0;
            let buckets_matrix = {
                let raw_limbs = MetalState::retrieve_contents::<u32>(&buckets_matrix_buffer);
                let limbs = raw_limbs
                    .chunks(8)
                    .map(|x| {
                        if counter < 10 {
                            println!("(metal) raw_limbs: {:?}", x);
                            counter += 1;
                        }
                        BigInteger256::from_u32_limbs(&x)
                    })
                    .collect::<Vec<_>>();
                limbs
                    .chunks(3)
                    .map(|chunk| {
                        let x = <Fq as PrimeField>::from_bigint(chunk[0]).unwrap();
                        let y = <Fq as PrimeField>::from_bigint(chunk[1]).unwrap();
                        let z = <Fq as PrimeField>::from_bigint(chunk[2]).unwrap();

                        // if counter < 10 {
                        //     println!("(metal) x: {:?}", x);
                        //     println!("(metal) y: {:?}", y);
                        //     println!("(metal) z: {:?}", z);
                        //     counter += 1;
                        // }
                        G::new_unchecked(x, y, z)
                    })
                    .collect::<Vec<_>>()
            };
            // println!("(metal) buckets_matrix: {:?}", buckets_matrix.len());

            // from matrix to bucket
            let mut buckets = Vec::with_capacity(buckets_size);
            // let mut buckets = Vec::with_capacity(buckets_size);
            for i in 0..buckets_size {
                let mut partial_sum = buckets_matrix[i].clone();

                for j in 1..instances_size {
                    partial_sum += &buckets_matrix[i + j * buckets_size];
                    // partial_sum = partial_sum.operate_with(&buckets_matrix[i + j * buckets_size]);
                }
                buckets.push(partial_sum);
            }
            println!("(metal) buckets: {:?}", buckets.len());
            // print the first 10 buckets
            for i in 0..10 {
                println!("(metal) bucket[{}]: {:?}", i, buckets[i]);
            }

            let mut res = zero;
            let mut running_sum = zero;
            let mut flag = 0;
            buckets.into_iter().rev().for_each(|b| {
                if flag < 10 {
                    println!("(metal) running_sum: {:?}", running_sum);
                    println!("(metal) res: {:?}", res);
                    flag += 1;
                }
                running_sum += &b;
                res += &running_sum;
            });
            res
        })
        .collect();

    // println!("window_sums[{:?}]: {:?}", window_sums.len(), window_sums);

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

        let msm_bigint = metal_msm(&points, &scalars, window_size, &state).unwrap();
        // let msm = <G as VariableBaseMSM>::msm(&points, &scalars).unwrap();

        // manual test for comparison between arkworks msm_bigint
        let bigints = cfg_into_iter!(scalars)
            .map(|s| s.into_bigint())
            .collect::<Vec<_>>();

        let size = ark_std::cmp::min(points.len(), bigints.len());
        let scalars = &bigints[..size];
        let bases = &points[..size];
        let scalars_and_bases_iter = scalars.iter().zip(bases).filter(|(s, _)| !s.is_zero());

        let c = if size < 32 {
            3
        } else {
            super::ln_without_floats(size) + 2
        };

        let num_bits = ScalarField::MODULUS_BIT_SIZE as usize;
        let one = ScalarField::one().into_bigint();

        let zero = G::zero();
        let window_starts: Vec<_> = (0..num_bits).step_by(c).collect();

        // Each window is of size `c`.
        // We divide up the bits 0..num_bits into windows of size `c`, and
        // in parallel process each such window.

        println!("(arkworks) window_starts: {:?}", window_starts.len());
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
                        // let mut counter = 0;
                        // if counter < 10 {
                        //     println!("(arkworks) scalar: {:?}", scalar.as_ref());
                        //     counter += 1;
                        // }
                        let scalar = scalar.as_ref()[0] % (1 << c);

                        // If the scalar is non-zero, we update the corresponding
                        // bucket.
                        // (Recall that `buckets` doesn't have a zero bucket.)
                        if scalar != 0 {
                            buckets[(scalar - 1) as usize] += base;
                        }
                    }
                });

                println!("(arkworks) buckets: {:?}", buckets.len());
                // print the first 10 buckets
                for i in 0..10 {
                    println!("(arkworks) bucket[{}]: {:?}", i, buckets[i]);
                }

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

                let mut flag = 0;
                let mut running_sum = G::zero();
                buckets.into_iter().rev().for_each(|b| {
                    if flag < 10 {
                        println!("(arkworks) running_sum: {:?}", running_sum);
                        println!("(arkworks) res: {:?}", res);
                        flag += 1;
                    }
                    running_sum += &b;
                    res += &running_sum;
                });
                res
            })
            .collect();

        // We store the sum for the lowest window.
        let lowest = *window_sums.first().unwrap();

        // We're traversing windows from high to low.
        let arkworks_result = lowest
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

        assert_eq!(arkworks_result, msm_bigint);
    }
}
