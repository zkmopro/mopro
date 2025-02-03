mod zkey;
use ark_ec::pairing::Pairing;
pub use zkey::{read_proving_key, read_zkey, FieldSerialization, ZkeyHeaderReader};

mod qap;
pub use qap::CircomReduction;

mod circuit;
pub use circuit::CircomCircuit;

mod r1cs_reader;
pub use r1cs_reader::R1CSFile;

pub type Constraints<E> = (ConstraintVec<E>, ConstraintVec<E>, ConstraintVec<E>);
pub type ConstraintVec<E> = Vec<(usize, <E as Pairing>::ScalarField)>;
