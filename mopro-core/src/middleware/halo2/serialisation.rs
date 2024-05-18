use std::fmt;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::{SeqAccess, Visitor};
use serde::ser::SerializeSeq;
use crate::middleware::halo2::{Fp, SerializableInputs};

impl Serialize for SerializableInputs {
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

impl<'de> Deserialize<'de> for SerializableInputs {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SerializableInputsVisitor;

        impl<'de> Visitor<'de> for SerializableInputsVisitor {
            type Value = SerializableInputs;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a sequence of byte arrays of length 32")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<SerializableInputs, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut vec = Vec::new();
                while let Some(bytes) = seq.next_element::<[u8; 32]>()? {
                    vec.push(Fp::from_bytes(&bytes).expect("Invalid bytes"));
                }
                Ok(SerializableInputs(vec))
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
        let inputs = SerializableInputs(vec![fp1, fp2]);

        let serialized = serde_json::to_string(&inputs).unwrap();
        println!("Serialized: {}", serialized);

        let deserialized: SerializableInputs = serde_json::from_str(&serialized).unwrap();
        assert_eq!(inputs.0.len(), deserialized.0.len());
        for (original, deserialized_fp) in inputs.0.iter().zip(deserialized.0.iter()) {
            assert_eq!(original.to_bytes(), deserialized_fp.to_bytes());
        }
    }
}