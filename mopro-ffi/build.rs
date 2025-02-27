fn main() {
    #[cfg(feature = "circom")]
    {
        if std::env::var("MOPRO_FFI_LINK_TEST_WITNESS").unwrap_or_default() != "" {
            rust_witness::transpile::transpile_wasm("../test-vectors/circom".to_string());
        }

        // witnesscalc only: copy .dat files to the target folder
        let out_dir = std::env::var("OUT_DIR").expect("Failed to get OUT_DIR");
        let target_dir = std::path::PathBuf::from(out_dir).join("witnesscalc/src");
        std::fs::create_dir_all(&target_dir).unwrap();

        for entry in std::fs::read_dir("../test-vectors/circom/witnesscalc")
            .expect("Failed to read source directory")
        {
            let entry = entry.expect("Failed to get directory entry");
            let src_path = entry.path();
            let dest_path = target_dir.join(entry.file_name());

            if let Err(e) = std::fs::copy(&src_path, &dest_path) {
                eprintln!(
                    "Failed to copy file {:?} to {:?}: {}",
                    src_path, dest_path, e
                );
            }
        }
    }
}
