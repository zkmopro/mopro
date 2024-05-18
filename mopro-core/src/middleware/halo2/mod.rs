use std::io::Cursor;
use std::time::Instant;

use ark_ff::BigInteger;
use ark_std::rand::thread_rng;
pub(crate) use halo2_proofs::halo2curves::bn256::{Bn256, Fr as Fp, G1Affine};
use halo2_proofs::plonk::{Circuit, create_proof, ProvingKey, verify_proof, VerifyingKey};
use halo2_proofs::poly::commitment::{Params, ParamsProver};
use halo2_proofs::poly::kzg::commitment::{KZGCommitmentScheme, ParamsKZG};
use halo2_proofs::poly::kzg::multiopen::{ProverGWC, VerifierGWC};
use halo2_proofs::poly::kzg::strategy::SingleStrategy;
use halo2_proofs::SerdeFormat::RawBytes;
use halo2_proofs::transcript::{Blake2bRead, Blake2bWrite, TranscriptReadBuffer, TranscriptWriterBuffer};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use halo2_examples::FinbonaciCircuit;

use crate::MoproError;

mod serialisation;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SerializableProof(pub Vec<u8>);

#[derive(Clone, Debug)]
pub struct SerializableInputs(pub Vec<Fp>);



/// Read SRS from file
const SRS_BYTES: &[u8] = include_bytes!(env!("BUILD_SRS_FILE"));

static SRS: Lazy<ParamsKZG<Bn256>> = Lazy::new(|| {
    let mut reader = Cursor::new(SRS_BYTES);
    ParamsKZG::read(&mut reader).expect("Unable to read SRS from file")
});


/// Read Proving Key (PK) from file

const PK_BYTES: &[u8] = include_bytes!(env!("BUILD_PK_FILE"));

static PK: Lazy<ProvingKey<G1Affine>> = Lazy::new(|| {
    let mut reader = Cursor::new(PK_BYTES);
    ProvingKey::read::<_, FinbonaciCircuit<Fp>>(&mut reader, RawBytes).expect("Unable to read PK from file")
});

/// Read Verification Key (VK) from file

const VK_BYTES: &[u8] = include_bytes!(env!("BUILD_VK_FILE"));

static VK: Lazy<VerifyingKey<G1Affine>> = Lazy::new(|| {
    let mut reader = Cursor::new(VK_BYTES);
    VerifyingKey::read::<_, FinbonaciCircuit<Fp>>(&mut reader, RawBytes).expect("Unable to read VK from file")
});


pub fn generate_halo2_proof2(
    input: u64,
) -> color_eyre::Result<(SerializableProof, SerializableInputs), MoproError> {

        let start = Instant::now();


        let k = 4;
        let circuit = FinbonaciCircuit::<Fp>::default();

        // Generate the proof - so far using dummy inputs, will be replaced with actual inputs

        let a = Fp::from(1); // F[0]
        let b = Fp::from(1); // F[1]
        let out = Fp::from(input); // F[9]

        let mut transcript = TranscriptWriterBuffer::<_, G1Affine, _>::init(Vec::new());


        let mut public_input = vec![a, b, out];

        create_proof::<
            KZGCommitmentScheme<Bn256>,
            ProverGWC<_>,
            _,
            _,
            Blake2bWrite<_, _, _>,
            _,
        >(
            &SRS,
            &PK,
            &[circuit],
            vec![vec![public_input.clone().as_slice()].as_slice()].as_slice(), // TODO - this might be wrong
            thread_rng(),
            &mut transcript,
        )
        .unwrap();

        let proof = transcript.finalize();

        let proving_duration = start.elapsed();
        println!("Proving time 2: {:?}", proving_duration);

        Ok((SerializableProof(proof), SerializableInputs(public_input)))

}

pub fn verify_halo2_proof2(
    serialized_proof: SerializableProof,
    serialized_inputs: SerializableInputs,
) -> color_eyre::Result<bool, MoproError> {
    let start = Instant::now();

    let proof = serialized_proof.0;
    let inputs = serialized_inputs.0;

    let mut transcript = TranscriptReadBuffer::<_, G1Affine, _>::init(proof.as_slice());
    let proof_verified = verify_proof::<_, VerifierGWC<_>, _, Blake2bRead<_, _, _>, _>(
        SRS.verifier_params(),
        &VK,
        SingleStrategy::new(&SRS),
        &[&[inputs.as_ref()]],
        &mut transcript,
    ).is_ok();

    let verification_duration = start.elapsed();
    println!("Verification time 2: {:?}", verification_duration);
    Ok(proof_verified)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_halo2_proof2() {
        let (proof, inputs) = generate_halo2_proof2(55).unwrap();
        assert_eq!(inputs.0[0], Fp::from(55));
    }

    #[test]
    fn test_verify_halo2_proof2() {
        let (proof, inputs) = generate_halo2_proof2(55).unwrap();
        let verified = verify_halo2_proof2(proof, inputs).unwrap();
        assert!(verified);
    }
}