//! Rate limiting error types

use thiserror::Error;

pub type RateLimitResult<T> = Result<T, RateLimitError>;

#[derive(Debug, Error)]
pub enum RateLimitError {
    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),
}
