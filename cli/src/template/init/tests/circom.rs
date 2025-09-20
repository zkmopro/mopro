mopro_ffi::uniffi_setup!();

uniffi::build_foreign_language_testcases!(
    "tests/bindings/circom/test_multiplier2.swift",
    "tests/bindings/circom/test_multiplier2.kts",
);
