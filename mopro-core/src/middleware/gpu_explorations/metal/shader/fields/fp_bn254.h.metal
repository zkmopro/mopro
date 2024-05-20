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
        // Generate by the command: addchain search '21888242871839275222246405745257275088696311157297823662689037894645226208583 - 2'
        // https://github.com/mmcloughlin/addchain

        // addchain: expr: "21888242871839275222246405745257275088696311157297823662689037894645226208583 - 2"
        // addchain: hex: 30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd45
        // addchain: dec: 21888242871839275222246405745257275088696311157297823662689037894645226208581
        // addchain: best: opt(dictionary(sliding_window(8),heuristic(use_first(halving,delta_largest))))
        // addchain: cost: 303
        // _10       = 2*1
        // _11       = 1 + _10
        // _101      = _10 + _11
        // _110      = 1 + _101
        // _1000     = _10 + _110
        // _1101     = _101 + _1000
        // _10010    = _101 + _1101
        // _10011    = 1 + _10010
        // _10100    = 1 + _10011
        // _10111    = _11 + _10100
        // _11100    = _101 + _10111
        // _100000   = _1101 + _10011
        // _100011   = _11 + _100000
        // _101011   = _1000 + _100011
        // _101111   = _10011 + _11100
        // _1000001  = _10010 + _101111
        // _1010011  = _10010 + _1000001
        // _1011011  = _1000 + _1010011
        // _1100001  = _110 + _1011011
        // _1110101  = _10100 + _1100001
        // _10010001 = _11100 + _1110101
        // _10010101 = _100000 + _1110101
        // _10110101 = _100000 + _10010101
        // _10111011 = _110 + _10110101
        // _11000001 = _110 + _10111011
        // _11000011 = _10 + _11000001
        // _11010011 = _10010 + _11000001
        // _11100001 = _100000 + _11000001
        // _11100011 = _10 + _11100001
        // _11100111 = _110 + _11100001
        // i57       = ((_11000001 << 8 + _10010001) << 10 + _11100111) << 7
        // i76       = ((_10111 + i57) << 9 + _10011) << 7 + _1101
        // i109      = ((i76 << 14 + _1010011) << 9 + _11100001) << 8
        // i127      = ((_1000001 + i109) << 10 + _1011011) << 5 + _1101
        // i161      = ((i127 << 8 + _11) << 12 + _101011) << 12
        // i186      = ((_10111011 + i161) << 8 + _101111) << 14 + _10110101
        // i214      = ((i186 << 9 + _10010001) << 5 + _1101) << 12
        // i236      = ((_11100011 + i214) << 8 + _10010101) << 11 + _11010011
        // i268      = ((i236 << 7 + _1100001) << 11 + _100011) << 12
        // i288      = ((_1011011 + i268) << 9 + _11000011) << 8 + _11100111
        // return      (i288 << 7 + _1110101) << 6 + _101

        u256 _10 = mul(inner, inner);
        u256 _11 = mul(_10, inner);
        u256 _101 = mul(_10, _11);
        u256 _110 = mul(inner, _101);
        u256 _1000 = mul(_10, _110);
        u256 _1101 = mul(_101, _1000);
        u256 _10010 = mul(_101, _1101);
        u256 _10011 = mul(inner, _10010);
        u256 _10100 = mul(inner, _10011);
        u256 _10111 = mul(_11, _10100);
        u256 _11100 = mul(_101, _10111);
        u256 _100000 = mul(_1101, _10011);
        u256 _100011 = mul(_11, _100000);
        u256 _101011 = mul(_1000, _100011);
        u256 _101111 = mul(_10011, _11100);
        u256 _1000001 = mul(_10010, _101111);
        u256 _1010011 = mul(_10010, _1000001);
        u256 _1011011 = mul(_1000, _1010011);
        u256 _1100001 = mul(_110, _1011011);
        u256 _1110101 = mul(_10100, _1100001);
        u256 _10010001 = mul(_11100, _1110101);
        u256 _10010101 = mul(_100000, _1110101);
        u256 _10110101 = mul(_100000, _10010101);
        u256 _10111011 = mul(_110, _10110101);
        u256 _11000001 = mul(_110, _10111011);
        u256 _11000011 = mul(_10, _11000001);
        u256 _11010011 = mul(_10010, _11000001);
        u256 _11100001 = mul(_100000, _11000001);
        u256 _11100011 = mul(_10, _11100001);
        u256 _11100111 = mul(_110, _11100001);
        u256 i57 = sqn<7>(mul(sqn<10>(mul(sqn<8>(_11000001),_10010001)),_11100111));
        u256 i76 = mul(sqn<7>(mul(sqn<9>(mul(_10111,i57)),_10011)), _10011);
        u256 i109 = sqn<8>(mul(sqn<9>(mul(sqn<14>(i76),_1010011)),_11100001));
        u256 i127 = mul(sqn<5>(mul(sqn<10>(mul(_1000001,i109)),_1011011)),_1101);
        u256 i161 = sqn<12>(mul(sqn<12>(mul(sqn<8>(i127),_11)),_101011));
        u256 i186 = mul(sqn<14>(mul(sqn<8>(mul(_10111011,i161)),_101111)),_10110101);
        u256 i214 = sqn<12>(mul(sqn<5>(mul(sqn<9>(i186),_10010001)),_1101));
        u256 i236 = mul(sqn<11>(mul(sqn<8>(mul(_11100011,i214)),_10010101)),_11010011);
        u256 i268 = sqn<12>(mul(sqn<11>(mul(sqn<7>(i236),_1100001)),_100011));
        u256 i288 = mul(sqn<8>(mul(sqn<9>(mul(_1011011,i268)),_11000011)),_11100111);
        return FpBN254(mul(sqn<6>(mul(sqn<7>(i288),_1110101)),_101));
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
