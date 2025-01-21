use num::{BigInt, BigUint};
use std::{collections::HashMap, str::FromStr};

/// Witness function signature for rust_witness (inputs) -> witness
pub type RustWitnessWtnsFn = fn(HashMap<String, Vec<BigInt>>) -> Vec<BigInt>;
/// Witness function signature for witnesscalc_adapter (inputs, .dat file path) -> witness
pub type WitnesscalcWtnsFn = fn(HashMap<String, Vec<BigInt>>, &str) -> Vec<BigInt>;

pub enum WitnessFn {
    WitnessCalc(WitnesscalcWtnsFn),
    RustWitness(RustWitnessWtnsFn),
}

pub enum WitnessLib {
    WitnessCalc,
    RustWitness,
}

/// To create witness functions corresponding to different witness generation libs.
#[macro_export]
macro_rules! create_witness_fn {
    ($witness:expr, $fn:ident) => {
        match $witness {
            WitnessLib::RustWitness => rust_witness::witness!($fn),
            WitnessLib::WitnessCalc => witnesscalc_adapter::witness!($fn),
        }
    };
}

pub fn generate_witness(
    witness_fn: WitnessFn,
    inputs: HashMap<String, Vec<String>>,
    dat_path: String,
) -> Vec<BigUint> {
    std::thread::spawn(move || {
        let bigint_inputs = inputs
            .into_iter()
            .map(|(k, v)| {
                (
                    k,
                    v.into_iter()
                        .map(|i| BigInt::from_str(&i).unwrap())
                        .collect(),
                )
            })
            .collect();

        let witness = match witness_fn {
            WitnessFn::WitnessCalc(wit_fn) => wit_fn(bigint_inputs, dat_path.as_str()),
            WitnessFn::RustWitness(wit_fn) => wit_fn(bigint_inputs),
        };
        witness
            .into_iter()
            .map(|w| w.to_biguint().unwrap())
            .collect::<Vec<_>>()
    })
    .join()
    .map_err(|_e| anyhow::anyhow!("witness thread panicked"))
    .unwrap()
}
