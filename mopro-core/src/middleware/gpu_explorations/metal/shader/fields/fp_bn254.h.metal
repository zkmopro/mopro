#pragma once

#include "unsigned_int.h.metal"


namespace {
    // 8 limbs of 32 bits uint 
    typedef UnsignedInteger<8> u256;
}

/* Constants for bn254 field operations
 * N: base field modulus
 * R_SQUARED: R^2 mod N
 * R_SUB_N: R - N
 * MU: Montgomery Multiplication Constant = -N^{-1} mod (2^32)
 *
 * For bn254, the modulus is "21888242871839275222246405745257275088696311157297823662689037894645226208583" [1, 2]
 * We use 8 limbs of 32 bits unsigned integers to represent the constanst
 *
 * References:
 * [1] https://github.com/arkworks-rs/algebra/blob/065cd24fc5ae17e024c892cee126ad3bd885f01c/curves/bn254/src/lib.rs
 * [2] https://github.com/scipr-lab/libff/blob/develop/libff/algebra/curves/alt_bn128/alt_bn128.sage
 */

constexpr static const constant u256 N = {
    0x30644E72, 0xE131A029,
    0xB85045B6, 0x8181585D,
    0x97816A91, 0x6871CA8D,
    0x3C208C16, 0xD87CFD47
};

constexpr static const constant u256 R_SQUARED = {
    0x06D89F71, 0xCAB8351F,
    0x47AB1EFF, 0x0A417FF6,
    0xB5E71911, 0xD44501FB,
    0xF32CFC5B, 0x538AFA89
};

constexpr static const constant u256 R_SUB_N = {
    0xCF9BB18D, 0x1ECE5FD6,
    0x47AFBA49, 0x7E7EA7A2,
    0x687E956E, 0x978E3572,
    0xC3DF73E9, 0x278302B9
};

constexpr static const constant uint64_t MU = 0xE4866389FFFFFFFF;

class FpBN254 {
public:
    u256 inner;
    constexpr FpBN254() = default;
    constexpr FpBN254(uint64_t v) : inner{u256::from_int(v)} {}
    constexpr FpBN254(u256 v) : inner{v} {}

    constexpr explicit operator u256() const {
        return inner;
    }

    constexpr FpBN254 operator+(const FpBN254 rhs) const {
        return FpBN254(add(inner, rhs.inner));
    }

    constexpr FpBN254 operator-(const FpBN254 rhs) const {
        return FpBN254(sub(inner, rhs.inner));
    }

    constexpr FpBN254 operator*(const FpBN254 rhs) const {
        return FpBN254(mul(inner, rhs.inner));
    }

    constexpr bool operator==(const FpBN254 rhs) const {
        return inner == rhs.inner;
    }

    constexpr bool operator!=(const FpBN254 rhs) const {
        return !(inner == rhs.inner);
    }

    constexpr explicit operator uint32_t() const {
        return inner.m_limbs[7];
    }

    FpBN254 operator>>(const uint32_t rhs) const {
        return FpBN254(inner >> rhs);
    }

    FpBN254 operator<<(const uint32_t rhs) const {
        return FpBN254(inner << rhs);
    }

    constexpr static FpBN254 one() {
        const FpBN254 ONE = FpBN254::mul(u256::from_int((uint32_t) 1), R_SQUARED);
        return ONE;
    }

    constexpr FpBN254 to_montgomery() {
        return mul(inner, R_SQUARED);
    }

    FpBN254 pow(uint32_t exp) const {
        FpBN254 const ONE = one();
        FpBN254 res = ONE;
        FpBN254 power = *this;

        while (exp > 0) {
            if (exp & 1) {
                res = res * power;
            }
            exp >>= 1;
            power = power * power;
        }

        return res;
    }

    FpBN254 inverse() {
        // Using an addition chain for inversion
        // https://github.com/mmcloughlin/addchain

        u256 _10 = mul(inner, inner);
        u256 _11 = mul(_10, inner);
        u256 _1100 = sqn<2>(_11);
        u256 _1101 = mul(inner, _1100);
        u256 _1111 = mul(_10, _1101);
        u256 _11001 = mul(_1100, _1101);
        u256 _110010 = mul(_11001, _11001);
        u256 _110011 = mul(inner, _110010);
        u256 _1000010 = mul(_1111, _110011);
        u256 _1001110 = mul(_1100, _1000010);
        u256 _10000001 = mul(_110011, _1001110);
        u256 _11001111 = mul(_1001110, _10000001);
        u256 i14 = mul(_11001111, _11001111);
        u256 i15 = mul(_10000001, i14);
        u256 i16 = mul(i14, i15);
        u256 x10 = mul(_1000010, i16);
        u256 i27 = sqn<10>(x10);
        u256 i28 = mul(i16, i27);
        u256 i38 = sqn<10>(i27);
        u256 i39 = mul(i28, i38);
        u256 i49 = sqn<10>(i38);
        u256 i50 = mul(i39, i49);
        u256 i60 = sqn<10>(i49);
        u256 i61 = mul(i50, i60);
        u256 i72 = mul(sqn<10>(i60), i61);
        u256 x60 = mul(_1000010, i72);
        u256 i76 = sqn<2>(mul(i72, x60));
        u256 x64 = mul(mul(i15, i76), i76);
        u256 i208 = mul(sqn<64>(mul(sqn<63>(mul(i15, x64)), x64)), x64);
        return FpBN254(mul(sqn<60>(i208), x60));
    }

    FpBN254 neg() {
        return FpBN254(sub(u256::from_int((uint32_t)0), inner));
    }

private:
    template<uint32_t N_ACC>
    u256 sqn(u256 base) const {
        u256 result = base;
#pragma unroll
        for (uint32_t i = 0; i < N_ACC; i++) {
            result = mul(result, result);
        }
        return result;
    }

    inline u256 add(const u256 lhs, const u256 rhs) const {
        u256 addition = lhs + rhs;
        u256 res = addition;

        return res - u256::from_int((uint64_t)(addition >= N)) * N + u256::from_int((uint64_t)(addition < lhs)) * R_SUB_N;
    }

    inline u256 sub(const u256 lhs, const u256 rhs) const {
        return add(lhs, ((u256)N) - rhs);
    }

    constexpr static u256 mul(const u256 a, const u256 b) {
        constexpr uint64_t NUM_LIMBS = 8;
        metal::array<uint32_t, NUM_LIMBS> t = {};
        metal::array<uint32_t, 2> t_extra = {};

        u256 q = N;

        uint64_t i = NUM_LIMBS;

        while (i > 0) {
            i -= 1;
            uint64_t c = 0;

            uint64_t cs = 0;
            uint64_t j = NUM_LIMBS;
            while (j > 0) {
                j -= 1;
                cs = (uint64_t)t[j] + (uint64_t)a.m_limbs[j] * (uint64_t)b.m_limbs[i] + c;
                c = cs >> 32;
                t[j] = (uint32_t)((cs << 32) >> 32);
            }

            cs = (uint64_t)t_extra[1] + c;
            t_extra[0] = (uint32_t)(cs >> 32);
            t_extra[1] = (uint32_t)((cs << 32) >> 32);

            uint64_t m = (((uint64_t)t[NUM_LIMBS - 1] * MU) << 32) >> 32;

            c = ((uint64_t)t[NUM_LIMBS - 1] + m * (uint64_t)q.m_limbs[NUM_LIMBS - 1]) >> 32;

            j = NUM_LIMBS - 1;
            while (j > 0) {
                j -= 1;
                cs = (uint64_t)t[j] + m * (uint64_t)q.m_limbs[j] + c;
                c = cs >> 32;
                t[j + 1] = (uint32_t)((cs << 32) >> 32);
            }

            cs = (uint64_t)t_extra[1] + c;
            c = cs >> 32;
            t[0] = (uint32_t)((cs << 32) >> 32);

            t_extra[1] = t_extra[0] + (uint32_t)c;
        }

        u256 result {t};

        uint64_t overflow = t_extra[0] > 0;
        if (overflow || q <= result) {
            result = result - q;
        }

        return result;
    }
};
