//! YAML serialization helpers

use crate::error::SerializationResult;
use serde::{de::DeserializeOwned, Serialize};

/// Serialize to YAML string
pub fn to_yaml<T: Serialize>(value: &T) -> SerializationResult<String> {
    serde_yaml::to_string(value).map_err(Into::into)
}

/// Deserialize from YAML string
pub fn from_yaml<T: DeserializeOwned>(yaml: &str) -> SerializationResult<T> {
    serde_yaml::from_str(yaml).map_err(Into::into)
}

/// Convert YAML to JSON (useful for validation and format conversion)
pub fn yaml_to_json(yaml: &str) -> SerializationResult<String> {
    let value: serde_yaml::Value = from_yaml(yaml)?;
    serde_json::to_string_pretty(&value).map_err(Into::into)
}

/// Convert JSON to YAML
pub fn json_to_yaml(json: &str) -> SerializationResult<String> {
    let value: serde_json::Value = serde_json::from_str(json)?;
    to_yaml(&value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Config {
        name: String,
        port: u16,
        debug: bool,
    }

    #[test]
    fn test_yaml_roundtrip() {
        let config = Config {
            name: "test-server".to_string(),
            port: 8080,
            debug: true,
        };

        let yaml = to_yaml(&config).unwrap();
        let decoded: Config = from_yaml(&yaml).unwrap();

        assert_eq!(config, decoded);
    }

    #[test]
    fn test_yaml_to_json() {
        let yaml = r#"
name: test-server
port: 8080
debug: true
"#;
        let json = yaml_to_json(yaml).unwrap();

        assert!(json.contains("test-server"));
        assert!(json.contains("8080"));
        assert!(json.contains("true"));
    }

    #[test]
    fn test_json_to_yaml() {
        let json = r#"{"name":"test-server","port":8080,"debug":true}"#;
        let yaml = json_to_yaml(json).unwrap();

        assert!(yaml.contains("test-server"));
        assert!(yaml.contains("8080"));
        assert!(yaml.contains("true"));
    }
}
