use ark_bn254::{Fr as ScalarField, G1Affine as GAffine, G1Projective as G};
use ark_ec::VariableBaseMSM;
use ark_std::{error::Error, UniformRand};

// For benchmarking
use jemalloc_ctl::{epoch, stats};
use std::time::{Duration, Instant};

// For ouputting benchmark data
use std::{env, fs::File, io::Write};

#[derive(Debug, Clone)]
pub struct BenchmarkResult {
    pub num_msm: u32,
    pub avg_processing_time: f64,
    pub total_processing_time: f64,
    pub allocated_memory: f64,
    pub resident_memory: f64,
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
    let resident = stats::resident::mib().unwrap();

    let mut total_msm = Duration::new(0, 0);

    for _ in 0..num_msm {
        let start = Instant::now();
        single_msm()?;
        total_msm += start.elapsed();
    }
    mem_epoch.advance().unwrap(); // Update jemalloc stats

    let allocated_size = allocated.read().unwrap() as f64 / usize::pow(1_024, 2) as f64; // Convert to MiB
    let resident_size = resident.read().unwrap() as f64 / usize::pow(1_024, 2) as f64; // Convert to MiB

    let msm_avg = total_msm / num_msm.try_into().unwrap();
    let msm_avg = msm_avg.subsec_nanos() as f64 / 1_000_000_000f64 + (msm_avg.as_secs() as f64);

    Ok(BenchmarkResult {
        num_msm,
        avg_processing_time: msm_avg,
        total_processing_time: total_msm.as_secs_f64(),
        allocated_memory: allocated_size,
        resident_memory: resident_size,
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
            "└─ Memory allocated: {:.5} MiB\n└─ Resident memory: {:.5} MiB",
            benchmarks.allocated_memory, benchmarks.resident_memory
        );
    }

    #[test]
    fn test_output_msm_benchmark() {
        let path = env::current_dir()
            .unwrap()
            .join("src/middleware/gpu_exploration/msm_bench.csv");
        let mut file = File::create(path).unwrap();
        writeln!(file, "num_msm,avg_processing_time(sec),total_processing_time(sec),memory_allocated(MiB),resident_memory(MiB)").unwrap();
        let trials = vec![1, 10, 50, 100, 500, 1_000, 5_000, 10_000];
        for each in trials {
            let bench_data = run_msm_benchmark(Some(each)).unwrap();
            writeln!(
                file,
                "{},{},{},{},{}",
                bench_data.num_msm,
                bench_data.avg_processing_time,
                bench_data.total_processing_time,
                bench_data.allocated_memory,
                bench_data.resident_memory
            )
            .unwrap();
        }
    }
}
