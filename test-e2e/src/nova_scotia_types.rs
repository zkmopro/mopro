pub use std::{collections::HashMap, path::PathBuf};
pub use serde_json::Value;
pub use nova_snark::{
    traits::{circuit::TrivialTestCircuit, Group},
    provider, PublicParams, RecursiveSNARK,
};
pub use nova_scotia::circom::circuit::{Constraint, R1CS, CircomCircuit};

pub type F<G> = <G as Group>::Scalar;
pub type P1 = provider::bn256_grumpkin::bn256::Point;
pub type P2 = provider::bn256_grumpkin::grumpkin::Point;
pub type C1<G> = CircomCircuit<<G as Group>::Scalar>;
pub type C2<G> = TrivialTestCircuit<<G as Group>::Scalar>;