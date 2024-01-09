use ark_ec::VariableBaseMSM;
use ark_bn254::{G1Projective as G, G1Affine as GAffine, Fr as ScalarField};
use ark_std::{UniformRand, error::Error};

// For benchmarking
use std::time::{Duration, Instant};
use jemalloc_ctl::{stats, epoch};

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

fn single_msm() -> Result<(), Box<dyn Error>> {
    epoch::advance().unwrap();  // Update jemalloc stats

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
pub fn run_msm_bench(num_msm: Option<u32>) -> Result<(), Box<dyn Error>> {
    let num_msm = num_msm.unwrap_or(1000);    // default to 1000 msm operations

    let allocated_before = stats::allocated::read().unwrap();
    let resident_before = stats::resident::read().unwrap();

    let mut total_msm = Duration::new(0, 0);

    for _ in 0..num_msm {
        let start = Instant::now();
        single_msm()?;
        total_msm += start.elapsed();
    }

    let allocated_after = stats::allocated::read().unwrap();
    let resident_after = stats::resident::read().unwrap();

    let allocated_mem = (allocated_after - allocated_before) as f64 / 1_000_000f64;
    let resident_mem = (resident_after - resident_before) as f64 / 1_000_000f64;
    let msm_avg = total_msm / num_msm.try_into().unwrap();
    let msm_avg = msm_avg.subsec_nanos() as f64 / 1_000_000_000f64 + (msm_avg.as_secs() as f64);
    
    println!("\nBenchmarking {:?} msm on BN254 curve", num_msm);
    println!("└─ Average msm time: {:?} seconds\n└─ Overall processing time: {:?}", msm_avg, total_msm);
    println!("└─ Memory allocated: {} MB", allocated_mem);
    println!("└─ Resident memory: {} MB", resident_mem);
    Ok(())
}