#pragma once

#include "unsigned_int.h.metal"


namespace {
    typedef UnsignedInteger<12> u384;
}

/* For bn254, the modulus is "21888242871839275222246405745257275088696311157297823662689037894645226208583" [1]
 * the field type arkworks using is Fp256 using 64*4 bits[2]
 * [1] Reference: https://github.com/arkworks-rs/algebra/blob/065cd24fc5ae17e024c892cee126ad3bd885f01c/curves/bn254/src/fields/fq.rs#L4C14-L4C91
 * [2] https://github.com/arkworks-rs/algebra/blob/master/ff/src/fields/models/fp/mod.rs 
 */
constexpr static const constant u256 N = {
    30644E72E131A029B85045B68181585D97816A916871CA8D3C208C16D87CFD47
};

// don't know this for now
constexpr static const constant u256 R_SQUARED = {
     //
};

// Equates to `(1 << 256) - N`
constexpr static const constant u256 R_SUB_N = {
    //
};

// MU = -N^{-1} mod (2^32)
constexpr static const constant uint64_t MU = 4294770685;

class FpBLS12381 {
public:
    u384 inner;
    constexpr FpBLS12381() = default;
    constexpr FpBLS12381(uint64_t v) : inner{u384::from_int(v)} {}
    constexpr FpBLS12381(u384 v) : inner{v} {}

    constexpr explicit operator u384() const
    {
        return inner;
    }

    constexpr FpBLS12381 operator+(const FpBLS12381 rhs) const
    {
        return FpBLS12381(add(inner, rhs.inner));
    }

    constexpr FpBLS12381 operator-(const FpBLS12381 rhs) const
    {
        return FpBLS12381(sub(inner, rhs.inner));
    }

    constexpr FpBLS12381 operator*(const FpBLS12381 rhs) const
    {
        return FpBLS12381(mul(inner, rhs.inner));
    }

    constexpr bool operator==(const FpBLS12381 rhs) const
    {
        return inner == rhs.inner;
    }

    constexpr bool operator!=(const FpBLS12381 rhs) const
    {
        return !(inner == rhs.inner);
    }

    constexpr explicit operator uint32_t() const
    {
        return inner.m_limbs[11];
    }

    FpBLS12381 operator>>(const uint32_t rhs) const
    {
        return FpBLS12381(inner >> rhs);
    }

    FpBLS12381 operator<<(const uint32_t rhs) const
    {
        return FpBLS12381(inner << rhs);
    }

    constexpr static FpBLS12381 one()
    {
        // TODO find a way to generate on compile time
        const FpBLS12381 ONE = FpBLS12381::mul(u384::from_int((uint32_t) 1), R_SQUARED);
        return ONE;
    }

    constexpr FpBLS12381 to_montgomery()
    {
        return mul(inner, R_SQUARED);
    }

    // TODO: make method for all fields
    FpBLS12381 pow(uint32_t exp) const
    {
        // TODO find a way to generate on compile time
        FpBLS12381 const ONE = one();
        FpBLS12381 res = ONE;
        FpBLS12381 power = *this;

        while (exp > 0)
        {
            if (exp & 1)
            {
                res = res * power;
            }
            exp >>= 1;
            power = power * power;
        }

        return res;
    }

    FpBLS12381 inverse() 
    {
        // used addchain
        // https://github.com/mmcloughlin/addchain
        u384 _10 = mul(inner, inner);
        u384 _11 = mul(_10, inner);
        u384 _1100 = sqn<2>(_11);
        u384 _1101 = mul(inner, _1100);
        u384 _1111 = mul(_10, _1101);
        u384 _11001 = mul(_1100, _1101);
        u384 _110010 = mul(_11001, _11001);
        u384 _110011 = mul(inner, _110010);
        u384 _1000010 = mul(_1111, _110011);
        u384 _1001110 = mul(_1100, _1000010);
        u384 _10000001 = mul(_110011, _1001110);
        u384 _11001111 = mul(_1001110, _10000001);
        u384 i14 = mul(_11001111, _11001111);
        u384 i15 = mul(_10000001, i14);
        u384 i16 = mul(i14, i15);
        u384 x10 = mul(_1000010, i16);
        u384 i27 = sqn<10>(x10);
        u384 i28 = mul(i16, i27);
        u384 i38 = sqn<10>(i27);
        u384 i39 = mul(i28, i38);
        u384 i49 = sqn<10>(i38);
        u384 i50 = mul(i39, i49);
        u384 i60 = sqn<10>(i49);
        u384 i61 = mul(i50, i60);
        u384 i72 = mul(sqn<10>(i60), i61);
        u384 x60 = mul(_1000010, i72);
        u384 i76 = sqn<2>(mul(i72, x60));
        u384 x64 = mul(mul(i15, i76), i76);
        u384 i208 = mul(sqn<64>(mul(sqn<63>(mul(i15, x64)), x64)), x64);
        return FpBLS12381(mul(sqn<60>(i208), x60));
    }

    FpBLS12381 neg()
    {
        // TODO: can improve
        return FpBLS12381(sub(u384::from_int((uint32_t)0), inner));
    }

private:

    template<uint32_t N_ACC>
    u384 sqn(u384 base) const {
        u384 result = base;
#pragma unroll
        for (uint32_t i = 0; i < N_ACC; i++) {
            result = mul(result, result);
        }
        return result;
    }

    // Computes `lhs + rhs mod N`
    // Returns value in range [0,N)
    inline u384 add(const u384 lhs, const u384 rhs) const
    {
        u384 addition = lhs + rhs;
        u384 res = addition;
        // TODO: determine if an if statement here are more optimal

        return res - u384::from_int((uint64_t)(addition >= N)) * N + u384::from_int((uint64_t)(addition < lhs)) * R_SUB_N;
    }

    // Computes `lhs - rhs mod N`
    // Assumes `rhs` value in range [0,N)
    inline u384 sub(const u384 lhs, const u384 rhs) const
    {
        return add(lhs, ((u384)N) - rhs);
    }

    // Computes `lhs * rhs mod M`
    //
    // Essential that inputs are already in the range [0,N) and are in montgomery
    // form. Multiplication performs single round of montgomery reduction.
    //
    // Reference:
    // - https://en.wikipedia.org/wiki/Montgomery_modular_multiplication (REDC)
    // - https://www.youtube.com/watch?v=2UmQDKcelBQ
    constexpr static u384 mul(const u384 a, const u384 b)
    {
        constexpr uint64_t NUM_LIMBS = 12;
        metal::array<uint32_t, NUM_LIMBS> t = {};
        metal::array<uint32_t, 2> t_extra = {};

        u384 q = N;

        uint64_t i = NUM_LIMBS;

        while (i > 0) {
            i -= 1;
            // C := 0
            uint64_t c = 0;

            // for j=0 to N-1
            //    (C,t[j]) := t[j] + a[j]*b[i] + C
            uint64_t cs = 0;
            uint64_t j = NUM_LIMBS;
            while (j > 0) {
                j -= 1;
                cs = (uint64_t)t[j] + (uint64_t)a.m_limbs[j] * (uint64_t)b.m_limbs[i] + c;
                c = cs >> 32;
                t[j] = (uint32_t)((cs << 32) >> 32);
            }

            // (t[N+1],t[N]) := t[N] + C
            cs = (uint64_t)t_extra[1] + c;
            t_extra[0] = (uint32_t)(cs >> 32);
            t_extra[1] = (uint32_t)((cs << 32) >> 32);

            // m := t[0]*q'[0] mod D
            uint64_t m = (((uint64_t)t[NUM_LIMBS - 1] * MU) << 32) >> 32;

            // (C,_) := t[0] + m*q[0]
            c = ((uint64_t)t[NUM_LIMBS - 1] + m * (uint64_t)q.m_limbs[NUM_LIMBS - 1]) >> 32;

            // for j=1 to N-1
            //    (C,t[j-1]) := t[j] + m*q[j] + C

            j = NUM_LIMBS - 1;
            while (j > 0) {
                j -= 1;
                cs = (uint64_t)t[j] + m * (uint64_t)q.m_limbs[j] + c;
                c = cs >> 32;
                t[j + 1] = (uint32_t)((cs << 32) >> 32);
            }

            // (C,t[N-1]) := t[N] + C
            cs = (uint64_t)t_extra[1] + c;
            c = cs >> 32;
            t[0] = (uint32_t)((cs << 32) >> 32);

            // t[N] := t[N+1] + C
            t_extra[1] = t_extra[0] + (uint32_t)c;
        }

        u384 result {t};

        uint64_t overflow = t_extra[0] > 0;
        // TODO: assuming the integer represented by
        // [t_extra[1], t[0], ..., t[NUM_LIMBS - 1]] is at most
        // 2q in any case.
        if (overflow || q <= result) {
            result = result - q;
        }

        return result;
    }
};
