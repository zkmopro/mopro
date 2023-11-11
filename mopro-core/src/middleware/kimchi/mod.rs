use crate::MoproError;
use kimchi::bench::{self, BenchmarkCtx};
use std::time::Instant;

fn hello() {
    println!("Hello, world!");
}

// TODO: Separate out proof generation and verification

pub fn bench() {
    // context created in 21.2235 ms
    let start = Instant::now();
    let srs_size = 4;
    let ctx = BenchmarkCtx::new(srs_size);
    println!("testing bench code for SRS of size {srs_size}");
    println!("context created in {}s", start.elapsed().as_secs());

    // proof created in 7.1227 ms
    let start = Instant::now();
    let (proof, public_input) = ctx.create_proof();
    println!("proof created in {}s", start.elapsed().as_secs());

    // proof verified in 1.710 ms
    let start = Instant::now();
    ctx.batch_verification(&vec![(proof, public_input)]);
    println!("proof verified in {}", start.elapsed().as_secs());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kimchi_bench() {
        bench();
    }
}
