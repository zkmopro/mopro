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
    constant const uint32_t& _window_size       [[ buffer(0) ]],
    constant const uint32_t& _instances_size    [[ buffer(1) ]],
    constant const uint32_t* _window_starts     [[ buffer(2) ]],
    constant const u256* k_buff                 [[ buffer(3) ]],
    constant const Point* p_buff                [[ buffer(4) ]],
    device Point* buckets_matrix                [[ buffer(5) ]],
    const uint32_t thread_id                    [[ thread_position_in_grid ]],
    const uint32_t thread_count                 [[ threads_per_grid ]]
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
            Point bucket = buckets_matrix[window_idx + idx];
            buckets_matrix[window_idx + idx] = bucket + p;
        }
    }
}

[[kernel]] void initialize_buckets(
    constant const uint32_t& _window_size       [[ buffer(0) ]],
    constant const uint32_t* _window_starts     [[ buffer(1) ]],
    device Point* buckets_matrix                [[ buffer(2) ]],
    const uint32_t thread_id                    [[ thread_position_in_grid ]]
)
{
    uint32_t window_size = _window_size;    // c in arkworks code
    uint32_t window_idx = _window_starts[thread_id];
    uint32_t buckets_len = (1 << window_size) - 1;

    for (uint32_t i = 0; i < buckets_len; i++) {
        buckets_matrix[window_idx + i] = Point::neutral_element();
    }
}

[[kernel]] void accumulation_and_reduction_phase(
    constant const uint32_t& _window_size       [[ buffer(0) ]],
    constant const uint32_t& _instances_size    [[ buffer(1) ]],
    constant const uint32_t* _window_starts     [[ buffer(2) ]],
    constant const u256* k_buff                 [[ buffer(3) ]],
    constant const Point* p_buff                [[ buffer(4) ]],
    device Point* buckets_matrix                [[ buffer(5) ]],
    device Point* res                           [[ buffer(6) ]],
    const uint32_t thread_id                    [[ thread_position_in_grid ]],
    const uint32_t thread_count                 [[ threads_per_grid ]]
)
{
    uint32_t window_size = _window_size;    // c in arkworks code
    uint32_t instances_size = _instances_size;
    uint32_t buckets_len = (1 << window_size) - 1;
    uint32_t window_idx = _window_starts[thread_id];

    u256 one = u256::from_int((uint32_t)1);
    res[thread_id] = Point::neutral_element();

    // for each points and scalars, calculate the bucket index and accumulate
    for (uint32_t i = 0; i < instances_size; i++) {
        u256 k = k_buff[i];
        Point p = p_buff[i];
        // pass if k is one
        if (k == one) {
            if (window_idx == 0) {
                Point tmp = res[thread_id];
                res[thread_id] = tmp + p_buff[i];
            }
        }
        else {
            // move the b-bit scalar to the correct c-bit scalar fragment
            uint32_t scalar_fragment = (k >> window_idx).m_limbs[NUM_LIMBS - 1];
            // truncate the scalar fragment to the window size
            uint32_t m_ij = scalar_fragment & buckets_len;

            if (m_ij != 0) {
                uint32_t idx = m_ij - 1;
                Point bucket = buckets_matrix[window_idx + idx];
                buckets_matrix[window_idx + idx] = bucket + p;
            }
        }
    }

    // Perform sum reduction on buckets
    // Get the last bucket of this window
    uint32_t last_bucket_idx = window_idx + buckets_len - 1;

    Point running_sum = Point::neutral_element();
    for (uint32_t i = 0; i < buckets_len; i++) {
        Point bucket = buckets_matrix[last_bucket_idx - i];

        Point tmp1 = running_sum;
        running_sum = tmp1 + bucket;

        Point tmp2 = res[thread_id];
        res[thread_id] = tmp2 + running_sum;
    }
}

[[visible]] Point final_accumulation(
    constant const uint32_t& _window_size       [[ buffer(0) ]],
    constant const uint32_t* _window_starts     [[ buffer(1) ]],
    constant const uint32_t& _num_windows       [[ buffer(2) ]],
    device Point* buckets_matrix                [[ buffer(3) ]],
    device Point* res                           [[ buffer(4) ]]
)
{
    uint32_t window_size = _window_size;    // c in arkworks code
    uint32_t num_windows = _num_windows;
    Point lowest_window_sum = res[0];

    Point total_sum = Point::neutral_element();
    for (uint32_t i = 1; i < num_windows; i++) {
        Point tmp = total_sum;
        total_sum = tmp + res[i];

        for (uint32_t j = 0; j < window_size; j++) {
            total_sum = total_sum.double_in_place();
        }
    }
    total_sum = total_sum + lowest_window_sum;
    return total_sum;
}
