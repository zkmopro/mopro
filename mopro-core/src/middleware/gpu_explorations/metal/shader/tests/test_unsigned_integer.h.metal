#pragma once

#include "../fields/unsigned_int.h.metal"

namespace {
    typedef UnsignedInteger<8> u256;
}

[[kernel]]
void test_uint_add(
    constant u256& _a [[ buffer(0) ]],
    constant u256& _b [[ buffer(1) ]],
    device u256& result [[ buffer(2) ]])
{
    u256 a = _a;
    u256 b = _b;

    result = a + b;
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
