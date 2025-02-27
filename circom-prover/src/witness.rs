use num::{BigInt, BigUint};
use std::{collections::HashMap, io, str::FromStr, thread::JoinHandle};
use witnesscalc_adapter::parse_witness_to_bigints;

/// Witness function signature for rust_witness (inputs) -> witness
#[cfg(feature = "rustwitness")]
type RustWitnessWtnsFn = fn(HashMap<String, Vec<BigInt>>) -> Vec<BigInt>;
/// Witness function signature for witnesscalc_adapter (inputs) -> witness
#[cfg(feature = "witnesscalc")]
type WitnesscalcWtnsFn = fn(&str) -> io::Result<Vec<u8>>;

#[derive(Debug, Clone, Copy)]
pub enum WitnessFn {
    #[cfg(feature = "witnesscalc")]
    WitnessCalc(WitnesscalcWtnsFn),
    #[cfg(feature = "rustwitness")]
    RustWitness(RustWitnessWtnsFn),
}

pub fn generate_witness(
    witness_fn: WitnessFn,
    // inputs: HashMap<String, Vec<String>>,
    input_str: String,
) -> JoinHandle<Vec<BigUint>> {
    #[cfg(feature = "rustwitness")]
    let witness_map = json_to_hashmap(input_str.as_str()).unwrap();

    std::thread::spawn(move || {
        let witness = match witness_fn {
            #[cfg(feature = "witnesscalc")]
            WitnessFn::WitnessCalc(wit_fn) => {
                let witness = wit_fn(input_str.as_str()).unwrap();
                parse_witness_to_bigints(&witness).unwrap()
            }
            #[cfg(feature = "rustwitness")]
            WitnessFn::RustWitness(wit_fn) => {
                let bigint_inputs = witness_map
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
                wit_fn(bigint_inputs)
            }
        };
        witness
            .into_iter()
            .map(|w| w.to_biguint().unwrap())
            .collect::<Vec<_>>()
    })
}

#[cfg(feature = "rustwitness")]
pub fn json_to_hashmap(json_str: &str) -> Result<HashMap<String, Vec<String>>, serde_json::Error> {
    use serde_json::Value;

    let value: Value = serde_json::from_str(json_str)?;

    let mut hashmap = HashMap::new();

    if let Value::Object(map) = value {
        for (key, val) in map {
            if let Value::Array(arr) = val {
                let vec: Vec<String> = arr
                    .into_iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect();
                hashmap.insert(key, vec);
            }
        }
    }

    Ok(hashmap)
}
