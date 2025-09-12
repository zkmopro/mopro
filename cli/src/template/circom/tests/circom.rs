mopro_ffi::uniffi_setup!();

uniffi::build_foreign_language_testcases!(
    "tests/bindings/test_multiplier2.swift",
    "tests/bindings/test_multiplier2.kts",
    "tests/bindings/test_multiplier2_bls.swift",
    "tests/bindings/test_multiplier2_bls.kts",
);
