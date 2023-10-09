use crate::MoproError;

use std::path::Path;

pub fn assert_paths_exists(wasm_path: &str, r1cs_path: &str) -> Result<(), MoproError> {
    // Check that the files exist - ark-circom should probably do this instead and not panic
    if !Path::new(wasm_path).exists() {
        return Err(MoproError::CircomError(format!(
            "Path does not exist: {}",
            wasm_path
        )));
    }

    if !Path::new(r1cs_path).exists() {
        return Err(MoproError::CircomError(format!(
            "Path does not exist: {}",
            r1cs_path
        )));
    };

    Ok(())
}
