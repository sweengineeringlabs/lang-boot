//! Configuration sources

use crate::error::{ConfigError, ConfigResult};
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use std::fs;

/// Trait for configuration sources
pub trait Source {
    /// Load configuration from this source
    fn load<T: DeserializeOwned>(&self) -> ConfigResult<T>;
}

/// File-based configuration source
pub struct FileSource {
    path: String,
    format: FileFormat,
}

#[derive(Debug, Clone, Copy)]
pub enum FileFormat {
    Yaml,
    Toml,
    Json,
}

impl FileSource {
    /// Create a new file source
    pub fn new(path: impl Into<String>, format: FileFormat) -> Self {
        Self {
            path: path.into(),
            format,
        }
    }

    /// Auto-detect format from file extension
    pub fn auto(path: impl Into<String>) -> ConfigResult<Self> {
        let path = path.into();
        let format = if path.ends_with(".yaml") || path.ends_with(".yml") {
            FileFormat::Yaml
        } else if path.ends_with(".toml") {
            FileFormat::Toml
        } else if path.ends_with(".json") {
            FileFormat::Json
        } else {
            return Err(ConfigError::FileLoad(
                "Unknown file format, use .yaml, .toml, or .json".to_string(),
            ));
        };

        Ok(Self { path, format })
    }
}

impl Source for FileSource {
    fn load<T: DeserializeOwned>(&self) -> ConfigResult<T> {
        let content = fs::read_to_string(&self.path)
            .map_err(|e| ConfigError::FileLoad(format!("{}: {}", self.path, e)))?;

        match self.format {
            FileFormat::Yaml => serde_yaml::from_str(&content).map_err(Into::into),
            FileFormat::Toml => toml::from_str(&content).map_err(Into::into),
            FileFormat::Json => serde_json::from_str(&content).map_err(Into::into),
        }
    }
}

/// Environment variable source
pub struct EnvSource {
    prefix: Option<String>,
    separator: String,
}

impl EnvSource {
    /// Create a new env source with optional prefix
    pub fn new(prefix: Option<String>) -> Self {
        Self {
            prefix,
            separator: "_".to_string(),
        }
    }

    /// Set the separator character (default: "_")
    pub fn with_separator(mut self, separator: impl Into<String>) -> Self {
        self.separator = separator.into();
        self
    }
}

impl Source for EnvSource {
    fn load<T: DeserializeOwned>(&self) -> ConfigResult<T> {
        let mut map: HashMap<String, String> = HashMap::new();

        for (key, value) in std::env::vars() {
            if let Some(prefix) = &self.prefix {
                if let Some(stripped) = key.strip_prefix(prefix) {
                    if let Some(stripped) = stripped.strip_prefix(&self.separator) {
                        map.insert(stripped.to_lowercase(), value);
                    }
                }
            } else {
                map.insert(key.to_lowercase(), value);
            }
        }

        serde_json::from_value(serde_json::to_value(map)?)
            .map_err(|e| ConfigError::MergeFailed(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;
    use std::fs::File;
    use std::io::Write;
    use tempfile::TempDir;

    #[derive(Debug, Deserialize, PartialEq)]
    struct TestConfig {
        name: String,
        value: i32,
    }

    #[test]
    fn test_yaml_file_source() {
        let dir = TempDir::new().unwrap();
        let file_path = dir.path().join("config.yaml");
        let mut file = File::create(&file_path).unwrap();
        write!(file, "name: test\nvalue: 42").unwrap();

        let source = FileSource::auto(file_path.to_str().unwrap()).unwrap();
        let config: TestConfig = source.load().unwrap();

        assert_eq!(config.name, "test");
        assert_eq!(config.value, 42);
    }

    #[test]
    fn test_json_file_source() {
        let dir = TempDir::new().unwrap();
        let file_path = dir.path().join("config.json");
        let mut file = File::create(&file_path).unwrap();
        write!(file, r#"{{"name": "test", "value": 42}}"#).unwrap();

        let source = FileSource::auto(file_path.to_str().unwrap()).unwrap();
        let config: TestConfig = source.load().unwrap();

        assert_eq!(config.name, "test");
        assert_eq!(config.value, 42);
    }
}
