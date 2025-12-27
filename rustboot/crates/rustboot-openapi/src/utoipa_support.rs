//! Utoipa integration support.
//!
//! This module provides integration with the utoipa crate for automatic
//! OpenAPI schema generation from Rust types.

#[cfg(feature = "utoipa")]
pub use utoipa::*;

#[cfg(feature = "utoipa")]
use crate::spec::OpenApiSpec;

/// Convert a utoipa OpenAPI to our OpenApiSpec.
#[cfg(feature = "utoipa")]
pub fn from_utoipa(utoipa_spec: &utoipa::openapi::OpenApi) -> crate::Result<OpenApiSpec> {
    // Serialize utoipa spec to JSON
    let json = serde_json::to_string(utoipa_spec)?;
    // Deserialize to our spec
    let spec: OpenApiSpec = serde_json::from_str(&json)?;
    Ok(spec)
}

/// Convert our OpenApiSpec to utoipa OpenAPI.
#[cfg(feature = "utoipa")]
pub fn to_utoipa(spec: &OpenApiSpec) -> crate::Result<utoipa::openapi::OpenApi> {
    // Serialize our spec to JSON
    let json = serde_json::to_string(spec)?;
    // Deserialize to utoipa spec
    let utoipa_spec: utoipa::openapi::OpenApi = serde_json::from_str(&json)?;
    Ok(utoipa_spec)
}
