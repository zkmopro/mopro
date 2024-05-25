#pragma once

#include "../fields/unsigned_int.h.metal"

namespace {
    typedef UnsignedInteger<8> u256;
}

[[kernel]]
void test_uint_add(
    constant u256& _a [[ buffer(0) ]],
    constant u256& _b [[ buffer(1) ]],
    device u256& result [[ buffer(2) ]],
    device uint32_t* debugBuffer [[ buffer(3) ]])
{
    u256 a = _a;
    u256 b = _b;

    // for each limb, reverse the order of the limbs
    // so that the most significant limb is at the start of the buffer
    // and the least significant limb is at the end of the buffer

    result = a + b;

    // Write the values of a and b to the debug buffer
    for (int i = 0; i < 8; ++i) {
        debugBuffer[i] = a.m_limbs[i];
        debugBuffer[i + 8] = b.m_limbs[i];
        debugBuffer[i + 16] = result.m_limbs[i];
    }
}

[[kernel]]
void test_uint_sub(
    constant u256& _a [[ buffer(0) ]],
    constant u256& _b [[ buffer(1) ]],
    device u256& result [[ buffer(2) ]])
{
    u256 a = _a;
    u256 b = _b;

    result = a - b;
}

[[kernel]]
void test_uint_prod(
    constant u256& _a [[ buffer(0) ]],
    constant u256& _b [[ buffer(1) ]],
    device u256& result [[ buffer(2) ]])
{
    u256 a = _a;
    u256 b = _b;

    result = a * b;
}

[[kernel]]
void test_uint_shl(
    constant u256& _a [[ buffer(0) ]],
    constant uint64_t& _b [[ buffer(1) ]],
    device u256& result [[ buffer(2) ]])
{
    u256 a = _a;
    uint64_t b = _b;

    result = a << b;
}

[[kernel]]
void test_uint_shr(
    constant u256& _a [[ buffer(0) ]],
    constant uint64_t& _b [[ buffer(1) ]],
    device u256& result [[ buffer(2) ]])
{
    u256 a = _a;
    uint64_t b = _b;

    result = a >> b;
}
