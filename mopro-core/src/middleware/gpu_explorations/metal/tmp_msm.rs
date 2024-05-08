use ark_bn254::{Fr as ScalarField, G1Affine as GAffine, G1Projective as G};
use ark_ec::{AffineRepr, VariableBaseMSM};
use ark_ff::{BigInt, PrimeField, UniformRand};
use ark_std::rand;

use lambdaworks_math::{
    cyclic_group::IsGroup,
    elliptic_curve::short_weierstrass::{
        curves::bls12_381::curve::BLS12381Curve, point::ShortWeierstrassProjectivePoint,
    },
    unsigned_integer::{element::U384, traits::U32Limbs},
};

use crate::middleware::gpu_explorations::metal::abstractions::{errors::MetalError, state::*};

use metal::*;
use objc::{rc::autoreleasepool, runtime::YES};

type Point = GAffine;
type Scalar = <ScalarField as PrimeField>::BigInt;

const NUM_LIMBS: usize = 6;

/// Computes the multiscalar multiplication (MSM), using Pippenger's algorithm parallelized in
/// Metal.
pub fn pippenger(
    cs: &[Scalar],
    hidings: &[Point],
    window_size: usize,
    state: &MetalState,
) -> Result<Point, MetalError> {
    if cs.len() != hidings.len() {
        return Err(MetalError::InputError(format!(
            "cs's length ({}) and hidings length ({}) need to match.",
            cs.len(),
            hidings.len(),
        )));
    }

    if cs.is_empty() {
        // return Ok(Point::neutral_element());
        return Ok(Point::identity());
    }

    let point_len = cs.len(); // == hidings.len();

    let n_bits = 64 * NUM_LIMBS;
    let num_windows = (n_bits - 1) / window_size + 1;
    let buckets_len = (1 << window_size) - 1;

    let rng = &mut rand::thread_rng();

    // let buckets_matrix_limbs = {
        // let matrix = vec![Point::rand(rng); buckets_len * point_len];
        // Point::to_flat_u32_limbs(&matrix)
    // };
    // let k_limbs = U384::to_flat_u32_limbs(cs);
    // let p_limbs = Point::to_flat_u32_limbs(hidings);

    // let k_buffer = state.alloc_buffer_data(&k_limbs);
    // let p_buffer = state.alloc_buffer_data(&p_limbs);

    let buckets_matrix_limbs = vec![Point::rand(rng); buckets_len * point_len * NUM_LIMBS];
    let k_buffer = state.alloc_buffer_data(&cs);
    let p_buffer = state.alloc_buffer_data(&hidings);
    let wsize_buffer = state.alloc_buffer_data(&[window_size as u32]);

    let calc_buckets_pipe = state.setup_pipeline("calculate_buckets")?;
    Ok((0..num_windows)
        .rev()
        .map(|window_idx| {
            let buckets_matrix_buffer = state.alloc_buffer_data(&buckets_matrix_limbs);

            objc::rc::autoreleasepool(|| {
                let (command_buffer, command_encoder) = state.setup_command(
                    &calc_buckets_pipe,
                    Some(&[
                        (1, &wsize_buffer),
                        (2, &k_buffer),
                        (3, &p_buffer),
                        (4, &buckets_matrix_buffer),
                    ]),
                );

                MetalState::set_bytes(0, &[window_idx], command_encoder);

                command_encoder.dispatch_thread_groups(
                    MTLSize::new(1, 1, 1),
                    MTLSize::new(point_len as u64, 1, 1),
                );
                command_encoder.end_encoding();
                command_buffer.commit();
                command_buffer.wait_until_completed();
            });

            // let buckets_matrix = {
            //     let limbs = MetalState::retrieve_contents(&buckets_matrix_buffer);
            //     Point::from_flat_u32_limbs(&limbs)
            // };

            //     let limbs = MetalState::retrieve_contents(&buckets_matrix_buffer);
            let buckets_matrix: Vec<GAffine> = MetalState::retrieve_contents(&buckets_matrix_buffer);

            let mut buckets = Vec::with_capacity(buckets_len);

            // TODO: use iterators
            for i in 0..buckets_len {
                let mut partial_sum = buckets_matrix[i].clone();

                for j in 1..point_len {
                    partial_sum = partial_sum.mul_bigint(&buckets_matrix[i + j * buckets_len]).into();
                    // partial_sum = partial_sum.operate_with(&buckets_matrix[i + j * buckets_len]);
                }
                buckets.push(partial_sum);
            }

            buckets
                .iter_mut()
                .rev()
                .scan(Point::identity(), |m, b| {
                    *m = m.mul_bigint(&b).into();
                    // *m = m.operate_with(b); // Reduction step.

                    // TODO: Should cleanup the buffer in the position of b
                    Some(m.clone())
                })
                // .reduce(|g, m| g.operate_with(&m))
                .reduce(|g, m| g.mul_bigint(&m).into())
                .unwrap_or_else(Point::identity)
        })
        .reduce(|t, g| t.operate_with_self(1_u64 << window_size).operate_with(&g))
        .unwrap_or_else(Point::identity))
}

/* 
#[cfg(test)]
mod tests {
    use crate::middleware::gpu_explorations::metal::abstractions::state::MetalState;
    
    const _CASES: u32 = 1;
    const _MAX_WSIZE: usize = 4;
    const _MAX_LEN: usize = 30;
    
    #[test]
    fn test_metal_pippenger_matches_cpu() {
        let state = MetalState::new(None).unwrap();
        let min_len = cs.len().min(hidings.len());
        let cs = cs[..min_len].to_vec();
        let hidings = hidings[..min_len].to_vec();
        
        let cpu_result = pippenger::msm_with(&cs, &hidings, window_size);
        let metal_result = super::pippenger(&cs, &hidings, window_size, &state).unwrap();
    }
}
*/
