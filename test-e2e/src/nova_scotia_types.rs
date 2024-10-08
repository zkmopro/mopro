pub use nova_scotia::circom::circuit::{CircomCircuit, Constraint, R1CS};
pub use nova_snark::{
    provider,
    traits::{circuit::TrivialTestCircuit, Group},
    PublicParams, RecursiveSNARK,
};
pub use serde_json::Value;
pub use std::{collections::HashMap, path::PathBuf};

pub type F<G> = <G as Group>::Scalar;
pub type P1 = provider::bn256_grumpkin::bn256::Point;
pub type P2 = provider::bn256_grumpkin::grumpkin::Point;
pub type C1<G> = CircomCircuit<<G as Group>::Scalar>;
pub type C2<G> = TrivialTestCircuit<<G as Group>::Scalar>;
