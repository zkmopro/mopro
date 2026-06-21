use std::path::PathBuf;

use circom_prover::{
    prover::{
        circom::{CURVE_BLS12_381, CURVE_BN254},
        CircomProof as UpstreamCircomProof,
        ProofLib as UpstreamProofLib,
    },
    witness::WitnessFn,
    CircomProver,
};
use mopro_core::{MoproError, Prover};

// ---------------------------------------------------------------------------
// FFI-ready output types
//
// All fields use String / Vec<String> so every FFI layer can consume them:
//   • UniFFI  — #[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
//   • wasm-bindgen — serialised via serde + serde_wasm_bindgen (no extra
//                    annotation needed; the ffi/backends/wasm layer handles it)
//   • flutter_rust_bridge — FRB codegen supports String/Vec natively
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct G1 {
    pub x: String,
    pub y: String,
    pub z: String,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct G2 {
    pub x: Vec<String>,
    pub y: Vec<String>,
    pub z: Vec<String>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct CircomProof {
    pub a: G1,
    pub b: G2,
    pub c: G1,
    pub protocol: String,
    pub curve: String,
}

/// Proof bundle returned by [`CircomProverAdapter::prove`].
///
/// All fields are String / Vec<String> — safe to pass across any FFI boundary.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct CircomOutput {
    pub proof: CircomProof,
    pub public_inputs: Vec<String>,
}

/// Which backend to use for proving / verifying.
///
/// Defined here (not re-exported from circom-prover) so we can attach
/// FFI derives.
#[derive(Debug, Clone, Copy, Default, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
pub enum ProofLib {
    #[default]
    Arkworks,
    Rapidsnark,
}

impl From<ProofLib> for UpstreamProofLib {
    fn from(lib: ProofLib) -> Self {
        match lib {
            ProofLib::Arkworks => UpstreamProofLib::Arkworks,
            ProofLib::Rapidsnark => UpstreamProofLib::Rapidsnark,
        }
    }
}

// ---------------------------------------------------------------------------
// Type conversions: upstream BigUint ↔ our String types
// ---------------------------------------------------------------------------

impl From<circom_prover::prover::circom::G1> for G1 {
    fn from(g: circom_prover::prover::circom::G1) -> Self {
        G1 { x: g.x.to_string(), y: g.y.to_string(), z: g.z.to_string() }
    }
}

impl From<circom_prover::prover::circom::G2> for G2 {
    fn from(g: circom_prover::prover::circom::G2) -> Self {
        G2 {
            x: vec![g.x[0].to_string(), g.x[1].to_string()],
            y: vec![g.y[0].to_string(), g.y[1].to_string()],
            z: vec![g.z[0].to_string(), g.z[1].to_string()],
        }
    }
}

impl From<circom_prover::prover::circom::Proof> for CircomProof {
    fn from(p: circom_prover::prover::circom::Proof) -> Self {
        CircomProof {
            a: p.a.into(),
            b: p.b.into(),
            c: p.c.into(),
            protocol: p.protocol,
            curve: p.curve,
        }
    }
}

impl From<G1> for circom_prover::prover::circom::G1 {
    fn from(g: G1) -> Self {
        use num_bigint::BigUint;
        use std::str::FromStr;
        circom_prover::prover::circom::G1 {
            x: BigUint::from_str(&g.x).unwrap(),
            y: BigUint::from_str(&g.y).unwrap(),
            z: BigUint::from_str(&g.z).unwrap(),
        }
    }
}

impl From<G2> for circom_prover::prover::circom::G2 {
    fn from(g: G2) -> Self {
        use num_bigint::BigUint;
        use std::str::FromStr;
        circom_prover::prover::circom::G2 {
            x: [BigUint::from_str(&g.x[0]).unwrap(), BigUint::from_str(&g.x[1]).unwrap()],
            y: [BigUint::from_str(&g.y[0]).unwrap(), BigUint::from_str(&g.y[1]).unwrap()],
            z: [BigUint::from_str(&g.z[0]).unwrap(), BigUint::from_str(&g.z[1]).unwrap()],
        }
    }
}

impl From<CircomProof> for circom_prover::prover::circom::Proof {
    fn from(p: CircomProof) -> Self {
        circom_prover::prover::circom::Proof {
            a: p.a.into(),
            b: p.b.into(),
            c: p.c.into(),
            protocol: p.protocol,
            curve: p.curve,
        }
    }
}

// ---------------------------------------------------------------------------
// Adapter
// ---------------------------------------------------------------------------

/// Thin adapter that wires the external `circom-prover` crate into
/// [`mopro_core::Prover`].
///
/// All circuit-level config (zkey path, backend, witness function) is fixed
/// at construction time; only the JSON witness input varies per call.
///
/// # FFI compatibility
/// Enable the `uniffi` feature to get `uniffi::Record` / `uniffi::Enum`
/// derives on all output types.  For wasm-bindgen, serialise `CircomOutput`
/// via `serde_wasm_bindgen` (the `serde` derives are always present).
/// flutter_rust_bridge handles `String` / `Vec<String>` fields natively.
pub struct CircomProverAdapter {
    zkey_path: PathBuf,
    proof_lib: ProofLib,
    witness_fn: WitnessFn,
}

impl CircomProverAdapter {
    pub fn new(
        zkey_path: impl Into<PathBuf>,
        proof_lib: ProofLib,
        witness_fn: WitnessFn,
    ) -> Self {
        Self { zkey_path: zkey_path.into(), proof_lib, witness_fn }
    }
}

impl Prover for CircomProverAdapter {
    /// JSON-encoded witness inputs, e.g. `{"a": "2", "b": "3"}`.
    type Input = String;

    /// String-based Groth16 proof + public inputs — FFI-safe.
    type Output = CircomOutput;

    fn prove(&self, input: Self::Input) -> Result<Self::Output, MoproError> {
        let ret = CircomProver::prove(
            self.proof_lib.into(),
            self.witness_fn,
            input,
            self.zkey_path.to_string_lossy().into_owned(),
        )
        .map_err(|e| MoproError::ProverError(e.to_string()))?;

        match ret.proof.curve.as_str() {
            CURVE_BN254 | CURVE_BLS12_381 => {}
            other => {
                return Err(MoproError::ProverError(format!("unsupported curve: {other}")))
            }
        }

        let public_inputs: Vec<String> = ret.pub_inputs.into();
        Ok(CircomOutput { proof: ret.proof.into(), public_inputs })
    }

    fn verify(&self, output: &Self::Output) -> Result<bool, MoproError> {
        let upstream = UpstreamCircomProof {
            proof: output.proof.clone().into(),
            pub_inputs: output.public_inputs.clone().into(),
        };
        CircomProver::verify(
            self.proof_lib.into(),
            upstream,
            self.zkey_path.to_string_lossy().into_owned(),
        )
        .map_err(|e| MoproError::VerifierError(e.to_string()))
    }
}
