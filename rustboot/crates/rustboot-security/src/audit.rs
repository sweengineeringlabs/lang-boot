//! Security auditing module
//!
//! Audit trails, compliance logging, security metrics

use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};

/// Security event types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EventType {
    /// Authentication events
    Login,
    Logout,
    LoginFailed,
    
    /// Authorization events
    PermissionGranted,
    PermissionDenied,
    
    /// Data access events
    DataRead,
    DataWrite,
    DataDelete,
    
    /// Administrative events
    UserCreated,
    UserDeleted,
    RoleChanged,
    
    /// Security events
    SecurityViolation,
    SuspiciousActivity,
    
    /// Custom event
    Custom(String),
}

/// Security event for auditing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityEvent {
    /// Event type
    pub event_type: EventType,
    
    /// Subject (user, system, etc.)
    pub subject: String,
    
    /// Timestamp (Unix epoch seconds)
    pub timestamp: i64,
    
    /// Additional metadata
    pub metadata: serde_json::Value,
    
    /// Event severity
    pub severity: Severity,
    
    /// Resource affected (optional)
    pub resource: Option<String>,
}

/// Event severity levels
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    Info,
    Warning,
    Error,
    Critical,
}

impl SecurityEvent {
    /// Create a new security event
    pub fn new(event_type: EventType, subject: impl Into<String>) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;
        
        Self {
            event_type,
            subject: subject.into(),
            timestamp,
            metadata: serde_json::Value::Null,
            severity: Severity::Info,
            resource: None,
        }
    }
    
    /// Set metadata
    pub fn with_metadata(mut self, metadata: serde_json::Value) -> Self {
        self.metadata = metadata;
        self
    }
    
    /// Set severity
    pub fn with_severity(mut self, severity: Severity) -> Self {
        self.severity = severity;
        self
    }
    
    /// Set resource
    pub fn with_resource(mut self, resource: impl Into<String>) -> Self {
        self.resource = Some(resource.into());
        self
    }
}

/// Audit logger for storing security events
#[derive(Debug, Clone)]
pub struct AuditLogger {
    events: Arc<RwLock<Vec<SecurityEvent>>>,
    max_events: usize,
}

impl AuditLogger {
    /// Create a new audit logger
    pub fn new() -> Self {
        Self::with_capacity(10000)
    }
    
    /// Create a new audit logger with specified capacity
    pub fn with_capacity(max_events: usize) -> Self {
        Self {
            events: Arc::new(RwLock::new(Vec::new())),
            max_events,
        }
    }
    
    /// Log a security event
    pub fn log_event(&self, event: SecurityEvent) -> crate::SecurityResult<()> {
        let mut events = self.events.write()
            .map_err(|e| crate::SecurityError::AuditError(format!("Lock error: {}", e)))?;
        events.push(event);
        Ok(())
    }
    
    /// Log a security event
    pub fn log(&self, event: SecurityEvent) -> crate::SecurityResult<()> {
        let mut events = self.events.write()
            .map_err(|e| crate::SecurityError::AuditError(format!("Lock error: {}", e)))?;
        
        events.push(event);
        
        // Rotate if we exceed max events (remove oldest)
        if events.len() > self.max_events {
            events.remove(0);
        }
        
        Ok(())
    }
    
    /// Get all events
    pub fn get_events(&self) -> Vec<SecurityEvent> {
        self.events.read()
            .map(|e| e.clone())
            .unwrap_or_default()
    }
    
    /// Get events by type
    pub fn get_events_by_type(&self, event_type: &EventType) -> Vec<SecurityEvent> {
        self.events.read()
            .map(|events| {
                events.iter()
                    .filter(|e| &e.event_type == event_type)
                    .cloned()
                    .collect()
            })
            .unwrap_or_default()
    }
    
    /// Get events by subject
    pub fn get_events_by_subject(&self, subject: &str) -> Vec<SecurityEvent> {
        self.events.read()
            .map(|events| {
                events.iter()
                    .filter(|e| e.subject == subject)
                    .cloned()
                    .collect()
            })
            .unwrap_or_default()
    }
    
    /// Get events by severity
    pub fn get_events_by_severity(&self, min_severity: Severity) -> Vec<SecurityEvent> {
        self.events.read()
            .map(|events| {
                events.iter()
                    .filter(|e| e.severity >= min_severity)
                    .cloned()
                    .collect()
            })
            .unwrap_or_default()
    }
    
    /// Get events within a time range
    pub fn get_events_in_range(&self, start: i64, end: i64) -> Vec<SecurityEvent> {
        self.events.read()
            .map(|events| {
                events.iter()
                    .filter(|e| e.timestamp >= start && e.timestamp <= end)
                    .cloned()
                    .collect()
            })
            .unwrap_or_default()
    }
    
    /// Count events
    pub fn count(&self) -> usize {
        self.events.read()
            .map(|e| e.len())
            .unwrap_or(0)
    }
    
    /// Clear all events
    pub fn clear(&self) -> crate::SecurityResult<()> {
        let mut events = self.events.write()
            .map_err(|e| crate::SecurityError::AuditError(format!("Lock error: {}", e)))?;
        
        events.clear();
        Ok(())
    }
}

impl Default for AuditLogger {
    fn default() -> Self {
        Self::new()
    }
}

/// Log security audit event (convenience function)
pub fn audit_event(
    event_type: EventType,
    subject: &str,
    metadata: serde_json::Value,
) -> crate::SecurityResult<()> {
    // This is a simplified implementation that just prints
    // In production, this would write to a persistent store or logging system
    let event = SecurityEvent::new(event_type, subject)
        .with_metadata(metadata);
    
    // For now, just print to stderr as a simple audit trail
    eprintln!("[AUDIT] {} - {}: {:?}", event.timestamp, event.subject, event.event_type);
    
    Ok(())
}

/// Log a login event
pub fn audit_login(subject: &str, success: bool) -> crate::SecurityResult<()> {
    let event_type = if success {
        EventType::Login
    } else {
        EventType::LoginFailed
    };
    
    audit_event(
        event_type,
        subject,
        serde_json::json!({"success": success}),
    )
}

/// Log a permission check event
pub fn audit_permission(subject: &str, permission: &str, granted: bool) -> crate::SecurityResult<()> {
    let event_type = if granted {
        EventType::PermissionGranted
    } else {
        EventType::PermissionDenied
    };
    
    audit_event(
        event_type,
        subject,
        serde_json::json!({"permission": permission, "granted": granted}),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_security_event() {
        let event = SecurityEvent::new(EventType::Login, "user123");
        assert_eq!(event.subject, "user123");
        assert!(matches!(event.event_type, EventType::Login));
        assert_eq!(event.severity, Severity::Info);
    }

    #[test]
    fn test_event_with_metadata() {
        let event = SecurityEvent::new(EventType::Login, "user123")
            .with_metadata(serde_json::json!({"ip": "192.168.1.1"}))
            .with_severity(Severity::Warning)
            .with_resource("api/v1/login");
        
        assert_eq!(event.severity, Severity::Warning);
        assert_eq!(event.resource, Some("api/v1/login".to_string()));
    }

    #[test]
    fn test_audit_logger() {
        let logger = AuditLogger::new();
        
        let event1 = SecurityEvent::new(EventType::Login, "alice");
        let event2 = SecurityEvent::new(EventType::Logout, "bob");
        
        logger.log(event1).unwrap();
        logger.log(event2).unwrap();
        
        assert_eq!(logger.count(), 2);
    }

    #[test]
    fn test_get_events_by_type() {
        let logger = AuditLogger::new();
        
        logger.log(SecurityEvent::new(EventType::Login, "alice")).unwrap();
        logger.log(SecurityEvent::new(EventType::Login, "bob")).unwrap();
        logger.log(SecurityEvent::new(EventType::Logout, "alice")).unwrap();
        
        let login_events = logger.get_events_by_type(&EventType::Login);
        assert_eq!(login_events.len(), 2);
    }

    #[test]
    fn test_get_events_by_subject() {
        let logger = AuditLogger::new();
        
        logger.log(SecurityEvent::new(EventType::Login, "alice")).unwrap();
        logger.log(SecurityEvent::new(EventType::DataRead, "alice")).unwrap();
        logger.log(SecurityEvent::new(EventType::Login, "bob")).unwrap();
        
        let alice_events = logger.get_events_by_subject("alice");
        assert_eq!(alice_events.len(), 2);
    }

    #[test]
    fn test_get_events_by_severity() {
        let logger = AuditLogger::new();
        
        logger.log(SecurityEvent::new(EventType::Login, "alice").with_severity(Severity::Info)).unwrap();
        logger.log(SecurityEvent::new(EventType::SecurityViolation, "bob").with_severity(Severity::Error)).unwrap();
        logger.log(SecurityEvent::new(EventType::SuspiciousActivity, "charlie").with_severity(Severity::Critical)).unwrap();
        
        let critical_events = logger.get_events_by_severity(Severity::Error);
        assert_eq!(critical_events.len(), 2); // Error and Critical
    }

    #[test]
    fn test_audit_logger_capacity() {
        let logger = AuditLogger::with_capacity(5);
        
        // Add 10 events
        for i in 0..10 {
            logger.log(SecurityEvent::new(EventType::Login, format!("user{}", i))).unwrap();
        }
        
        // Should only keep the last 5
        assert_eq!(logger.count(), 5);
    }

    #[test]
    fn test_clear_events() {
        let logger = AuditLogger::new();
        
        logger.log(SecurityEvent::new(EventType::Login, "alice")).unwrap();
        logger.log(SecurityEvent::new(EventType::Logout, "bob")).unwrap();
        
        assert_eq!(logger.count(), 2);
        
        logger.clear().unwrap();
        assert_eq!(logger.count(), 0);
    }

    #[test]
    fn test_audit_event_convenience() {
        // Just test it doesn't panic
        let result = audit_event(
            EventType::Login,
            "test_user",
            serde_json::json!({"test": true}),
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_audit_login() {
        assert!(audit_login("alice", true).is_ok());
        assert!(audit_login("bob", false).is_ok());
    }

    #[test]
    fn test_audit_permission() {
        assert!(audit_permission("alice", "read", true).is_ok());
        assert!(audit_permission("bob", "write", false).is_ok());
    }
}
