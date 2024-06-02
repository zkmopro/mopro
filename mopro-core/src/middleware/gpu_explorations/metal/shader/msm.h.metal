#pragma once

#include "fields/bn254.h.metal"
#include "fields/fp_bn254.h.metal"
#include "fields/unsigned_int.h.metal"

namespace {
    typedef UnsignedInteger<8> u256;
    typedef FpBN254 FE;
    typedef ECPoint<FE, 0> Point;
}

constant constexpr uint32_t NUM_LIMBS = 8;  // u256

[[kernel]] void calculate_buckets(
    constant const uint32_t& _window_idx  [[ buffer(0) ]],
    constant const uint32_t& _window_size  [[ buffer(1) ]],
    constant const u256* k_buff           [[ buffer(2) ]],
    constant const Point* p_buff          [[ buffer(3) ]],
    device Point* buckets_matrix          [[ buffer(4) ]],
    const uint32_t thread_id      [[ thread_position_in_grid ]],
    const uint32_t thread_count   [[ threads_per_grid ]]
)
{
    uint32_t window_idx = _window_idx;
    uint32_t window_size = _window_size;    // c in arkworks code

    u256 k = k_buff[thread_id];
    Point p = p_buff[thread_id];

    uint32_t buckets_len = (1 << window_size) - 1;
    uint32_t window_unmasked = (k >> (window_idx * window_size)).m_limbs[NUM_LIMBS - 1];

    // perform accumulation on window-wise
    uint32_t m_ij = window_unmasked & buckets_len;
    if (m_ij != 0) {
        uint64_t idx = (m_ij - 1);
        Point bucket = buckets_matrix[thread_id * buckets_len + idx];
        buckets_matrix[thread_id * buckets_len + idx] = bucket + p;
    }
}
