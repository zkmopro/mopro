use crate::MoproError;

/// The core contract that every ZK proof backend must implement.
///
/// The struct implementing this trait acts as the "configured prover": it holds
/// all circuit-specific state (key paths, compile options, runtime handles,
/// etc.) established at construction time. `prove` and `verify` are then
/// called without re-passing that configuration.
///
/// Associated types let each backend own its wire format:
/// - `Input`  — the witness / circuit inputs (JSON string, `HashMap`, `Vec<String>`, …)
/// - `Output` — the proof artifact plus public outputs bundled together
///
/// Both bounds are intentionally minimal (`Send` only). Backends may add
/// `Clone`, `Serialize`, or other traits on their concrete types as needed.
pub trait Prover: Send + Sync {
    /// Witness / circuit inputs consumed during proof generation.
    type Input: Send;

    /// Opaque bundle of proof bytes and public outputs.
    /// The same value produced by `prove` is passed back to `verify`.
    type Output: Send;

    /// Generate a ZK proof from the given inputs.
    ///
    /// The prover's configuration (key paths, options) was fixed at
    /// construction and is accessed via `&self`.
    fn prove(&self, input: Self::Input) -> Result<Self::Output, MoproError>;

    /// Verify a proof previously produced by `prove`.
    ///
    /// Returns `Ok(true)` if the proof is valid, `Ok(false)` if it is not,
    /// and `Err(…)` only when the verification process itself fails.
    fn verify(&self, output: &Self::Output) -> Result<bool, MoproError>;
}
