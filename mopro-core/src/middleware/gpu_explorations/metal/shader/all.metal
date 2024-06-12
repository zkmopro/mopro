// This is necessary because pragma once doesn't work as expected
// and some symbols are being duplicated.

// TODO: Investigate this issue, having .metal sources would be better
// than headers and a unique source.

#include "fields/fp_bn254.h.metal"
#include "tests/test_bn254.h.metal"
#include "tests/test_unsigned_integer.h.metal"
// #include "utils/parallel_radix_sort.h.metal"
#include "msm.h.metal"
