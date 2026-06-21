#[derive(Debug, thiserror::Error)]
pub enum MoproError {
    #[error("prover error: {0}")]
    ProverError(String),

    #[error("verifier error: {0}")]
    VerifierError(String),

    #[error("serialization error: {0}")]
    SerializationError(String),

    #[error("circuit not found: {0}")]
    CircuitNotFound(String),

    #[error("invalid input: {0}")]
    InvalidInput(String),
}
