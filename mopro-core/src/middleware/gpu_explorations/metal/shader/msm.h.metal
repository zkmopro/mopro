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
    device Point* buckets          [[ buffer(4) ]],
    device uint32_t* debugBufferK [[ buffer(5) ]],
    device uint32_t* debugBufferP [[ buffer(6) ]],

    const uint32_t thread_id      [[ thread_position_in_grid ]],
    const uint32_t thread_count   [[ threads_per_grid ]]
)
{
    // Iterate over the scalars and points
    uint32_t window_idx = _window_idx;
    uint32_t window_size = _window_size;
    uint32_t buckets_len = (1 << window_size) - 1;

    u256 scalar = k_buff[thread_id];
    Point point = p_buff[thread_id];


    uint32_t window_unmasked = (scalar >> (window_idx * window_size)).m_limbs[NUM_LIMBS - 1];
    uint32_t m_ij = window_unmasked & buckets_len;
    // // Write the debug values
    // if (thread_id == 0) {
    //     for (uint32_t i = 0; i < NUM_LIMBS; i++) {
    //         debugBufferK[i] = window_unmasked;
    //         debugBufferP[i] = m_ij;
    //     }
    // }
    if (m_ij != 0) {
        uint64_t idx = (m_ij - 1);
        Point bucket = buckets[idx];
        buckets[idx] = bucket + point;
        // Write the debug values
        if (thread_id == 0) {
            for (uint32_t i = 0; i < NUM_LIMBS; i++) {
                debugBufferK[i] = buckets[idx].x.inner.m_limbs[i];
                debugBufferP[i] = buckets[idx].z.inner.m_limbs[i];
            }
        }
    }

    /*
    uint32_t window_idx = _window_idx;
    uint32_t window_size = _window_size;

    u256 k = k_buff[thread_id];
    Point p = p_buff[thread_id];

    uint32_t buckets_len = (1 << window_size) - 1;

    uint32_t window_unmasked = (k >> (window_idx * window_size)).m_limbs[NUM_LIMBS - 1];
    uint32_t m_ij = window_unmasked & buckets_len;
    if (m_ij != 0) {
        uint64_t idx = (m_ij - 1);
        Point bucket = buckets[thread_id * buckets_len + idx];
        buckets[thread_id * buckets_len + idx] = bucket + p;
    }
    */
}
