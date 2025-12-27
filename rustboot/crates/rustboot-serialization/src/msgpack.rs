//! MessagePack serialization helpers

use crate::error::SerializationResult;
use serde::{de::DeserializeOwned, Serialize};

/// Serialize to MessagePack bytes
pub fn to_msgpack<T: Serialize>(value: &T) -> SerializationResult<Vec<u8>> {
    rmp_serde::to_vec(value).map_err(Into::into)
}

/// Deserialize from MessagePack bytes
pub fn from_msgpack<T: DeserializeOwned>(bytes: &[u8]) -> SerializationResult<T> {
    rmp_serde::from_slice(bytes).map_err(Into::into)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct TestData {
        name: String,
        age: u32,
        active: bool,
        tags: Vec<String>,
    }

    #[test]
    fn test_msgpack_roundtrip() {
        let data = TestData {
            name: "Alice".to_string(),
            age: 30,
            active: true,
            tags: vec!["rust".to_string(), "developer".to_string()],
        };

        let bytes = to_msgpack(&data).unwrap();
        let decoded: TestData = from_msgpack(&bytes).unwrap();

        assert_eq!(data, decoded);
    }

    #[test]
    fn test_msgpack_smaller_than_json() {
        use crate::json::to_json_bytes;

        let data = TestData {
            name: "Bob".to_string(),
            age: 25,
            active: false,
            tags: vec!["backend".to_string(), "api".to_string()],
        };

        let json_bytes = to_json_bytes(&data).unwrap();
        let msgpack_bytes = to_msgpack(&data).unwrap();

        // MessagePack should be smaller or equal
        assert!(msgpack_bytes.len() <= json_bytes.len());
    }

    #[test]
    fn test_msgpack_complex_types() {
        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        struct Complex {
            numbers: Vec<i32>,
            nested: Vec<Vec<String>>,
        }

        let data = Complex {
            numbers: vec![1, 2, 3, 4, 5],
            nested: vec![
                vec!["a".to_string(), "b".to_string()],
                vec!["c".to_string(), "d".to_string()],
            ],
        };

        let bytes = to_msgpack(&data).unwrap();
        let decoded: Complex = from_msgpack(&bytes).unwrap();

        assert_eq!(data, decoded);
    }
}
