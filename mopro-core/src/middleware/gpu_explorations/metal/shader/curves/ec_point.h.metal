#pragma once

template<typename Fp, const uint64_t A_CURVE>
class ECPoint {
public:
    Fp x;
    Fp y;
    Fp z;

    constexpr ECPoint() : ECPoint(ECPoint::neutral_element()) {}
    constexpr ECPoint(Fp _x, Fp _y, Fp _z) : x(_x), y(_y), z(_z) {}

    constexpr ECPoint operator+(const ECPoint other) const {
        if (is_neutral_element(*this)) {
            return other;
        }
        if (is_neutral_element(other)) {
            return *this;
        }

        // Z1Z1 = Z1^2
        Fp z1z1 = z * z;

        // Z2Z2 = Z2^2
        Fp z2z2 = other.z * other.z;

        // U1 = X1 * Z2Z2
        Fp u1 = x * z2z2;

        // U2 = X2 * Z1Z1
        Fp u2 = other.x * z1z1;

        // S1 = Y1 * Z2 * Z2Z2
        Fp s1 = y * other.z * z2z2;

        // S2 = Y2 * Z1 * Z1Z1
        Fp s2 = other.y * z * z1z1;

        if (u1 == u2 && s1 == s2) {
            // The points are equal, so we double
            return double_in_place();
        }

        // H = U2 - U1
        Fp h = u2 - u1;

        // I = (2 * H)^2
        Fp i = (h + h) * (h + h);

        // J = H * I
        Fp j = h * i;

        // r = 2 * (S2 - S1)
        Fp r = (s2 - s1) + (s2 - s1);

        // V = U1 * I
        Fp v = u1 * i;

        // X3 = r^2 - J - 2 * V
        Fp x3 = r * r - j - (v + v);

        // Y3 = r * (V - X3) - 2 * S1 * J
        Fp y3 = r * (v - x3) - (s1 + s1) * j;

        // Z3 = (Z1 + Z2)^2 - Z1Z1 - Z2Z2) * H
        Fp z3 = ((z + other.z) * (z + other.z) - z1z1 - z2z2) * h;

        return ECPoint(x3, y3, z3);
    }

    void operator+=(const ECPoint other) {
        *this = *this + other;
    }

    static ECPoint neutral_element() {
        return ECPoint(Fp(1), Fp(1), Fp(0)); // Updated to new neutral element (1, 1, 0)
    }

    ECPoint operate_with_self(uint64_t exponent) const {
        ECPoint result = neutral_element();
        ECPoint base = ECPoint(x, y, z);

        while (exponent > 0) {
            if ((exponent & 1) == 1) {
                result = result + base;
            }
            exponent = exponent >> 1;
            base = base + base;
        }

        return result;
    }

    constexpr ECPoint operator*(uint64_t exponent) const {
        return operate_with_self(exponent);
    }

    constexpr void operator*=(uint64_t exponent) {
        *this = operate_with_self(exponent);
    }

    constexpr ECPoint neg() const {
        return ECPoint(x, y.neg(), z);
    }

    constexpr bool is_neutral_element(const ECPoint a_point) const {
        return a_point.z == Fp(0); // Updated to check for (1, 1, 0)
    }

    constexpr ECPoint double_in_place() const {
        if (is_neutral_element(*this)) {
            return *this;
        }

        // Doubling formulas
        Fp a_fp = Fp(A_CURVE).to_montgomery();
        Fp two = Fp(2).to_montgomery();
        Fp three = Fp(3).to_montgomery();

        Fp eight = Fp(8).to_montgomery();

        Fp xx = x * x; // x^2
        Fp yy = y * y; // y^2
        Fp yyyy = yy * yy; // y^4
        Fp zz = z * z; // z^2

        // S = 2 * ((X1 + YY)^2 - XX - YYYY)
        Fp s = two * (((x + yy) * (x + yy)) - xx - yyyy);

        // M = 3 * XX + a * ZZ ^ 2
        Fp m = (three * xx) + (a_fp * (zz * zz));

        // X3 = T = M^2 - 2*S
        Fp x3 = (m * m) - (two * s);

        // Z3 = (Y + Z) ^ 2 - YY - ZZ
        // or Z3 = 2 * Y * Z
        Fp z3 = two * y * z;

        // Y3 = M*(S-X3)-8*YYYY
        Fp y3 = m * (s - x3) - eight * yyyy;

        return ECPoint(x3, y3, z3);
    }
};
