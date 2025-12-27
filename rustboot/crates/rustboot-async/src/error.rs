//! Async error types

use thiserror::Error;

pub type AsyncResult<T> = Result<T, AsyncError>;

#[derive(Debug, Error)]
pub enum AsyncError {
    #[error("Task join error: {0}")]
    JoinError(String),

    #[error("Task timeout")]
    Timeout,

    #[error("Runtime error: {0}")]
    RuntimeError(String),
}
