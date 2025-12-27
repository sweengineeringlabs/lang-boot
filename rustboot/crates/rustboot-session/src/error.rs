//! Session error types.

/// Session error types.
#[derive(Debug, thiserror::Error)]
pub enum SessionError {
    /// Session not found.
    #[error("Session not found")]
    NotFound,

    /// Session expired.
    #[error("Session expired")]
    Expired,

    /// Invalid session ID.
    #[error("Invalid session ID: {0}")]
    InvalidSessionId(String),

    /// Serialization error.
    #[error("Serialization error: {0}")]
    Serialization(String),

    /// Storage backend error.
    #[error("Storage error: {0}")]
    Storage(String),

    /// Session data error.
    #[error("Session data error: {0}")]
    DataError(String),

    /// Custom error.
    #[error("{0}")]
    Custom(String),
}

/// Result type for session operations.
pub type SessionResult<T> = Result<T, SessionError>;
