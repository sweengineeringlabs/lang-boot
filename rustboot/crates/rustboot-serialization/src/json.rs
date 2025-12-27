//! JSON serialization helpers

use crate::error::SerializationResult;
use serde::{de::DeserializeOwned, Serialize};

/// Serialize to JSON string
pub fn to_json<T: Serialize>(value: &T) -> SerializationResult<String> {
    serde_json::to_string(value).map_err(Into::into)
}

/// Serialize to pretty JSON string
pub fn to_json_pretty<T: Serialize>(value: &T) -> SerializationResult<String> {
    serde_json::to_string_pretty(value).map_err(Into::into)
}

/// Serialize to JSON bytes
pub fn to_json_bytes<T: Serialize>(value: &T) -> SerializationResult<Vec<u8>> {
    serde_json::to_vec(value).map_err(Into::into)
}

/// Deserialize from JSON string
pub fn from_json<T: DeserializeOwned>(json: &str) -> SerializationResult<T> {
    serde_json::from_str(json).map_err(Into::into)
}

/// Deserialize from JSON bytes
pub fn from_json_bytes<T: DeserializeOwned>(bytes: &[u8]) -> SerializationResult<T> {
    serde_json::from_slice(bytes).map_err(Into::into)
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
    }

    #[test]
    fn test_json_roundtrip() {
        let data = TestData {
            name: "Alice".to_string(),
            age: 30,
            active: true,
        };

        let json = to_json(&data).unwrap();
        let decoded: TestData = from_json(&json).unwrap();

        assert_eq!(data, decoded);
    }

    #[test]
    fn test_json_pretty() {
        let data = TestData {
            name: "Bob".to_string(),
            age: 25,
            active: false,
        };

        let pretty = to_json_pretty(&data).unwrap();
        assert!(pretty.contains('\n'));
        assert!(pretty.contains("  "));
    }

    #[test]
    fn test_json_bytes() {
        let data = TestData {
            name: "Charlie".to_string(),
            age: 35,
            active: true,
        };

        let bytes = to_json_bytes(&data).unwrap();
        let decoded: TestData = from_json_bytes(&bytes).unwrap();

        assert_eq!(data, decoded);
    }
}
