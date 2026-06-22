use std::path::PathBuf;

use mopro_core::{MoproError, Prover};

/// Thin adapter that wires `noir-rs` (Barretenberg backend) into
/// [`mopro_core::Prover`].
///
/// All circuit-level config is fixed at construction; only the ordered
/// witness inputs vary per call.
///
/// Enable the `barretenberg` Cargo feature to compile this with the
/// actual `noir-rs` engine.  Without that feature the struct exists but
/// both `prove` and `verify` return `MoproError::ProverError`.
///
/// # Hash function selection
/// - `on_chain = false` → Poseidon (better performance, off-chain verification)
/// - `on_chain = true`  → Keccak256 (Solidity verifier compatible, on-chain)
#[allow(dead_code)]
pub struct NoirProverAdapter {
    circuit_path: PathBuf,
    srs_path: Option<PathBuf>,
    vk: Vec<u8>,
    on_chain: bool,
    low_memory_mode: bool,
}

impl NoirProverAdapter {
    pub fn new(
        circuit_path: impl Into<PathBuf>,
        srs_path: Option<impl Into<PathBuf>>,
        vk: Vec<u8>,
        on_chain: bool,
        low_memory_mode: bool,
    ) -> Self {
        Self {
            circuit_path: circuit_path.into(),
            srs_path: srs_path.map(Into::into),
            vk,
            on_chain,
            low_memory_mode,
        }
    }

    /// Derive the verification key from the circuit.
    /// Call once before constructing the adapter.
    ///
    /// Requires the `barretenberg` feature.
    pub fn compute_vk(
        circuit_path: impl Into<PathBuf>,
        srs_path: Option<impl Into<PathBuf>>,
        on_chain: bool,
        low_memory_mode: bool,
    ) -> Result<Vec<u8>, MoproError> {
        #[cfg(feature = "barretenberg")]
        {
            use noir_rs::barretenberg::{
                srs::setup_srs_from_bytecode,
                verify::{
                    get_ultra_honk_keccak_verification_key, get_ultra_honk_verification_key,
                },
            };

            let path: PathBuf = circuit_path.into();
            let srs_str = srs_path.map(|p| p.into().to_string_lossy().into_owned());
            let bytecode = read_bytecode(&path)?;

            setup_srs_from_bytecode(&bytecode, srs_str.as_deref(), false)
                .map_err(|e| MoproError::ProverError(format!("SRS setup failed: {e}")))?;

            if on_chain {
                get_ultra_honk_keccak_verification_key(&bytecode, false, low_memory_mode)
            } else {
                get_ultra_honk_verification_key(&bytecode, low_memory_mode)
            }
            .map_err(|e| MoproError::ProverError(format!("VK derivation failed: {e}")))
        }

        #[cfg(not(feature = "barretenberg"))]
        {
            let _ = (circuit_path, srs_path, on_chain, low_memory_mode);
            Err(MoproError::ProverError(
                "noir-adapter: compile with --features barretenberg to enable".into(),
            ))
        }
    }
}

impl Prover for NoirProverAdapter {
    /// Ordered public witness inputs as decimal strings, e.g. `["3", "5"]`.
    type Input = Vec<String>;

    /// Raw proof bytes produced by Barretenberg.
    type Output = Vec<u8>;

    fn prove(&self, input: Self::Input) -> Result<Self::Output, MoproError> {
        #[cfg(feature = "barretenberg")]
        {
            use noir_rs::barretenberg::{
                prove::{prove_ultra_honk, prove_ultra_honk_keccak},
                srs::setup_srs_from_bytecode,
            };
            use noir_rs::witness::from_vec_str_to_witness_map;

            let bytecode = read_bytecode(&self.circuit_path)?;
            let srs_str = self
                .srs_path
                .as_deref()
                .map(|p| p.to_string_lossy().into_owned());

            setup_srs_from_bytecode(&bytecode, srs_str.as_deref(), false)
                .map_err(|e| MoproError::ProverError(format!("SRS setup failed: {e}")))?;

            let witness =
                from_vec_str_to_witness_map(input.iter().map(|s| s.as_str()).collect())
                    .map_err(|e| MoproError::InvalidInput(e.to_string()))?;

            if self.on_chain {
                prove_ultra_honk_keccak(
                    &bytecode,
                    witness,
                    self.vk.clone(),
                    false,
                    self.low_memory_mode,
                )
            } else {
                prove_ultra_honk(&bytecode, witness, self.vk.clone(), self.low_memory_mode)
            }
            .map_err(|e| MoproError::ProverError(e.to_string()))
        }

        #[cfg(not(feature = "barretenberg"))]
        {
            let _ = input;
            Err(MoproError::ProverError(
                "noir-adapter: compile with --features barretenberg to enable".into(),
            ))
        }
    }

    fn verify(&self, output: &Self::Output) -> Result<bool, MoproError> {
        #[cfg(feature = "barretenberg")]
        {
            use noir_rs::barretenberg::verify::{verify_ultra_honk, verify_ultra_honk_keccak};

            if self.on_chain {
                verify_ultra_honk_keccak(output.clone(), self.vk.clone(), false)
            } else {
                verify_ultra_honk(output.clone(), self.vk.clone())
            }
            .map_err(|e| MoproError::VerifierError(e.to_string()))
        }

        #[cfg(not(feature = "barretenberg"))]
        {
            let _ = output;
            Err(MoproError::VerifierError(
                "noir-adapter: compile with --features barretenberg to enable".into(),
            ))
        }
    }
}

#[cfg(feature = "barretenberg")]
fn read_bytecode(circuit_path: &PathBuf) -> Result<String, MoproError> {
    let text = std::fs::read_to_string(circuit_path)
        .map_err(|e| MoproError::CircuitNotFound(e.to_string()))?;
    let json: serde_json::Value = serde_json::from_str(&text)
        .map_err(|e| MoproError::SerializationError(e.to_string()))?;
    json["bytecode"]
        .as_str()
        .map(str::to_owned)
        .ok_or_else(|| MoproError::SerializationError("missing `bytecode` field".into()))
}
