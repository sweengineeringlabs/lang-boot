//! Rustboot Config - Configuration management
//!
//! Load configuration from multiple sources (YAML, TOML, JSON, ENV)
//! with hierarchical merging and type-safe deserialization.

pub mod error;
pub mod loader;
pub mod source;
pub mod traits;

pub use error::{ConfigError, ConfigResult};
pub use loader::ConfigLoader;
pub use source::{EnvSource, FileSource, Source};
pub use traits::Mergeable;
