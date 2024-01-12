use ark_bn254::{Fr as ScalarField, G1Affine as GAffine, G1Projective as G};
use ark_ec::VariableBaseMSM;
use ark_std::{error::Error, UniformRand};

// For benchmarking
use jemalloc_ctl::{epoch, stats};
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct BenchmarkResult {
    pub num_msm: u32,
    pub avg_processing_time: f64,
    pub total_processing_time: f64,
    pub allocated_memory: f64,
}

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

fn single_msm() -> Result<(), Box<dyn Error>> {
    let mut rng = ark_std::test_rng();

    // We use the BN254 curve to match Groth16 proving system
    let a = GAffine::rand(&mut rng);
    let b = GAffine::rand(&mut rng);
    let s1 = ScalarField::rand(&mut rng);
    let s2 = ScalarField::rand(&mut rng);

    let r = G::msm(&[a, b], &[s1, s2]).unwrap();

    assert_eq!(r, a * s1 + b * s2);
    Ok(())
}

// Run the msm benchmark with timing
pub fn run_msm_benchmark(num_msm: Option<u32>) -> Result<BenchmarkResult, Box<dyn Error>> {
    let num_msm = num_msm.unwrap_or(1000); // default to 1000 msm operations

    let mem_epoch = epoch::mib().unwrap(); // For updating jemalloc stats of memory usage
    let allocated = stats::allocated::mib().unwrap();

    let mut total_msm = Duration::new(0, 0);

    for _ in 0..num_msm {
        let start = Instant::now();
        single_msm()?;
        total_msm += start.elapsed();
    }
    mem_epoch.advance().unwrap(); // Update msm memory usage

    let allocated_size = allocated.read().unwrap() as f64 / usize::pow(1_024, 2) as f64; // Convert to MiB

    let msm_avg = total_msm / num_msm.try_into().unwrap();
    let msm_avg = msm_avg.subsec_nanos() as f64 / 1_000_000_000f64 + (msm_avg.as_secs() as f64);

    Ok(BenchmarkResult {
        num_msm,
        avg_processing_time: msm_avg,
        total_processing_time: total_msm.as_secs_f64(),
        allocated_memory: allocated_size,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_msm() {
        assert!(single_msm().is_ok());
    }

    #[test]
    fn test_run_msm_benchmark() {
        let benchmarks = run_msm_benchmark(None).unwrap();
        println!("\nBenchmarking {:?} msm on BN254 curve", benchmarks.num_msm);
        println!(
            "└─ Average msm time: {:.5} seconds\n└─ Overall processing time: {:.5} seconds",
            benchmarks.avg_processing_time, benchmarks.total_processing_time
        );
        println!(
            "└─ Memory allocated: {:.5} MiB",
            benchmarks.allocated_memory,
        );
    }
}
