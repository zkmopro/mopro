mopro_ffi::uniffi_setup!();

uniffi::build_foreign_language_testcases!(
    "tests/bindings/test_circom_multiplier2.swift",
    "tests/bindings/test_circom_multiplier2.kts",
    "tests/bindings/test_circom_keccak.swift",
    "tests/bindings/test_circom_keccak.kts",
    "tests/bindings/test_circom_multiplier2_bls.swift",
    "tests/bindings/test_circom_multiplier2_bls.kts",
);
