#[derive(Debug, Clone)]
pub struct G1 {
    pub x: String,
    pub y: String,
    pub z: String,
}

#[derive(Debug, Clone)]
pub struct G2 {
    pub x: Vec<String>,
    pub y: Vec<String>,
    pub z: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct CircomProof {
    pub a: G1,
    pub b: G2,
    pub c: G1,
    pub protocol: String,
    pub curve: String,
}

#[derive(Debug, Clone)]
pub struct CircomProofResult {
    pub proof: CircomProof,
    pub inputs: Vec<String>,
}

#[path = "../../../cli/src/template/init/src/circom/circom_garaga.rs"]
pub mod circom_garaga;
#[path = "../../../cli/src/template/init/src/circom/garaga_convert.rs"]
pub mod garaga_convert;
#[path = "../../../cli/src/template/init/src/circom/snarkjs_types.rs"]
pub mod snarkjs_types;

pub use circom_garaga::generate_circom_groth16_garaga_calldata;
