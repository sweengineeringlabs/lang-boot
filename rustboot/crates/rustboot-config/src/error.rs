//! Configuration error types

use thiserror::Error;

pub type ConfigResult<T> = Result<T, ConfigError>;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Failed to load config from file: {0}")]
    FileLoad(String),

    #[error("Failed to parse YAML: {0}")]
    YamlParse(#[from] serde_yaml::Error),

    #[error("Failed to parse TOML: {0}")]
    TomlParse(#[from] toml::de::Error),

    #[error("Failed to parse JSON: {0}")]
    JsonParse(#[from] serde_json::Error),

    #[error("Environment variable not found: {0}")]
    EnvNotFound(String),

    #[error("Failed to merge configs: {0}")]
    MergeFailed(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
