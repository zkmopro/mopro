use mopro_ffi::{app, WtnsFn};
use std::path::Path;

rust_witness::witness!(multiplier2);
rust_witness::witness!(keccak256256test);

app!();

// This should be defined by a file that the mopro package consumer authors
// then we reference it in our build somehow
fn circuit_data(zkey_path: &str) -> Result<WtnsFn, MoproError> {
    let name = Path::new(zkey_path).file_stem().unwrap();
    match name.to_str().unwrap() {
        "multiplier2_final" => Ok(multiplier2_witness),
        "keccak256_256_test_final" => Ok(keccak256256test_witness),
        _ => Err(MoproError::CircomError("Unknown circuit name".to_string())),
    }
}
