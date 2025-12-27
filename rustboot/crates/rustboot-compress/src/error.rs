//! Compression error types

use thiserror::Error;

pub type CompressionResult<T> = Result<T, CompressionError>;

#[derive(Debug, Error)]
pub enum CompressionError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Compression failed: {0}")]
    CompressionFailed(String),
}
