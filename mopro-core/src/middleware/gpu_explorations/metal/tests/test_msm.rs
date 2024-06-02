#[cfg(all(test))]
mod tests {
    use ark_bn254::{Fq, FqConfig, Fr as ScalarField, G1Affine as GAffine, G1Projective as G};
    use ark_ec::{AffineRepr, CurveGroup, Group, VariableBaseMSM};
    use ark_ff::{
        biginteger::{BigInteger, BigInteger256},
        PrimeField, UniformRand,
    };
    use ark_std::{cfg_into_iter, rand, vec::Vec, One, Zero};

    use crate::middleware::gpu_explorations::metal::abstraction::{
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
        let num_points = 1000;
        let num_scalars = 1000;
        let modulus_bit_size = ScalarField::MODULUS_BIT_SIZE as usize;
        let zero = G::zero();

        let mut rng = rand::thread_rng();
        let points: Vec<GAffine> = (0..num_points)
            .map(|_| G::rand(&mut rng).into_affine())
            .collect();
        let scalars: Vec<ScalarField> = (0..num_scalars)
            .map(|_| ScalarField::rand(&mut rng))
            .collect();
        let instances_size = ark_std::cmp::min(points.len(), scalars.len());
        let c = if instances_size < 32 {
            3
        } else {
            ln_without_floats(instances_size) + 2
        };
        let buckets_size = (1 << c) - 1;

        let bases = &points[..instances_size];
        let num_windows = modulus_bit_size / c + 1;

        // flatten scalar and base to Vec<u32>
        let scalars_limbs = cfg_into_iter!(scalars.clone())
            .map(|s| s.into_bigint().to_u32_limbs())
            .flatten()
            .collect::<Vec<u32>>();

        let bases_limbs = cfg_into_iter!(bases)
            .map(|b| {
                let b = b.into_group();
                b.x.to_u32_limbs()
                    .into_iter()
                    .chain(b.y.to_u32_limbs())
                    .chain(b.z.to_u32_limbs())
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect::<Vec<u32>>();
        let buckets_matrix_limbs = {
            // buckets_size * instances_size is for parallelism on windows
            let matrix = vec![zero; buckets_size * instances_size];
            cfg_into_iter!(matrix)
                .map(|b| {
                    b.x.to_u32_limbs()
                        .into_iter()
                        .chain(b.y.to_u32_limbs())
                        .chain(b.z.to_u32_limbs())
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

        let calc_bucket_pipe = state.setup_pipeline("calculate_buckets").unwrap();

        let metal_result = (0..num_windows)
            .rev()
            .map(|w_start| {
                let buckets_matrix_buffer = state.alloc_buffer_data(&buckets_matrix_limbs);

                autoreleasepool(|| {
                    let (command_buffer, command_encoder) = state.setup_command(
                        &calc_bucket_pipe,
                        Some(&[
                            (1, &window_size_buffer),
                            (2, &scalar_buffer),
                            (3, &base_buffer),
                            (4, &buckets_matrix_buffer),
                        ]),
                    );

                    MetalState::set_bytes(0, &[w_start as u32], command_encoder);

                    command_encoder.dispatch_thread_groups(
                        MTLSize::new(1, 1, 1),
                        MTLSize::new(instances_size as u64, 1, 1),
                    );
                    command_encoder.end_encoding();
                    command_buffer.commit();
                    command_buffer.wait_until_completed();
                });

                let buckets_matrix = {
                    let raw_limbs = MetalState::retrieve_contents::<u32>(&buckets_matrix_buffer);
                    raw_limbs
                        .chunks(24)
                        .map(|chunk| {
                            G::new(
                                Fq::from_u32_limbs(&chunk[0..8]),
                                Fq::from_u32_limbs(&chunk[8..16]),
                                Fq::from_u32_limbs(&chunk[16..24]),
                            )
                        })
                        .collect::<Vec<_>>()
                };

                let mut buckets = Vec::with_capacity(buckets_size);

                cfg_into_iter!(0..buckets_size)
                    .map(|i| {
                        let mut partial_accumulation = buckets_matrix[i].clone();
                        for j in 1..instances_size {
                            partial_accumulation += buckets_matrix[i + j * buckets_size];
                        }
                        partial_accumulation
                    })
                    .for_each(|b| buckets.push(b));

                // Reduction phase
                buckets
                    .iter_mut()
                    .rev()
                    .scan(zero, |m, b| {
                        *m += b.clone();
                        Some(m.clone())
                    })
                    .reduce(|g, m| g + m)
                    .unwrap_or_else(|| zero)
            })
            .reduce(|t, g| t.mul_bigint(&[1_u64 << c]) + g)
            .unwrap_or_else(|| zero);

        let arkworks_result = G::msm(&points, &scalars).unwrap();
        assert_eq!(arkworks_result, metal_result);
    }
}
