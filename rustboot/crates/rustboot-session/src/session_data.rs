//! Session data structures.

use crate::error::{SessionError, SessionResult};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Session data container.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionData {
    /// Session values stored as JSON.
    values: HashMap<String, Value>,

    /// When the session was created.
    #[serde(default = "default_created_at")]
    created_at: u64,

    /// When the session was last accessed.
    #[serde(default = "default_last_accessed")]
    last_accessed: u64,

    /// Session expiration time in seconds from creation.
    #[serde(default)]
    expires_in: Option<u64>,
}

fn default_created_at() -> u64 {
    current_timestamp()
}

fn default_last_accessed() -> u64 {
    current_timestamp()
}

fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
}

impl SessionData {
    /// Create a new session data container.
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            created_at: current_timestamp(),
            last_accessed: current_timestamp(),
            expires_in: None,
        }
    }

    /// Create a new session with expiration.
    pub fn with_expiration(ttl: Duration) -> Self {
        Self {
            values: HashMap::new(),
            created_at: current_timestamp(),
            last_accessed: current_timestamp(),
            expires_in: Some(ttl.as_secs()),
        }
    }

    /// Get a value from the session.
    pub fn get<T>(&self, key: &str) -> SessionResult<Option<T>>
    where
        T: for<'de> Deserialize<'de>,
    {
        match self.values.get(key) {
            Some(value) => {
                let result = serde_json::from_value(value.clone())
                    .map_err(|e| SessionError::Serialization(e.to_string()))?;
                Ok(Some(result))
            }
            None => Ok(None),
        }
    }

    /// Set a value in the session.
    pub fn set<T>(&mut self, key: impl Into<String>, value: T) -> SessionResult<()>
    where
        T: Serialize,
    {
        let json_value = serde_json::to_value(value)
            .map_err(|e| SessionError::Serialization(e.to_string()))?;
        self.values.insert(key.into(), json_value);
        self.touch();
        Ok(())
    }

    /// Remove a value from the session.
    pub fn remove(&mut self, key: &str) -> Option<Value> {
        self.touch();
        self.values.remove(key)
    }

    /// Check if a key exists.
    pub fn contains(&self, key: &str) -> bool {
        self.values.contains_key(key)
    }

    /// Clear all session data.
    pub fn clear(&mut self) {
        self.values.clear();
        self.touch();
    }

    /// Get all keys in the session.
    pub fn keys(&self) -> Vec<&String> {
        self.values.keys().collect()
    }

    /// Check if the session is empty.
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Get the number of values in the session.
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Update the last accessed timestamp.
    pub fn touch(&mut self) {
        self.last_accessed = current_timestamp();
    }

    /// Check if the session has expired.
    pub fn is_expired(&self) -> bool {
        if let Some(expires_in) = self.expires_in {
            let now = current_timestamp();
            let expiry_time = self.created_at + expires_in;
            now >= expiry_time
        } else {
            false
        }
    }

    /// Get the session creation timestamp.
    pub fn created_at(&self) -> u64 {
        self.created_at
    }

    /// Get the last accessed timestamp.
    pub fn last_accessed(&self) -> u64 {
        self.last_accessed
    }

    /// Get the expiration duration.
    pub fn expires_in(&self) -> Option<Duration> {
        self.expires_in.map(Duration::from_secs)
    }

    /// Set the expiration duration.
    pub fn set_expiration(&mut self, ttl: Duration) {
        self.expires_in = Some(ttl.as_secs());
    }

    /// Remove expiration.
    pub fn remove_expiration(&mut self) {
        self.expires_in = None;
    }

    /// Serialize to JSON string.
    pub fn to_json(&self) -> SessionResult<String> {
        serde_json::to_string(self)
            .map_err(|e| SessionError::Serialization(e.to_string()))
    }

    /// Deserialize from JSON string.
    pub fn from_json(json: &str) -> SessionResult<Self> {
        serde_json::from_str(json)
            .map_err(|e| SessionError::Serialization(e.to_string()))
    }
}

impl Default for SessionData {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    struct User {
        id: u64,
        name: String,
    }

    #[test]
    fn test_session_data_basic() {
        let mut session = SessionData::new();

        session.set("user_id", 42u64).unwrap();
        session.set("username", "alice".to_string()).unwrap();

        let user_id: u64 = session.get("user_id").unwrap().unwrap();
        let username: String = session.get("username").unwrap().unwrap();

        assert_eq!(user_id, 42);
        assert_eq!(username, "alice");
        assert_eq!(session.len(), 2);
    }

    #[test]
    fn test_session_data_complex_types() {
        let mut session = SessionData::new();

        let user = User {
            id: 1,
            name: "Bob".to_string(),
        };

        session.set("user", user.clone()).unwrap();

        let retrieved: User = session.get("user").unwrap().unwrap();
        assert_eq!(retrieved, user);
    }

    #[test]
    fn test_session_data_remove() {
        let mut session = SessionData::new();

        session.set("key", "value".to_string()).unwrap();
        assert!(session.contains("key"));

        session.remove("key");
        assert!(!session.contains("key"));
    }

    #[test]
    fn test_session_data_clear() {
        let mut session = SessionData::new();

        session.set("key1", "value1".to_string()).unwrap();
        session.set("key2", "value2".to_string()).unwrap();

        assert_eq!(session.len(), 2);

        session.clear();
        assert_eq!(session.len(), 0);
        assert!(session.is_empty());
    }

    #[test]
    fn test_session_expiration() {
        let session = SessionData::with_expiration(Duration::from_secs(3600));

        assert!(!session.is_expired());
        assert_eq!(session.expires_in(), Some(Duration::from_secs(3600)));
    }

    #[test]
    fn test_session_serialization() {
        let mut session = SessionData::new();
        session.set("key", "value".to_string()).unwrap();

        let json = session.to_json().unwrap();
        let deserialized = SessionData::from_json(&json).unwrap();

        let value: String = deserialized.get("key").unwrap().unwrap();
        assert_eq!(value, "value");
    }

    #[test]
    fn test_session_touch() {
        let mut session = SessionData::new();
        let initial_accessed = session.last_accessed();

        // Sleep for more than 1 second to ensure timestamp changes
        std::thread::sleep(Duration::from_millis(1100));
        session.touch();

        assert!(session.last_accessed() > initial_accessed);
    }
}
