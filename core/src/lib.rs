mod error;
mod prover;
mod types;

pub use error::MoproError;
pub use prover::Prover;
pub use types::ProofBytes;

#[cfg(test)]
mod tests {
    use super::*;

    struct FakeProver {
        secret: u8,
    }

    impl Prover for FakeProver {
        type Input = Vec<u8>;
        type Output = ProofBytes;

        fn prove(&self, input: Self::Input) -> Result<ProofBytes, MoproError> {
            let proof = input.iter().map(|b| b ^ self.secret).collect();
            Ok(ProofBytes { proof, public_inputs: vec![self.secret] })
        }

        fn verify(&self, output: &ProofBytes) -> Result<bool, MoproError> {
            Ok(output.public_inputs.first().copied() == Some(self.secret))
        }
    }

    #[test]
    fn prover_trait_is_implementable() {
        let p = FakeProver { secret: 42 };
        let out = p.prove(vec![1, 2, 3]).unwrap();
        assert!(p.verify(&out).unwrap());
    }

    #[test]
    fn mopro_error_display() {
        let e = MoproError::ProverError("bad zkey".to_string());
        assert_eq!(e.to_string(), "prover error: bad zkey");

        let e = MoproError::CircuitNotFound("multiplier2".to_string());
        assert_eq!(e.to_string(), "circuit not found: multiplier2");

        let e = MoproError::InvalidInput("expected JSON".to_string());
        assert_eq!(e.to_string(), "invalid input: expected JSON");
    }

    #[test]
    fn proof_bytes_equality() {
        let a = ProofBytes { proof: vec![1, 2], public_inputs: vec![3] };
        let b = a.clone();
        assert_eq!(a, b);
    }
}
