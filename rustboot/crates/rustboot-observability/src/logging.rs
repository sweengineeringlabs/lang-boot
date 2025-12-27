//! Structured logging abstractions (L4: Core - Observability).
//!
//! Trait-based logging with structured fields.

use std::collections::HashMap;

/// Log level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Level {
    /// Trace level (most verbose).
    Trace,
    /// Debug level.
    Debug,
    /// Info level.
    Info,
    /// Warning level.
    Warn,
    /// Error level.
    Error,
}

/// A log field value.
#[derive(Debug, Clone)]
pub enum Value {
    /// String value.
    String(String),
    /// Integer value.
    Int(i64),
    /// Float value.
    Float(f64),
    /// Boolean value.
    Bool(bool),
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        Value::String(s)
    }
}

impl From<&str> for Value {
    fn from(s: &str) -> Self {
        Value::String(s.to_string())
    }
}

impl From<i64> for Value {
    fn from(i: i64) -> Self {
        Value::Int(i)
    }
}

impl From<f64> for Value {
    fn from(f: f64) -> Self {
        Value::Float(f)
    }
}

impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Value::Bool(b)
    }
}

/// Trait for structured logging.
pub trait Logger: Send + Sync {
    /// Log a message with structured fields.
    fn log(&self, level: Level, message: &str, fields: &[(&str, Value)]);
    
    /// Log at trace level.
    fn trace(&self, message: &str, fields: &[(&str, Value)]) {
        self.log(Level::Trace, message, fields);
    }
    
    /// Log at debug level.
    fn debug(&self, message: &str, fields: &[(&str, Value)]) {
        self.log(Level::Debug, message, fields);
    }
    
    /// Log at info level.
    fn info(&self, message: &str, fields: &[(&str, Value)]) {
        self.log(Level::Info, message, fields);
    }
    
    /// Log at warn level.
    fn warn(&self, message: &str, fields: &[(&str, Value)]) {
        self.log(Level::Warn, message, fields);
    }
    
    /// Log at error level.
    fn error(&self, message: &str, fields: &[(&str, Value)]) {
        self.log(Level::Error, message, fields);
    }
}

/// Logger that uses the tracing crate.
#[derive(Debug, Default)]
pub struct TracingLogger;

impl TracingLogger {
    /// Create a new tracing logger.
    pub fn new() -> Self {
        Self
    }
}

impl Logger for TracingLogger {
    fn log(&self, level: Level, message: &str, fields: &[(&str, Value)]) {
        // Convert fields to a format string
        let field_strs: Vec<String> = fields
            .iter()
            .map(|(k, v)| format!("{}={:?}", k, v))
            .collect();
        
        let full_message = if field_strs.is_empty() {
            message.to_string()
        } else {
            format!("{} [{}]", message, field_strs.join(", "))
        };

        match level {
            Level::Trace => tracing::trace!("{}", full_message),
            Level::Debug => tracing::debug!("{}", full_message),
            Level::Info => tracing::info!("{}", full_message),
            Level::Warn => tracing::warn!("{}", full_message),
            Level::Error => tracing::error!("{}", full_message),
        }
    }
}

/// In-memory logger for testing.
#[derive(Debug, Default)]
pub struct InMemoryLogger {
    logs: std::sync::Arc<std::sync::Mutex<Vec<LogEntry>>>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct LogEntry {
    level: Level,
    message: String,
    fields: HashMap<String, String>,
}

impl InMemoryLogger {
    /// Create a new in-memory logger.
    pub fn new() -> Self {
        Self {
            logs: std::sync::Arc::new(std::sync::Mutex::new(Vec::new())),
        }
    }
    
    /// Get all logged entries.
    pub fn entries(&self) -> Vec<(Level, String)> {
        self.logs
            .lock()
            .unwrap()
            .iter()
            .map(|e| (e.level, e.message.clone()))
            .collect()
    }
    
    /// Clear all logs.
    pub fn clear(&self) {
        self.logs.lock().unwrap().clear();
    }
}

impl Logger for InMemoryLogger {
    fn log(&self, level: Level, message: &str, fields: &[(&str, Value)]) {
        let entry = LogEntry {
            level,
            message: message.to_string(),
            fields: fields
                .iter()
                .map(|(k, v)| (k.to_string(), format!("{:?}", v)))
                .collect(),
        };
        
        self.logs.lock().unwrap().push(entry);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in_memory_logger() {
        let logger = InMemoryLogger::new();
        
        logger.info("Test message", &[("key", Value::String("value".to_string()))]);
        logger.error("Error occurred", &[]);
        
        let entries = logger.entries();
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].0, Level::Info);
        assert_eq!(entries[1].0, Level::Error);
    }

    #[test]
    fn test_level_ordering() {
        assert!(Level::Error > Level::Warn);
        assert!(Level::Warn > Level::Info);
        assert!(Level::Info > Level::Debug);
        assert!(Level::Debug > Level::Trace);
    }
}
