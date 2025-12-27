//! Rustboot Serialization - Multi-format serialization helpers
//!
//! Simplified serialization and deserialization for common formats:
//! - JSON - Typed serialization/deserialization
//! - MessagePack - Binary serialization format
//! - CSV - Typed CSV parsing and writing
//! - YAML - YAML serialization with JSON conversion

pub mod csv;
pub mod error;
pub mod json;
pub mod msgpack;
pub mod yaml;

pub use error::{SerializationError, SerializationResult};

// JSON exports
pub use json::{from_json, from_json_bytes, to_json, to_json_bytes, to_json_pretty};

// MessagePack exports
pub use msgpack::{from_msgpack, to_msgpack};

// CSV exports
pub use csv::{from_csv, from_csv_with_delimiter, to_csv};

// YAML exports
pub use yaml::{from_yaml, json_to_yaml, to_yaml, yaml_to_json};
