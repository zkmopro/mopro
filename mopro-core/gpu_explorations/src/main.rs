mod msm_bench;
use crate::msm_bench::run_msm_bench;


// Dummy main function as driver code
fn main() {
    run_msm_bench(None).unwrap();
}