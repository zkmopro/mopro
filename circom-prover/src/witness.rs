use num::{BigInt, BigUint};
use std::{collections::HashMap, thread::JoinHandle};

#[cfg(feature = "rustwitness")]
use std::str::FromStr;

#[cfg(any(feature = "witnesscalc", feature = "circom-witnesscalc"))]
use witnesscalc_adapter::parse_witness_to_bigints;

/// Witness function signature for rust_witness (inputs) -> witness
type RustWitnessWtnsFn = fn(HashMap<String, Vec<BigInt>>) -> Vec<BigInt>;
/// Witness function signature for witnesscalc_adapter (inputs) -> witness
type WitnesscalcWtnsFn = fn(&str) -> anyhow::Result<Vec<u8>>;
/// Witness function signature for circom-witnesscalc (inputs, graph_path) -> witness
type CircomWitnessCalcWtnsFn = fn(&str) -> anyhow::Result<Vec<u8>>;

#[derive(Debug, Clone, Copy)]
pub enum WitnessFn {
    WitnessCalc(WitnesscalcWtnsFn),
    RustWitness(RustWitnessWtnsFn),
    CircomWitnessCalc(CircomWitnessCalcWtnsFn),
}

#[macro_export]
#[cfg(feature = "circom-witnesscalc")]
macro_rules! graph {
    ($name:ident, $path:expr) => {
        $crate::paste::paste! {
            static [<$name:upper _GRAPH_DATA>]: &[u8] = include_bytes!($path);

            pub fn [<$name _witness>](json_input: &str) -> Result<Vec<u8>, $crate::__macro_deps::anyhow::Error> {
                $crate::__macro_deps::circom_witnesscalc::calc_witness(json_input, [<$name:upper _GRAPH_DATA>])
                    .map_err(|e| $crate::__macro_deps::anyhow::anyhow!("{}", e))
            }
        }
    };
}

#[allow(unused_variables)]
pub fn generate_witness(witness_fn: WitnessFn, json_input_str: String) -> JoinHandle<Vec<BigUint>> {
    #[cfg(feature = "rustwitness")]
    let witness_map = json_to_hashmap(json_input_str.as_str()).unwrap();

    std::thread::spawn(move || {
        let witness: Vec<BigInt> = match witness_fn {
            #[cfg(feature = "witnesscalc")]
            WitnessFn::WitnessCalc(wit_fn) => {
                let witness = wit_fn(json_input_str.as_str()).unwrap();
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
            #[cfg(feature = "circom-witnesscalc")]
            WitnessFn::CircomWitnessCalc(wit_fn) => {
                let witness = wit_fn(json_input_str.as_str()).unwrap();
                parse_witness_to_bigints(&witness).unwrap()
            }
            #[allow(unreachable_patterns)]
            _ => panic!("Unsupported witness function"),
        };
        #[allow(unreachable_code)]
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
