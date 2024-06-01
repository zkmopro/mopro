#[cfg(all(test))]
mod tests {
    use ark_bn254::{Fq, FqConfig, Fr as ScalarField, G1Affine as GAffine, G1Projective as G};
    use ark_ec::{
        short_weierstrass::Projective, twisted_edwards::MontCurveConfig, AffineRepr, CurveGroup,
        Group, VariableBaseMSM,
    };
    use ark_ff::{
        biginteger::{BigInteger, BigInteger256},
        models::{MontBackend, MontConfig, MontFp},
        PrimeField, UniformRand,
    };
    use ark_std::{cfg_into_iter, rand, vec::Vec, One, Zero};

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

    #[test]
    fn test_msm_accumulation_phase() {
        /* Init params for both schemes */
        let num_points = 2;
        let num_scalars = 2;
        let modulus_bit_size = ScalarField::MODULUS_BIT_SIZE as usize;
        let one = ScalarField::one().into_bigint();
        let zero = G::zero();

        let mut rng = rand::thread_rng();
        // let points: Vec<GAffine> = (0..num_points)
        //     .map(|_| G::rand(&mut rng).into_affine())
        //     .collect();
        // let scalars: Vec<ScalarField> = (0..num_scalars)
        // .map(|_| ScalarField::rand(&mut rng))
        // .collect();
        let a_point = G::rand(&mut rng).into_affine();
        let b_point = G::rand(&mut rng).into_affine();
        let s1 = ScalarField::from(2u64);
        let s2 = ScalarField::from(3u64);
        let points: Vec<GAffine> = vec![a_point.clone(), b_point.clone()];
        let scalars: Vec<ScalarField> = vec![s1.clone(), s2.clone()];
        let instances_size = ark_std::cmp::min(points.len(), scalars.len());
        let c = if instances_size < 32 {
            3
        } else {
            ln_without_floats(instances_size) + 2
        };
        let buckets_size = (1 << c) - 1;

        let bases = &points[..instances_size];
        let window_starts: Vec<_> = (0..modulus_bit_size).step_by(c).collect();

        /* do msm accumulation stage on metal */
        // flatten scalar and base to Vec<u32>
        let scalars_limbs = cfg_into_iter!(scalars.clone())
            .map(|s| s.0.to_u32_limbs())
            .flatten()
            .collect::<Vec<u32>>();

        // The base point should be in montgomery form
        // montgomery form point at infinity: x = 0, y = 1, z = 0
        // sw form point at infinity: x = 1, y = 1, z = 0

        let bases_limbs = cfg_into_iter!(bases)
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
        let buckets_limbs = {
            let matrix = vec![zero; buckets_size];
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
        let state = MetalState::new(None).unwrap();
        let window_size_buffer = state.alloc_buffer_data(&[c as u32]);
        let scalar_buffer = state.alloc_buffer_data(&scalars_limbs);
        let base_buffer = state.alloc_buffer_data(&bases_limbs);

        // Debug buffer
        let debug_buffer_k = state.alloc_buffer::<u32>(8);
        let debug_buffer_p = state.alloc_buffer::<u32>(8);

        let calc_bucket_pipe = state.setup_pipeline("calculate_buckets").unwrap();

        let metal_accumulation_phase: Vec<_> = (0..window_starts.len())
            .map(|w_start| {
                // Get the res value
                let mut res = zero;
                if w_start == 0 && scalars[0].0 == one {
                    res += bases[0];
                }

                let buckets_buffer = state.alloc_buffer_data(&buckets_limbs);

                autoreleasepool(|| {
                    let (command_buffer, command_encoder) = state.setup_command(
                        &calc_bucket_pipe,
                        Some(&[
                            (1, &window_size_buffer),
                            (2, &scalar_buffer),
                            (3, &base_buffer),
                            (4, &buckets_buffer),
                            (5, &debug_buffer_k),
                            (6, &debug_buffer_p),
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

                // Debug buffer
                let debug_k = MetalState::retrieve_contents::<u32>(&debug_buffer_k);
                let debug_p = MetalState::retrieve_contents::<u32>(&debug_buffer_p);
                println!("(metal) debug_k: {:?}", debug_k);
                println!("(metal) debug_p: {:?}", debug_p);

                // println!("(metal) origin_k: {:#?}", scalars_limbs);
                // println!("(metal) origin_p: {:#?}", bases_limbs);

                // recover the points from the buckets
                let buckets = {
                    let raw_limbs = MetalState::retrieve_contents::<u32>(&buckets_buffer);
                    println!("(metal) raw_limbs: {:?}", raw_limbs);
                    let limbs = raw_limbs
                        .chunks(8)
                        .map(|x| BigInteger256::from_u32_limbs(&x))
                        .collect::<Vec<_>>();
                    println!("(metal) limbs: {:?}", limbs);
                    limbs
                        .chunks(3)
                        .map(|chunk| {
                            let x = <Fq as PrimeField>::from_bigint(chunk[0]).unwrap();
                            let y = <Fq as PrimeField>::from_bigint(chunk[1]).unwrap();
                            let z = <Fq as PrimeField>::from_bigint(chunk[2]).unwrap();
                            println!("(metal) (x, y, z): {:?}", (x, y, z));
                            // println!("(metal) new point: {:?}", G::new(x, y, z).is_zero());
                            G::new_unchecked(x, y, z)
                        })
                        .collect::<Vec<_>>()
                };

                (res, buckets)
            })
            .collect();
        println!(
            "(metal) metal_accumulation_phase: {:#?}\n",
            metal_accumulation_phase
        );

        /* do msm accumulation stage on arkworks */
        let bigints = cfg_into_iter!(scalars.clone())
            .map(|s| s.into_bigint())
            .collect::<Vec<_>>();
        let scalars_arkworks = &bigints[..instances_size];
        let scalars_and_bases_iter = scalars_arkworks
            .iter()
            .zip(bases)
            .filter(|(s, _)| !s.is_zero());

        let arkworks_accumulation_phase: Vec<_> = ark_std::cfg_into_iter!(window_starts)
            .map(|w_start| {
                let mut res = zero;
                let mut buckets = vec![zero; buckets_size];
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

                // // print the first 10 buckets
                // println!("(arkworks) buckets: {:?}", buckets.len());
                // for i in 0..10 {
                //     println!("(arkworks) bucket[{}]: {:?}", i, buckets[i]);
                // }

                (res, buckets)
            })
            .collect();
        println!(
            "(arkworks) arkworks_accumulation_phase: {:#?}\n",
            arkworks_accumulation_phase
        );

        // Metal and arkworks accumulation phase should be the same
        // assert_eq!(metal_accumulation_phase, arkworks_accumulation_phase);

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
        let window_sums: Vec<_> = arkworks_accumulation_phase
            .into_iter()
            .map(|(mut res, buckets)| {
                let mut running_sum = G::zero();
                // let mut flag = 0;
                buckets.into_iter().rev().for_each(|b| {
                    // if flag < 10 {
                    //     println!("(arkworks) running_sum: {:?}", running_sum);
                    //     println!("(arkworks) res: {:?}", res);
                    //     flag += 1;
                    // }
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

        let msm = G::msm(&[a_point, b_point], &[s1, s2]).unwrap();
        assert_eq!(arkworks_result, msm);

        let small_msm = a_point * s1 + b_point * s2;
        println!("(arkworks) arkworks_result: {:?}", arkworks_result);
        println!("(arkworks) msm: {:?}", msm);
        println!("(arkworks) small_msm: {:?}", small_msm);
        assert_eq!(arkworks_result, small_msm);
        assert_eq!(msm, small_msm);
    }
}
