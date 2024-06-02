use ark_bn254::{Fq, Fr as ScalarField, G1Affine as GAffine, G1Projective as G};
use ark_ec::{AffineRepr, CurveGroup, Group, VariableBaseMSM};
use ark_ff::{
    biginteger::{BigInteger, BigInteger256},
    PrimeField, UniformRand,
};
use ark_std::{cfg_into_iter, rand, vec::Vec, One, Zero};

// For benchmarking
use ark_serialize::CanonicalDeserialize;
use std::time::{Duration, Instant};

use crate::middleware::gpu_explorations::metal::abstraction::{
    errors::MetalError,
    limbs_conversion::{FromLimbs, ToLimbs},
    state::*,
};
use crate::middleware::gpu_explorations::utils::{benchmark::BenchmarkResult, preprocess};

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
fn metal_msm(points: &[GAffine], scalars: &[ScalarField]) -> Result<G, MetalError> {
    let modulus_bit_size = ScalarField::MODULUS_BIT_SIZE as usize;
    let zero = G::zero();

    let instances_size = ark_std::cmp::min(points.len(), scalars.len());
    let window_size = if instances_size < 32 {
        3
    } else {
        ln_without_floats(instances_size) + 2
    };
    let buckets_size = (1 << window_size) - 1;
    let num_windows = modulus_bit_size / window_size + 1;

    // flatten scalar and base to Vec<u32>
    let scalars_limbs = cfg_into_iter!(scalars.clone())
        .map(|s| s.into_bigint().to_u32_limbs())
        .flatten()
        .collect::<Vec<u32>>();

    let bases_limbs = cfg_into_iter!(points)
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
    let window_size_buffer = state.alloc_buffer_data(&[window_size as u32]);
    let scalar_buffer = state.alloc_buffer_data(&scalars_limbs);
    let base_buffer = state.alloc_buffer_data(&bases_limbs);

    let calc_bucket_pipe = state.setup_pipeline("calculate_buckets").unwrap();

    Ok((0..num_windows)
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
        .reduce(|t, g| t.mul_bigint(&[1_u64 << window_size]) + g)
        .unwrap_or_else(|| zero))
}

pub fn benchmark_msm<I>(
    instances: I,
    iterations: u32,
) -> Result<Vec<Duration>, preprocess::HarnessError>
where
    I: Iterator<Item = preprocess::Instance>,
{
    let mut instance_durations = Vec::new();

    for instance in instances {
        let points = &instance.0;
        // map each scalar to a ScalarField
        let scalars = &instance
            .1
            .iter()
            .map(|s| ScalarField::new(*s))
            .collect::<Vec<ScalarField>>();

        let mut instance_total_duration = Duration::ZERO;
        for _i in 0..iterations {
            let start = Instant::now();
            let _result = metal_msm(&points[..], &scalars[..]).unwrap();

            instance_total_duration += start.elapsed();
        }
        let instance_avg_duration = instance_total_duration / iterations;

        println!(
            "Average time to execute MSM with {} points and scalars in {} iterations is: {:?}",
            points.len(),
            iterations,
            instance_avg_duration,
        );
        instance_durations.push(instance_avg_duration);
    }
    Ok(instance_durations)
}

pub fn run_benchmark(
    instance_size: u32,
    num_instance: u32,
    utils_dir: &str,
) -> Result<BenchmarkResult, preprocess::HarnessError> {
    // Check if the vectors have been generated
    match preprocess::FileInputIterator::open(&utils_dir) {
        Ok(_) => {
            println!("Vectors already generated");
        }
        Err(_) => {
            preprocess::gen_vectors(instance_size, num_instance, &utils_dir);
        }
    }

    let benchmark_data = preprocess::FileInputIterator::open(&utils_dir).unwrap();
    let instance_durations = benchmark_msm(benchmark_data, 1).unwrap();
    // in milliseconds
    let avg_processing_time: f64 = instance_durations
        .iter()
        .map(|d| d.as_secs_f64() * 1000.0)
        .sum::<f64>()
        / instance_durations.len() as f64;

    println!("Done running benchmark.");
    Ok(BenchmarkResult {
        instance_size: instance_size,
        num_instance: num_instance,
        avg_processing_time: avg_processing_time,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    use ark_serialize::Write;
    use std::fs::File;

    const INSTANCE_SIZE: u32 = 10;
    const NUM_INSTANCE: u32 = 1;
    const UTILSPATH: &str = "mopro-core/src/middleware/gpu_explorations/utils/vectors";
    const BENCHMARKSPATH: &str = "mopro-core/gpu_explorations/benchmarks";

    #[test]
    fn test_metal_msm() {
        let dir = format!(
            "{}/{}/{}x{}",
            preprocess::get_root_path(),
            UTILSPATH,
            INSTANCE_SIZE,
            NUM_INSTANCE
        );

        // Check if the vectors have been generated
        match preprocess::FileInputIterator::open(&dir) {
            Ok(_) => {
                println!("Vectors already generated");
            }
            Err(_) => {
                preprocess::gen_vectors(INSTANCE_SIZE, NUM_INSTANCE, &dir);
            }
        }

        let instances = preprocess::FileInputIterator::open(&dir).unwrap();

        for (i, instance) in instances.enumerate() {
            let points = &instance.0;
            // map each scalar to a ScalarField
            let scalars = &instance
                .1
                .iter()
                .map(|s| ScalarField::new(*s))
                .collect::<Vec<ScalarField>>();
            let metal_msm = metal_msm(&points[..], &scalars[..]).unwrap();
            let arkworks_msm = G::msm(&points[..], &scalars[..]).unwrap();
            assert_eq!(metal_msm, arkworks_msm);
            println!("(pass) Instance {} success", i);
        }
    }

    #[test]
    fn test_benchmark_msm() {
        let dir = format!(
            "{}/{}/{}x{}",
            preprocess::get_root_path(),
            UTILSPATH,
            INSTANCE_SIZE,
            NUM_INSTANCE
        );

        // Check if the vectors have been generated
        match preprocess::FileInputIterator::open(&dir) {
            Ok(_) => {
                println!("Vectors already generated");
            }
            Err(_) => {
                preprocess::gen_vectors(INSTANCE_SIZE, NUM_INSTANCE, &dir);
            }
        }

        let benchmark_data = preprocess::FileInputIterator::open(&dir).unwrap();
        let result = benchmark_msm(benchmark_data, 1);
        println!("Done running benchmark: {:?}", result);
    }

    #[test]
    fn test_run_benchmark() {
        let utils_path = format!(
            "{}/{}/{}x{}",
            preprocess::get_root_path(),
            &UTILSPATH,
            INSTANCE_SIZE,
            NUM_INSTANCE
        );
        let result = run_benchmark(INSTANCE_SIZE, NUM_INSTANCE, &utils_path).unwrap();
        println!("Benchmark result: {:#?}", result);
    }

    #[test]
    fn test_run_multi_benchmarks() {
        let output_path = format!(
            "{}/{}/{}_benchmark.txt",
            preprocess::get_root_path(),
            &BENCHMARKSPATH,
            "metal_msm"
        );
        let mut output_file = File::create(output_path).expect("output file creation failed");
        writeln!(output_file, "msm_size,num_msm,avg_processing_time(ms)");

        let instance_size = vec![8, 12, 16, 18, 20, 22];
        let num_instance = vec![10];
        for size in &instance_size {
            for num in &num_instance {
                let utils_path = format!(
                    "{}/{}/{}x{}",
                    preprocess::get_root_path(),
                    &UTILSPATH,
                    *size,
                    *num
                );
                let result = run_benchmark(*size, *num, &utils_path).unwrap();
                println!("{}x{} result: {:#?}", *size, *num, result);
                writeln!(
                    output_file,
                    "{},{},{}",
                    result.instance_size, result.num_instance, result.avg_processing_time
                );
            }
        }
    }
}
