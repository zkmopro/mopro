#[cfg(feature = "circom")]
uniffi::build_foreign_language_testcases!(
    "tests/bindings/test_circom_multiplier2.swift",
    "tests/bindings/test_circom_multiplier2.kts",
    "tests/bindings/test_circom_keccak.swift",
    "tests/bindings/test_circom_keccak.kts",
    "tests/bindings/test_circom_multiplier2_bls.swift",
    "tests/bindings/test_circom_multiplier2_bls.kts",
);

#[cfg(feature = "halo2")]
uniffi::build_foreign_language_testcases!(
    "tests/bindings/test_halo2_fibonacci.swift",
    "tests/bindings/test_halo2_fibonacci.kts",
);
