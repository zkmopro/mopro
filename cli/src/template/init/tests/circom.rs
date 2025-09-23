mopro_ffi::uniffi_setup!();

#[cfg(target_os = "macos")]
uniffi::build_foreign_language_testcases!("tests/bindings/circom/test_multiplier2.swift",);

uniffi::build_foreign_language_testcases!("tests/bindings/circom/test_multiplier2.kts",);
