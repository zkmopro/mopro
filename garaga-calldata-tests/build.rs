fn main() {
    // Template sources use `cfg(feature = "uniffi")` but this crate has no such
    // Cargo feature (a real `uniffi = []` would break `--all-features`). Tell
    // rustc the value is expected so check-cfg stays quiet.
    println!("cargo::rustc-check-cfg=cfg(feature, values(\"uniffi\"))");
}
