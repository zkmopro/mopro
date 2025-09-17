mopro_ffi::uniffi_setup!();

uniffi::build_foreign_language_testcases!(
    "tests/bindings/halo2/test_plonk_fibonacci.swift",
    "tests/bindings/halo2/test_plonk_fibonacci.kts",
    "tests/bindings/halo2/test_hyperplonk_fibonacci.swift",
    "tests/bindings/halo2/test_hyperplonk_fibonacci.kts",
    "tests/bindings/halo2/test_gemini_fibonacci.swift",
    "tests/bindings/halo2/test_gemini_fibonacci.kts"
);
