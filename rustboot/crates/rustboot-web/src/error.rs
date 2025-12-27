//! Error types for web operations.

/// Result type for web operations.
pub type WebResult<T> = Result<T, WebError>;

/// Errors that can occur in web operations.
#[derive(Debug, thiserror::Error)]
pub enum WebError {
    /// Route not found.
    #[error("Route not found: {0}")]
    NotFound(String),

    /// Method not allowed for route.
    #[error("Method not allowed: {0}")]
    MethodNotAllowed(String),

    /// Invalid request.
    #[error("Invalid request: {0}")]
    InvalidRequest(String),

    /// Handler error.
    #[error("Handler error: {0}")]
    HandlerError(String),

    /// Middleware error.
    #[error("Middleware error: {0}")]
    MiddlewareError(String),

    /// JSON parsing error.
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    /// IO error.
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    /// Server error.
    #[error("Server error: {0}")]
    ServerError(String),

    /// Custom error.
    #[error("{0}")]
    Custom(String),
}

impl WebError {
    /// Create a new custom error.
    pub fn custom<S: Into<String>>(msg: S) -> Self {
        Self::Custom(msg.into())
    }

    /// Create a new handler error.
    pub fn handler<S: Into<String>>(msg: S) -> Self {
        Self::HandlerError(msg.into())
    }

    /// Create a new invalid request error.
    pub fn invalid_request<S: Into<String>>(msg: S) -> Self {
        Self::InvalidRequest(msg.into())
    }

    /// Get the HTTP status code for this error.
    pub fn status_code(&self) -> u16 {
        match self {
            Self::NotFound(_) => 404,
            Self::MethodNotAllowed(_) => 405,
            Self::InvalidRequest(_) => 400,
            Self::JsonError(_) => 400,
            Self::HandlerError(_) => 500,
            Self::MiddlewareError(_) => 500,
            Self::IoError(_) => 500,
            Self::ServerError(_) => 500,
            Self::Custom(_) => 500,
        }
    }
}
