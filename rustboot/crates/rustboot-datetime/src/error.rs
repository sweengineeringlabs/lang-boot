//! DateTime error types

use thiserror::Error;

pub type DateTimeResult<T> = Result<T, DateTimeError>;

#[derive(Debug, Error)]
pub enum DateTimeError {
    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("Format error: {0}")]
    FormatError(String),
}
