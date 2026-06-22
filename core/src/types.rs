/// A serialized proof bundle suitable for backends that represent proofs as
/// raw bytes (e.g. Halo2, Noir).
///
/// Backends with structured proof types (e.g. Circom's G1/G2 representation,
/// Gnark's hex-encoded serialization) define their own output types in their
/// respective crates and do not need to use this type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProofBytes {
    pub proof: Vec<u8>,
    pub public_inputs: Vec<u8>,
}
