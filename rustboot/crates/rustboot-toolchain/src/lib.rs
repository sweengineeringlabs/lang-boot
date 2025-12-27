//! Development tooling and utilities

pub mod error;

pub use error::*;

/// Result type for toolchain operations
pub type ToolchainResult<T> = Result<T, ToolchainError>;
