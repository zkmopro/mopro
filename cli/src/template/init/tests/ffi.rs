mopro_ffi::uniffi_setup!();

#[cfg(target_os = "macos")]
uniffi::build_foreign_language_testcases!("tests/bindings/ffi/test_ffi_hello_world.swift",);

uniffi::build_foreign_language_testcases!("tests/bindings/ffi/test_ffi_hello_world.kts",);
