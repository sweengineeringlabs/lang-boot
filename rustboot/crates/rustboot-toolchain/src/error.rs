//! Error types for toolchain operations

/// Toolchain error types
#[derive(Debug, thiserror::Error)]
pub enum ToolchainError {
    #[error("Tool error: {0}")]
    ToolError(String),
}
