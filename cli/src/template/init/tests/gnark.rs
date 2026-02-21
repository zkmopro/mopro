mopro_ffi::uniffi_setup!();

#[cfg(target_os = "macos")]
uniffi::build_foreign_language_testcases!("tests/bindings/gnark/test_gnark_cubic.swift",);

uniffi::build_foreign_language_testcases!("tests/bindings/gnark/test_gnark_cubic.kts",);
