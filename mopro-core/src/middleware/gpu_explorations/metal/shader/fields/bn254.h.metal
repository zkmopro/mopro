#pragma once

#include "fp_bn254.h.metal"
#include "ec_point.h.metal"
#include "../tests/test_bn254.h.metal"

namespace {
    typedef ECPoint<FpBN254, 0> BN254;
    typedef UnsignedInteger<8> u256;
}

template [[ host_name("fp_bn254_add") ]]
[[kernel]] void bn254_add<FpBN254>(
    constant FpBN254&,
    constant FpBN254&,
    device FpBN254&
);

template [[ host_name("fp_bn254_sub") ]]
[[kernel]] void bn254_sub<FpBN254>(
    constant FpBN254&,
    constant FpBN254&,
    device FpBN254&
);

template [[ host_name("fp_bn254_mul") ]]
[[kernel]] void bn254_mul<FpBN254>(
    constant FpBN254&,
    constant FpBN254&,
    device FpBN254&
);

template [[ host_name("fp_bn254_pow") ]]
[[kernel]] void bn254_pow<FpBN254>(
    constant FpBN254&,
    constant uint32_t&,
    device FpBN254&
);
