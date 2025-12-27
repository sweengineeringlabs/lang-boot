//! UUID error types

use thiserror::Error;

pub type UuidResult<T> = Result<T, UuidError>;

#[derive(Debug, Error)]
pub enum UuidError {
    #[error("Parse error: {0}")]
    ParseError(#[from] uuid::Error),

    #[error("Invalid UUID")]
    InvalidUuid,
}
