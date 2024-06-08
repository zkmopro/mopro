#pragma once

#include "curves/bn254.h.metal"
#include "fields/fp_bn254.h.metal"
#include "arithmetics/unsigned_int.h.metal"

namespace {
    typedef UnsignedInteger<8> u256;
    typedef FpBN254 FieldElement;
    typedef ECPoint<FieldElement, 0> Point;
}

constant constexpr uint32_t NUM_LIMBS = 8;  // u256

[[kernel]] void accumulation_in_window_wise(
    constant const uint32_t& _window_size  [[ buffer(0) ]],
    constant const uint32_t& _instances_size  [[ buffer(1) ]],
    constant const uint32_t* _window_starts  [[ buffer(2) ]],
    constant const u256* k_buff           [[ buffer(3) ]],
    constant const Point* p_buff          [[ buffer(4) ]],
    device Point* buckets_matrix          [[ buffer(5) ]],
    const uint32_t thread_id      [[ thread_position_in_grid ]],
    const uint32_t thread_count   [[ threads_per_grid ]]
)
{
    uint32_t window_size = _window_size;    // c in arkworks code
    uint32_t instances_size = _instances_size;
    uint32_t buckets_len = (1 << window_size) - 1;
    
    uint32_t window_idx = _window_starts[thread_id];

    u256 one = u256::from_int((uint32_t)1);

    // for each points and scalars, calculate the bucket index and accumulate
    for (uint32_t i = 0; i < instances_size; i++) {
        u256 k = k_buff[i];
        // pass if k is one
        if (k == one) { continue; }

        Point p = p_buff[i];

        // move the b-bit scalar to the correct c-bit scalar fragment
        uint32_t scalar_fragment = (k >> window_idx).m_limbs[NUM_LIMBS - 1];
        // truncate the scalar fragment to the window size
        uint32_t m_ij = scalar_fragment & buckets_len;

        if (m_ij != 0) {
            uint32_t idx = m_ij - 1;
            Point bucket = buckets_matrix[thread_id * buckets_len + idx];
            buckets_matrix[thread_id * buckets_len + idx] = bucket + p;
        }
    }
}
