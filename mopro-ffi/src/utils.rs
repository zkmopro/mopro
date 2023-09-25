use crate::FFIError;

pub fn serialize_to_vec<T: serde::Serialize>(data: &T) -> Result<Vec<u8>, FFIError> {
    bincode::serialize(data).map_err(|e| FFIError::SerializationError(e.to_string()))
}

pub fn deserialize_from_vec<T: for<'a> serde::Deserialize<'a>>(data: &[u8]) -> Result<T, FFIError> {
    bincode::deserialize(data).map_err(|e| FFIError::SerializationError(e.to_string()))
}

// Test the above functions
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_to_vec_works() {
        let data = "Hello World".to_string();
        let result = serialize_to_vec(&data);
        assert!(result.is_ok());
    }

    #[test]
    fn deserialize_from_vec_works() {
        let data = "Hello World".to_string();
        let serialized = serialize_to_vec(&data).unwrap();
        let result = deserialize_from_vec::<String>(&serialized);
        assert!(result.is_ok());
    }
}
