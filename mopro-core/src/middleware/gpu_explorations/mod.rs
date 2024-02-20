use ark_bn254::{Fr as ScalarField, G1Affine as GAffine, G1Projective as G};
use ark_ec::VariableBaseMSM;
use ark_std::{error::Error, UniformRand};

// For benchmarking
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct BenchmarkResult {
    pub num_msm: u32,
    pub avg_processing_time: f64,
    pub total_processing_time: f64,
}

// TODO: refactor fn name and add more benchmarks in the future
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

// TODO: figure out a way to configure the algorithm fn used
// Run the msm benchmark with timing
pub fn run_msm_benchmark(num_msm: Option<u32>) -> Result<BenchmarkResult, Box<dyn Error>> {
    let num_msm = num_msm.unwrap_or(1000); // default to 1000 msm operations

    let mut total_msm = Duration::new(0, 0);
    for _ in 0..num_msm {
        let start = Instant::now();
        single_msm()?;
        total_msm += start.elapsed();
    }

    let msm_avg = (total_msm.as_secs_f64() / num_msm as f64) * 1_000.0; // in ms

    Ok(BenchmarkResult {
        num_msm,
        avg_processing_time: msm_avg,
        total_processing_time: total_msm.as_secs_f64() * 1_000.0,
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
            "└─ Average msm time: {:.5} ms\n└─ Overall processing time: {:.5} ms",
            benchmarks.avg_processing_time, benchmarks.total_processing_time
        );
    }
}
