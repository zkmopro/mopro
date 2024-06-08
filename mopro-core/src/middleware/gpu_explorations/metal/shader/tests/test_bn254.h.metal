#pragma once

#include "../fields/fp_bn254.h.metal"

using namespace metal;

template<typename BN254, typename Fp>
[[kernel]] void bn254_add(
    constant Fp* p [[ buffer(0) ]],
    constant Fp* q [[ buffer(1) ]],
    device Fp* result [[ buffer(2) ]]
)
{
    BN254 P = BN254(p[0], p[1], p[2]);
    BN254 Q = BN254(q[0], q[1], q[2]);
    BN254 res = P + Q;

    result[0] = res.x;
    result[1] = res.y;
    result[2] = res.z;
}

template<typename Fp>
[[kernel]] void fp_bn254_add(
    constant FpBN254& _p [[ buffer(0) ]],
    constant FpBN254& _q [[ buffer(1) ]],
    device FpBN254& result [[ buffer(2) ]]
) {
    FpBN254 p = _p;
    FpBN254 q = _q;
    result = p + q;
}

template<typename Fp>
[[kernel]] void fp_bn254_sub(
    constant FpBN254 &_p [[ buffer(0) ]],
    constant FpBN254 &_q [[ buffer(1) ]],
    device FpBN254 &result [[ buffer(2) ]]
) {
    FpBN254 p = _p;
    FpBN254 q = _q;
    result = p - q;
}

template<typename Fp>
[[kernel]] void fp_bn254_mul(
    constant FpBN254 &_p [[ buffer(0) ]],
    constant FpBN254 &_q [[ buffer(1) ]],
    device FpBN254 &result [[ buffer(2) ]]
) {
    FpBN254 p = _p;
    FpBN254 q = _q;
    result = p * q;
}

template<typename Fp>
[[kernel]] void fp_bn254_pow(
    constant FpBN254 &_p [[ buffer(0) ]],
    constant uint32_t &_a [[ buffer(1) ]],
    device FpBN254 &result [[ buffer(2) ]]
) {
    FpBN254 p = _p;
    result = p.pow(_a);
}

template<typename Fp>
[[kernel]] void fp_bn254_neg(
    constant FpBN254 &_p [[ buffer(0) ]],
    constant uint32_t &_a [[ buffer(1) ]],  // TODO: Remove this dummy arg
    device FpBN254 &result [[ buffer(2) ]]
) {
    FpBN254 p = _p;
    result = p.neg();
}

// // TODO: Implement inverse if needed in the future
// [[kernel]] void fp_bn254_inv(
//     constant FpBN254 &_p [[ buffer(0) ]],
//     constant FpBN254 &_q [[ buffer(1) ]],
//     device FpBN254 &result [[ buffer(2) ]]
// ) {
//     FpBN254 p = _p;
//     FpBN254 inv_p = p.inverse();
//     result = inv_p;
// }
