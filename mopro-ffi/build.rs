use std::path::PathBuf;

fn main() {
    #[cfg(feature = "circom")]
    if std::env::var("MOPRO_FFI_LINK_TEST_WITNESS").unwrap_or_default() != "" {
        rust_witness::transpile::transpile_wasm("../test-vectors/circom".to_string());
    }
    #[cfg(feature = "rapidsnark")]
    link_rapidsnark();
}

#[allow(dead_code)]
fn link_rapidsnark() {
    let target = std::env::var("TARGET").unwrap();
    let arch = target.split('-').next().unwrap();
    println!("cargo:warning={target}");

    // Try to list contents of the target directory
    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let rapidsnark_dir = manifest_dir.join("rapidsnark");
    let absolute_lib_path = if rapidsnark_dir.join(&target).exists() {
        rapidsnark_dir.join(target)
    } else {
        rapidsnark_dir.join(arch)
    };

    let compiler = cc::Build::new().get_compiler();
    let cpp_stdlib = if compiler.is_like_clang() {
        "c++"
    } else {
        "stdc++"
    };

    // println!("cargo:rustc-link-arg=-Wl,-unexported_symbols_list,/dev/null");
    // println!("cargo:rustc-link-arg=-Wl,-exported_symbols_list,/dev/null");
    // println!("cargo:rustc-link-arg=-Wl,-keep_private_externs");
    // println!("cargo:rustc-link-arg=-Wl,-no_compact_unwind");

    // println!("cargo:rustc-link-arg=-Wl,-keep_private_externs");
    // println!("cargo:rustc-link-arg=-Wl,-no_dead_strip");
    // println!("cargo:rustc-link-arg=-Wl,-demangle");
    println!(
        "cargo:rustc-link-search=native={}",
        absolute_lib_path.clone().display()
    );

    // println!("cargo:rustc-link-arg=-Wl,-stack_size,0x1000000");
    // println!("cargo:rustc-link-arg=-Wl,--whole-archive");

    println!("cargo:rustc-link-lib=static=rapidsnark");
    println!("cargo:rustc-link-lib={}", cpp_stdlib);
    println!("cargo:rustc-link-lib=pthread");
    println!("cargo:rustc-link-lib=static=fr");
    println!("cargo:rustc-link-lib=static=fq");
    println!("cargo:rustc-link-lib=static=gmp");
}
