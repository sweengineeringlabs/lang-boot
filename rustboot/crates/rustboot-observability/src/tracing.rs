//! Distributed tracing support (L4: Core - Observability).
//!
//! Utilities for distributed tracing and span management.
//!
//! This module re-exports key items from the `tracing` crate for use with
//! `rustboot-macros` (e.g., `#[traced]`), and provides additional utilities
//! for distributed tracing context propagation.

use std::time::Instant;

// Re-export tracing crate items for use with #[traced] macro
pub use tracing::{event, span, Level};

/// A timed span for measuring operation duration.
///
/// This is a simple wrapper that tracks timing and fields for a unit of work.
/// For full tracing integration, use the `tracing` crate's `Span` directly
/// or the `#[traced]` macro from `rustboot-macros`.
#[derive(Debug)]
pub struct TimedSpan {
    name: String,
    start: Instant,
    fields: Vec<(String, String)>,
}

impl TimedSpan {
    /// Create a new timed span.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            start: Instant::now(),
            fields: Vec::new(),
        }
    }

    /// Add a field to the span.
    pub fn field(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.fields.push((key.into(), value.into()));
        self
    }

    /// Complete the span and return duration.
    pub fn finish(self) -> std::time::Duration {
        let duration = self.start.elapsed();
        tracing::debug!(
            "TimedSpan '{}' completed in {:?} with fields: {:?}",
            self.name,
            duration,
            self.fields
        );
        duration
    }
}

/// Trace context for distributed tracing.
#[derive(Debug, Clone)]
pub struct TraceContext {
    /// Trace ID.
    pub trace_id: String,
    /// Span ID.
   pub span_id: String,
    /// Parent span ID, if any.
    pub parent_span_id: Option<String>,
}

impl TraceContext {
    /// Create a new root trace context.
    pub fn new() -> Self {
        Self {
            trace_id: uuid::Uuid::new_v4().to_string(),
            span_id: uuid::Uuid::new_v4().to_string(),
            parent_span_id: None,
        }
    }
    
    /// Create a child span context.
    pub fn child(&self) -> Self {
        Self {
            trace_id: self.trace_id.clone(),
            span_id: uuid::Uuid::new_v4().to_string(),
            parent_span_id: Some(self.span_id.clone()),
        }
    }
}

impl Default for TraceContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Macro for creating a timed span with optional fields.
///
/// # Example
///
/// ```rust,ignore
/// use rustboot_observability::timed_span;
///
/// let span = timed_span!("my_operation");
/// // ... do work ...
/// let duration = span.finish();
///
/// // With fields:
/// let span = timed_span!("my_operation", "user_id" => "123", "action" => "create");
/// ```
#[macro_export]
macro_rules! timed_span {
    ($name:expr) => {{
        $crate::tracing::TimedSpan::new($name)
    }};

    ($name:expr, $($key:expr => $value:expr),+ $(,)?) => {{
        let mut span = $crate::tracing::TimedSpan::new($name);
        $(
            span = span.field($key, $value);
        )+
        span
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timed_span_creation() {
        let span = TimedSpan::new("test_operation");
        let duration = span.finish();
        assert!(duration.as_nanos() > 0);
    }

    #[test]
    fn test_timed_span_with_fields() {
        let span = TimedSpan::new("test_operation")
            .field("user_id", "123")
            .field("action", "create");

        let duration = span.finish();
        assert!(duration.as_nanos() > 0);
    }

    #[test]
    fn test_trace_context() {
        let root = TraceContext::new();
        assert!(root.parent_span_id.is_none());

        let child = root.child();
        assert_eq!(child.trace_id, root.trace_id);
        assert_ne!(child.span_id, root.span_id);
        assert_eq!(child.parent_span_id, Some(root.span_id.clone()));
    }

    #[test]
    fn test_tracing_reexports() {
        // Verify tracing crate items are re-exported
        let _level = Level::INFO;
        // span! and event! are macros, just verify they're accessible
    }
}
