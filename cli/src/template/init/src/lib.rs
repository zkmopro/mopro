// Here we're calling a macro exported with Uniffi. This macro will
// write some functions and bind them to FFI type.
// These functions include:
// - `generate_circom_proof`
// - `verify_circom_proof`
// - `generate_halo2_proof`
// - `verify_halo2_proof`
// - `generate_noir_proof`
// - `verify_noir_proof`
mopro_ffi::app!();

/// You can also customize the bindings by #[uniffi::export]
/// Reference: https://mozilla.github.io/uniffi-rs/latest/proc_macro/index.html
#[uniffi::export]
fn mopro_uniffi_hello_world() -> String {
    "Hello, World!".to_string()
}

// CIRCOM_TEMPLATE

// HALO2_TEMPLATE

#[cfg(test)]
mod noir_tests {
    // Import the generated functions from the uniffi bindings

    use super::*;

    #[test]
    fn test_noir_multiplier2() {
        let srs_path = "./test-vectors/noir/noir_multiplier2.srs".to_string();
        let circuit_path = "./test-vectors/noir/noir_multiplier2.json".to_string();
        let circuit_inputs = vec!["3".to_string(), "5".to_string()];
        let vk = get_noir_verification_key(
            circuit_path.clone(),
            Some(srs_path.clone()),
            true,   // on_chain (uses Keccak for Solidity compatibility)
            false,  // low_memory_mode
        ).unwrap();

        let proof = generate_noir_proof(
            circuit_path.clone(),
            Some(srs_path.clone()),
            circuit_inputs.clone(),
            true,   // on_chain (uses Keccak for Solidity compatibility)
            vk.clone(),
            false,  // low_memory_mode
        ).unwrap();

        let valid = verify_noir_proof(
            circuit_path,
            proof,
            true,   // on_chain (uses Keccak for Solidity compatibility)
            vk,
            false,  // low_memory_mode
        ).unwrap();
        assert!(valid);
    }

    #[test]
    fn test_noir_multiplier2_with_existing_vk() {
        let srs_path = "./test-vectors/noir/noir_multiplier2.srs".to_string();
        let circuit_path = "./test-vectors/noir/noir_multiplier2.json".to_string();
        let vk_path = "./test-vectors/noir/noir_multiplier2.vk".to_string();

        // read vk from file as Vec<u8>
        let vk = std::fs::read(vk_path).unwrap();

        let circuit_inputs = vec!["3".to_string(), "5".to_string()];

        let proof = generate_noir_proof(
            circuit_path.clone(),
            Some(srs_path),
            circuit_inputs,
            true,   // on_chain (uses Keccak for Solidity compatibility)
            vk.clone(),
            false,  // low_memory_mode
        ).unwrap();

        let valid = verify_noir_proof(
            circuit_path,
            proof,
            true,   // on_chain (uses Keccak for Solidity compatibility)
            vk,
            false,  // low_memory_mode
        ).unwrap();
        assert!(valid);
    }
}

#[cfg(test)]
mod uniffi_tests {
    use super::*;

    #[test]
    fn test_mopro_uniffi_hello_world() {
        assert_eq!(mopro_uniffi_hello_world(), "Hello, World!");
    }
}
