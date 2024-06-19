use ark_bn254::{Fr as ScalarField, G1Projective as G};
use ark_ec::VariableBaseMSM;
use ark_ff::{BigInteger, Field, PrimeField};
use ark_std::{self, cfg_into_iter, One, Zero};
use std::sync::Mutex;
use std::time::{Duration, Instant};

use crate::{
    middleware::gpu_explorations::utils::{benchmark::BenchmarkResult, preprocess},
    MoproError,
};

#[cfg(feature = "parallel")]
use rayon::prelude::*;

// Helper function for getting the windows size
fn ln_without_floats(a: usize) -> usize {
    // log2(a) * ln(2)
    (ark_std::log2(a) * 69 / 100) as usize
}

/// use bucket-wise accumulation in msm
fn bucket_wise_msm<V: VariableBaseMSM>(
    bases: &[V::MulBase],
    scalars: &[V::ScalarField],
) -> Result<V, MoproError> {
    let bigints = cfg_into_iter!(scalars)
        .map(|s| s.into_bigint())
        .collect::<Vec<_>>();
    let instance_size = ark_std::cmp::min(bases.len(), bigints.len());
    let scalars = &bigints[..instance_size];
    let bases = &bases[..instance_size];

    let c = if instance_size < 32 {
        3
    } else {
        ln_without_floats(instance_size) + 2
    };

    let num_bits = V::ScalarField::MODULUS_BIT_SIZE as usize;
    let one = V::ScalarField::one().into_bigint();

    let zero = V::zero();
    let window_starts: Vec<_> = (0..num_bits).step_by(c).collect();
    let num_window = window_starts.len();
    let bucket_len = (1 << c) - 1;

    // prepare buckets and points indices (no need to sort in this case)
    let prepare_start = Instant::now();
    let indices_lists = Mutex::new(vec![(0 as usize, 0 as usize); instance_size * num_window]);
    ark_std::cfg_into_iter!(scalars)
        .enumerate()
        .for_each(|(point_idx, each_scalar)| {
            if *each_scalar == one {
                return;
            }

            for i in 0..num_window {
                let w_start = window_starts[i];
                let mut scalar = *each_scalar;
                scalar.divn(w_start as u32);
                let scalar = scalar.as_ref()[0] % (1 << c);
                if scalar != 0 {
                    let bucket_idx = i * bucket_len + (scalar as usize) - 1;
                    let mut indices_lists = indices_lists.lock().unwrap();
                    indices_lists[point_idx * num_window + i] = (bucket_idx, point_idx);
                }
            }
        });
    println!(
        "Prepare buckets indices time: {:?}",
        prepare_start.elapsed()
    );

    // sort the buckets_indices parallelly
    let sort_start = Instant::now();
    let mut indices_lists = indices_lists.lock().unwrap();
    // indices_lists.par_sort_unstable_by_key(|a| a.0);    // unstable version is faster
    indices_lists.par_sort_unstable_by(|a, b| {
        if a.0 == b.0 {
            a.1.cmp(&b.1)
        } else {
            a.0.cmp(&b.0)
        }
    });

    // remove the first few (0, 0) indices
    let mut k = 0;
    while (indices_lists[k].0 == 0) && (indices_lists[k].1 == 0) {
        k += 1;
    }
    indices_lists.par_drain(0..k);
    println!("Sort buckets indices time: {:?}", sort_start.elapsed());

    // find the start and end of each bucket
    let total_buckets_size = num_window * bucket_len;
    let mut bucket_start = vec![0; total_buckets_size];
    let mut bucket_end = vec![0; total_buckets_size];
    let mut prev_bucket_idx = 0;
    let last_idx = indices_lists.len() - 1;
    for (idx, (bucket_idx, _)) in indices_lists.iter().enumerate() {
        if idx == 0 {
            prev_bucket_idx = *bucket_idx;
        } else {
            if *bucket_idx != prev_bucket_idx {
                bucket_end[prev_bucket_idx] = idx;
                bucket_start[*bucket_idx] = idx;
                prev_bucket_idx = *bucket_idx;
            }
            // add the last idx to the end
            if idx == last_idx {
                bucket_end[*bucket_idx] = idx + 1;
            }
        }
    }

    // build an active bucket list to reduce meaning initialization of threads
    let active_buckets: Vec<_> = ark_std::cfg_into_iter!(0..total_buckets_size)
        .filter(|i| bucket_start[*i] != 0 || bucket_end[*i] != 0)
        .collect();

    // do the bucket-wise accumulation
    let accumulation_start = Instant::now();
    let buckets = Mutex::new(vec![V::zero(); total_buckets_size]);
    ark_std::cfg_into_iter!(active_buckets).for_each(|bucket_idx| {
        let mut buckets = buckets.lock().unwrap();
        for i in bucket_start[bucket_idx]..bucket_end[bucket_idx] {
            buckets[bucket_idx] += bases[indices_lists[i].1];
        }
    });
    println!(
        "Accumulate buckets time: {:?}",
        accumulation_start.elapsed()
    );

    // do window-wise reduction
    let reduction_start = Instant::now();
    let window_sums: Vec<_> = ark_std::cfg_into_iter!(buckets.lock().unwrap().clone())
        .chunks(bucket_len)
        .enumerate()
        .map(|(window_idx, bucket)| {
            let mut res = zero;
            if window_idx == 0 {
                for i in 0..instance_size {
                    if scalars[i] == one {
                        res += &bucket[i];
                    }
                }
            }
            let mut running_sum = zero;
            bucket.into_iter().rev().for_each(|b| {
                running_sum += b;
                res += &running_sum;
            });
            res
        })
        .collect();
    println!("Sum reduction time: {:?}", reduction_start.elapsed());

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
            let original_result = <G as VariableBaseMSM>::msm(&points[..], &scalars[..]).unwrap();
            let _result = bucket_wise_msm::<G>(&points[..], &scalars[..]).unwrap();
            println!("Original MSM result: {:?}", original_result);
            println!("Result: {:?}", _result);
            if original_result == _result {
                println!("MSM is correctly computed");
            } else {
                println!("MSM is wrongly computed");
            }

            instance_total_duration += start.elapsed();
        }
        let instance_avg_duration = instance_total_duration / iterations;

        println!(
            "Average time to execute MSM with {} points and {} scalars in {} iterations is: {:?}",
            points.len(),
            scalars.len(),
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

    const INSTANCE_SIZE: u32 = 26;
    const NUM_INSTANCE: u32 = 5;
    const UTILSPATH: &str = "mopro-core/src/middleware/gpu_explorations/utils/vectors";
    const BENCHMARKSPATH: &str = "mopro-core/gpu_explorations/benchmarks";

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
            "arkworks_pippenger"
        );
        let mut output_file = File::create(output_path).expect("output file creation failed");
        writeln!(output_file, "msm_size,num_msm,avg_processing_time(ms)").unwrap();

        let instance_size = vec![16, 18, 20, 22, 24, 26];
        let num_instance = vec![5];
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
                )
                .unwrap();
            }
        }
    }
}
