//! Session storage trait and types.

use crate::error::SessionResult;
use crate::session_data::SessionData;
use crate::session_id::SessionId;
use async_trait::async_trait;
use std::time::Duration;

/// Session store trait for different storage backends.
///
/// This trait defines the core operations for session persistence,
/// allowing multiple storage implementations (memory, Redis, database, etc.).
#[async_trait]
pub trait SessionStore: Send + Sync {
    /// Load a session by ID.
    ///
    /// Returns `Ok(Some(data))` if the session exists and is valid,
    /// `Ok(None)` if the session doesn't exist or has expired.
    async fn load(&self, session_id: &SessionId) -> SessionResult<Option<SessionData>>;

    /// Save a session.
    ///
    /// Creates or updates the session with the given ID.
    async fn save(&self, session_id: &SessionId, data: SessionData) -> SessionResult<()>;

    /// Delete a session.
    ///
    /// Removes the session from storage.
    async fn delete(&self, session_id: &SessionId) -> SessionResult<()>;

    /// Check if a session exists.
    ///
    /// Returns `true` if the session exists and is valid (not expired).
    async fn exists(&self, session_id: &SessionId) -> SessionResult<bool> {
        Ok(self.load(session_id).await?.is_some())
    }

    /// Delete all expired sessions.
    ///
    /// This is a maintenance operation that should be called periodically.
    async fn cleanup_expired(&self) -> SessionResult<usize>;

    /// Get the total number of sessions.
    ///
    /// This includes both valid and expired sessions if they haven't been cleaned up.
    async fn count(&self) -> SessionResult<usize>;

    /// Delete all sessions.
    ///
    /// This is primarily useful for testing and maintenance.
    async fn clear(&self) -> SessionResult<()>;
}

/// Session configuration.
#[derive(Debug, Clone)]
pub struct SessionConfig {
    /// Default session expiration time.
    pub default_ttl: Option<Duration>,

    /// Cookie name for session ID.
    pub cookie_name: String,

    /// Cookie path.
    pub cookie_path: String,

    /// Cookie domain.
    pub cookie_domain: Option<String>,

    /// Cookie secure flag (HTTPS only).
    pub cookie_secure: bool,

    /// Cookie HTTP only flag.
    pub cookie_http_only: bool,

    /// Cookie SameSite attribute.
    pub cookie_same_site: SameSite,

    /// Auto-cleanup interval for expired sessions.
    pub cleanup_interval: Option<Duration>,
}

impl Default for SessionConfig {
    fn default() -> Self {
        Self {
            default_ttl: Some(Duration::from_secs(3600 * 24)), // 24 hours
            cookie_name: "session_id".to_string(),
            cookie_path: "/".to_string(),
            cookie_domain: None,
            cookie_secure: true,
            cookie_http_only: true,
            cookie_same_site: SameSite::Lax,
            cleanup_interval: Some(Duration::from_secs(3600)), // 1 hour
        }
    }
}

impl SessionConfig {
    /// Create a new session configuration with defaults.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the default TTL.
    pub fn with_ttl(mut self, ttl: Duration) -> Self {
        self.default_ttl = Some(ttl);
        self
    }

    /// Set the cookie name.
    pub fn with_cookie_name(mut self, name: impl Into<String>) -> Self {
        self.cookie_name = name.into();
        self
    }

    /// Set the cookie domain.
    pub fn with_cookie_domain(mut self, domain: impl Into<String>) -> Self {
        self.cookie_domain = Some(domain.into());
        self
    }

    /// Set the cookie secure flag.
    pub fn with_cookie_secure(mut self, secure: bool) -> Self {
        self.cookie_secure = secure;
        self
    }

    /// Set the cleanup interval.
    pub fn with_cleanup_interval(mut self, interval: Duration) -> Self {
        self.cleanup_interval = Some(interval);
        self
    }
}

/// Cookie SameSite attribute.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SameSite {
    /// Strict mode - cookie only sent in first-party context.
    Strict,
    /// Lax mode - cookie sent with top-level navigations.
    Lax,
    /// None mode - cookie sent in all contexts (requires Secure).
    None,
}

impl SameSite {
    /// Convert to string representation.
    pub fn as_str(&self) -> &'static str {
        match self {
            SameSite::Strict => "Strict",
            SameSite::Lax => "Lax",
            SameSite::None => "None",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_config_default() {
        let config = SessionConfig::default();

        assert_eq!(config.cookie_name, "session_id");
        assert_eq!(config.cookie_path, "/");
        assert!(config.cookie_secure);
        assert!(config.cookie_http_only);
        assert_eq!(config.cookie_same_site, SameSite::Lax);
        assert!(config.default_ttl.is_some());
    }

    #[test]
    fn test_session_config_builder() {
        let config = SessionConfig::new()
            .with_ttl(Duration::from_secs(7200))
            .with_cookie_name("my_session")
            .with_cookie_domain("example.com")
            .with_cookie_secure(false)
            .with_cleanup_interval(Duration::from_secs(1800));

        assert_eq!(config.default_ttl, Some(Duration::from_secs(7200)));
        assert_eq!(config.cookie_name, "my_session");
        assert_eq!(config.cookie_domain, Some("example.com".to_string()));
        assert!(!config.cookie_secure);
        assert_eq!(config.cleanup_interval, Some(Duration::from_secs(1800)));
    }

    #[test]
    fn test_same_site_as_str() {
        assert_eq!(SameSite::Strict.as_str(), "Strict");
        assert_eq!(SameSite::Lax.as_str(), "Lax");
        assert_eq!(SameSite::None.as_str(), "None");
    }
}
