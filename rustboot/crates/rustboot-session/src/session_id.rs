//! Session ID generation and validation.

use crate::error::{SessionError, SessionResult};
use std::fmt;
use uuid::Uuid;

/// Session identifier.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SessionId(String);

impl SessionId {
    /// Generate a new random session ID.
    pub fn generate() -> Self {
        Self(Uuid::new_v4().to_string())
    }

    /// Create a session ID from a string.
    pub fn from_string(id: impl Into<String>) -> SessionResult<Self> {
        let id = id.into();

        // Validate the session ID format (UUID v4)
        if Uuid::parse_str(&id).is_ok() {
            Ok(Self(id))
        } else {
            Err(SessionError::InvalidSessionId(id))
        }
    }

    /// Get the session ID as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Convert to String.
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for SessionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<SessionId> for String {
    fn from(id: SessionId) -> Self {
        id.0
    }
}

impl TryFrom<String> for SessionId {
    type Error = SessionError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        SessionId::from_string(s)
    }
}

impl TryFrom<&str> for SessionId {
    type Error = SessionError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        SessionId::from_string(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_session_id() {
        let id1 = SessionId::generate();
        let id2 = SessionId::generate();

        // IDs should be unique
        assert_ne!(id1, id2);

        // Should be valid UUIDs
        assert!(Uuid::parse_str(id1.as_str()).is_ok());
        assert!(Uuid::parse_str(id2.as_str()).is_ok());
    }

    #[test]
    fn test_from_string_valid() {
        let uuid = Uuid::new_v4().to_string();
        let result = SessionId::from_string(uuid.clone());

        assert!(result.is_ok());
        assert_eq!(result.unwrap().as_str(), uuid);
    }

    #[test]
    fn test_from_string_invalid() {
        let result = SessionId::from_string("invalid-session-id");
        assert!(result.is_err());

        match result {
            Err(SessionError::InvalidSessionId(_)) => {},
            _ => panic!("Expected InvalidSessionId error"),
        }
    }

    #[test]
    fn test_display() {
        let id = SessionId::generate();
        let display = format!("{}", id);

        assert_eq!(display, id.as_str());
    }
}
