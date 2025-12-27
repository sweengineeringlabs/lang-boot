//! # Rustboot OpenAPI
//!
//! OpenAPI/Swagger documentation generation for the Rustboot framework.
//!
//! This crate provides tools for generating OpenAPI 3.0 specifications from Rust code,
//! with optional Swagger UI integration for interactive API documentation.
//!
//! ## Features
//!
//! - `utoipa` - Enables utoipa-based OpenAPI generation (recommended)
//! - `swagger-ui` - Enables Swagger UI integration for serving documentation
//! - `yaml` - Enables YAML format output in addition to JSON
//!
//! ## Basic Usage
//!
//! ```rust,ignore
//! use rustboot_openapi::{OpenApiBuilder, OpenApiSpec};
//! use rustboot_macros::{OpenApiSchema, OpenApiPath};
//! use serde::{Deserialize, Serialize};
//!
//! #[derive(Serialize, Deserialize, OpenApiSchema)]
//! struct User {
//!     id: u64,
//!     name: String,
//!     email: String,
//! }
//!
//! #[derive(Serialize, Deserialize, OpenApiSchema)]
//! struct CreateUserRequest {
//!     name: String,
//!     email: String,
//! }
//!
//! // Build OpenAPI specification
//! let spec = OpenApiBuilder::new()
//!     .title("User API")
//!     .version("1.0.0")
//!     .description("API for managing users")
//!     .add_schema::<User>()
//!     .add_schema::<CreateUserRequest>()
//!     .build();
//!
//! // Generate JSON
//! let json = spec.to_json().unwrap();
//!
//! // Generate YAML (with yaml feature)
//! #[cfg(feature = "yaml")]
//! let yaml = spec.to_yaml().unwrap();
//! ```

pub mod spec;
pub mod builder;
pub mod schema;

#[cfg(feature = "utoipa")]
pub mod utoipa_support;

#[cfg(feature = "swagger-ui")]
pub mod swagger_ui;

pub use spec::{OpenApiSpec, Info, Server, Contact, License, PathItem, Operation, Parameter, Response, Schema};
pub use builder::OpenApiBuilder;
pub use schema::{SchemaGenerator, SchemaType};

#[cfg(feature = "utoipa")]
pub use utoipa_support::*;

/// Re-export commonly used types
pub use serde_json::Value as JsonValue;

/// Result type for OpenAPI operations
pub type Result<T> = std::result::Result<T, Error>;

/// OpenAPI error type
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// JSON serialization/deserialization error
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// YAML serialization/deserialization error
    #[cfg(feature = "yaml")]
    #[error("YAML error: {0}")]
    Yaml(#[from] serde_yaml::Error),

    /// Invalid specification error
    #[error("Invalid OpenAPI spec: {0}")]
    InvalidSpec(String),

    /// Missing required field
    #[error("Missing required field: {0}")]
    MissingField(String),
}
