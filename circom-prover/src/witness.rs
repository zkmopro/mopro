use num::{BigInt, BigUint};
use std::{collections::HashMap, str::FromStr, thread::JoinHandle};

/// Witness function signature for rust_witness (inputs) -> witness
#[cfg(feature = "rustwitness")]
type RustWitnessWtnsFn = fn(HashMap<String, Vec<BigInt>>) -> Vec<BigInt>;
/// Witness function signature for witnesscalc_adapter (inputs) -> witness
#[cfg(feature = "witnesscalc")]
type WitnesscalcWtnsFn = fn(HashMap<String, Vec<BigInt>>) -> Vec<BigInt>;

pub enum WitnessFn {
    #[cfg(feature = "witnesscalc")]
    WitnessCalc(WitnesscalcWtnsFn),
    #[cfg(feature = "rustwitness")]
    RustWitness(RustWitnessWtnsFn),
}

pub fn generate_witness(
    witness_fn: WitnessFn,
    inputs: HashMap<String, Vec<String>>,
) -> JoinHandle<Vec<BigUint>> {
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
            #[cfg(feature = "witnesscalc")]
            WitnessFn::WitnessCalc(wit_fn) => wit_fn(bigint_inputs),
            #[cfg(feature = "rustwitness")]
            WitnessFn::RustWitness(wit_fn) => wit_fn(bigint_inputs),
        };
        witness
            .into_iter()
            .map(|w| w.to_biguint().unwrap())
            .collect::<Vec<_>>()
    })
}
