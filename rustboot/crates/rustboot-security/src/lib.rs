//! Security utilities for authentication, authorization, secrets, and auditing.
//!
//! This crate provides purely security-focused features:
//! - **Authentication**: JWT tokens, sessions, API keys
//! - **Authorization**: RBAC, permissions, policies
//! - **Secrets**: Secure secret management and encryption
//! - **Auditing**: Security event logging and compliance

pub mod auth;
pub mod authz;
pub mod secrets;
pub mod audit;

// Re-export commonly used types
pub use auth::*;
pub use authz::*;
pub use secrets::*;
pub use audit::*;

/// Result type for security operations
pub type SecurityResult<T> = Result<T, SecurityError>;

/// Security error types
#[derive(Debug, thiserror::Error)]
pub enum SecurityError {
    #[error("Authentication failed: {0}")]
    AuthenticationFailed(String),
    
    #[error("Authorization denied: {0}")]
    AuthorizationDenied(String),
    
    #[error("Invalid token: {0}")]
    InvalidToken(String),
    
    #[error("Secret not found: {0}")]
    SecretNotFound(String),
    
    #[error("Encryption error: {0}")]
    EncryptionError(String),
    
    #[error("Audit error: {0}")]
    AuditError(String),
}
