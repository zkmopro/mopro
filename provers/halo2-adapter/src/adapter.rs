use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;

use mopro_core::{MoproError, Prover};

// ---------------------------------------------------------------------------
// FFI-ready output type
//
// Fields are Vec<u8> — natively handled by every FFI layer:
//   • UniFFI  — Data (Swift) / ByteArray (Kotlin) via uniffi::Record
//   • wasm-bindgen — Uint8Array via serde_wasm_bindgen
//   • flutter_rust_bridge — Uint8List natively
//
// Input type HashMap<String, Vec<String>> is also FFI-compatible:
//   • UniFFI  — [String: [String]] (Swift) / Map<String, List<String>> (Kotlin)
//   • wasm-bindgen — serialised via serde_wasm_bindgen
//   • flutter_rust_bridge — Map<String, List<String>> via FRB codegen
// ---------------------------------------------------------------------------

/// Proof bundle returned by [`Halo2ProverAdapter::prove`].
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct Halo2Output {
    pub proof: Vec<u8>,
    pub public_inputs: Vec<u8>,
}

// ---------------------------------------------------------------------------
// Function pointer types
//
// These are Rust-internal — function pointers cannot cross FFI boundaries,
// so Halo2ProverAdapter is always constructed on the Rust side (in cli/ or
// an ffi/backends/* glue module). Only the *output* crosses the FFI boundary.
// ---------------------------------------------------------------------------

/// Function pointer type for Halo2 proof generation.
///
/// Arguments: `(srs_path, pk_path, circuit_inputs) → (proof_bytes, public_input_bytes)`
pub type Halo2ProveFn = fn(
    &str,
    &str,
    HashMap<String, Vec<String>>,
) -> Result<(Vec<u8>, Vec<u8>), Box<dyn Error>>;

/// Function pointer type for Halo2 proof verification.
///
/// Arguments: `(srs_path, vk_path, proof_bytes, public_input_bytes) → bool`
pub type Halo2VerifyFn =
    fn(&str, &str, Vec<u8>, Vec<u8>) -> Result<bool, Box<dyn Error>>;

// ---------------------------------------------------------------------------
// Adapter
// ---------------------------------------------------------------------------

/// Thin adapter that wires a user-supplied Halo2 circuit into [`mopro_core::Prover`].
///
/// Halo2 has no single universal prover crate — each circuit ships its own
/// `prove` / `verify` functions. This adapter holds those function pointers
/// alongside the key-file paths, keeping the per-call API to inputs-only.
///
/// # Example
///
/// ```ignore
/// use halo2_prover_adapter::{Halo2ProverAdapter, Halo2Output};
/// use std::collections::HashMap;
///
/// let prover = Halo2ProverAdapter::new(
///     "./test-vectors/halo2/plonk_fibonacci_srs.bin",
///     "./test-vectors/halo2/plonk_fibonacci_pk.bin",
///     "./test-vectors/halo2/plonk_fibonacci_vk.bin",
///     plonk_fibonacci::prove,
///     plonk_fibonacci::verify,
/// );
///
/// let mut inputs = HashMap::new();
/// inputs.insert("out".to_string(), vec!["55".to_string()]);
///
/// let output: Halo2Output = prover.prove(inputs).unwrap();
/// assert!(prover.verify(&output).unwrap());
/// ```
pub struct Halo2ProverAdapter {
    srs_path: PathBuf,
    pk_path: PathBuf,
    vk_path: PathBuf,
    prove_fn: Halo2ProveFn,
    verify_fn: Halo2VerifyFn,
}

impl Halo2ProverAdapter {
    pub fn new(
        srs_path: impl Into<PathBuf>,
        pk_path: impl Into<PathBuf>,
        vk_path: impl Into<PathBuf>,
        prove_fn: Halo2ProveFn,
        verify_fn: Halo2VerifyFn,
    ) -> Self {
        Self {
            srs_path: srs_path.into(),
            pk_path: pk_path.into(),
            vk_path: vk_path.into(),
            prove_fn,
            verify_fn,
        }
    }
}

impl Prover for Halo2ProverAdapter {
    /// Named circuit inputs, e.g. `{"out": ["55"]}`.
    type Input = HashMap<String, Vec<String>>;

    /// Raw proof bytes bundled with public inputs — FFI-safe.
    type Output = Halo2Output;

    fn prove(&self, input: Self::Input) -> Result<Self::Output, MoproError> {
        let srs = self.srs_path.to_string_lossy();
        let pk = self.pk_path.to_string_lossy();

        let (proof, public_inputs) = (self.prove_fn)(&srs, &pk, input)
            .map_err(|e| MoproError::ProverError(e.to_string()))?;

        Ok(Halo2Output { proof, public_inputs })
    }

    fn verify(&self, output: &Self::Output) -> Result<bool, MoproError> {
        let srs = self.srs_path.to_string_lossy();
        let vk = self.vk_path.to_string_lossy();

        (self.verify_fn)(&srs, &vk, output.proof.clone(), output.public_inputs.clone())
            .map_err(|e| MoproError::VerifierError(e.to_string()))
    }
}
