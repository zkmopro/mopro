use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

use halo2_proofs::halo2curves::ff::PrimeField;
use serde::de::{SeqAccess, Visitor};
use serde::ser::SerializeSeq;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::halo2::{Fp, SerializablePublicInputs};

pub fn deserialize_circuit_inputs(
    ser_inputs: HashMap<String, Vec<String>>,
) -> HashMap<String, Vec<Fp>> {
    ser_inputs
        .iter()
        .map(|(k, v)| {
            let fp_vec: Vec<Fp> = v
                .iter()
                .map(|s| {
                    // TODO - support big integers full range, not just u128
                    let int = u128::from_str(s).unwrap();
                    Fp::from_u128(int)
                })
                .collect();
            (k.clone(), fp_vec)
        })
        .collect()
}

impl Serialize for SerializablePublicInputs {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.0.len()))?;
        for fp in &self.0 {
            seq.serialize_element(&fp.to_bytes())?;
        }
        seq.end()
    }
}

impl<'de> Deserialize<'de> for SerializablePublicInputs {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SerializableInputsVisitor;

        impl<'de> Visitor<'de> for SerializableInputsVisitor {
            type Value = SerializablePublicInputs;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a sequence of byte arrays of length 32")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<SerializablePublicInputs, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut vec = Vec::new();
                while let Some(bytes) = seq.next_element::<[u8; 32]>()? {
                    vec.push(Fp::from_bytes(&bytes).unwrap());
                }
                Ok(SerializablePublicInputs(vec))
            }
        }

        deserializer.deserialize_seq(SerializableInputsVisitor)
    }
}

// Tests for serialization and deserialization
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_serialization() {
        let fp1 = Fp::from(1);
        let fp2 = Fp::from(2);
        let inputs = SerializablePublicInputs(vec![fp1, fp2]);

        let serialized = serde_json::to_string(&inputs).unwrap();
        println!("Serialized: {}", serialized);

        let deserialized: SerializablePublicInputs = serde_json::from_str(&serialized).unwrap();
        assert_eq!(inputs.0.len(), deserialized.0.len());
        for (original, deserialized_fp) in inputs.0.iter().zip(deserialized.0.iter()) {
            assert_eq!(original.to_bytes(), deserialized_fp.to_bytes());
        }
    }

    #[test]
    fn test_circuit_inputs_deserialization() {
        let mut serialized = HashMap::new();
        serialized.insert("out".to_string(), vec!["1".to_string(), "2".to_string()]);
        let deserialized = deserialize_circuit_inputs(serialized);
        assert_eq!(deserialized.len(), 1);
        assert_eq!(deserialized.get("out").unwrap().len(), 2);
        assert_eq!(deserialized.get("out").unwrap()[0], Fp::from(1));
        assert_eq!(deserialized.get("out").unwrap()[1], Fp::from(2));
    }
}
