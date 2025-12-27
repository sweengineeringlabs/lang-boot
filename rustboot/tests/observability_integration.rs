//! Integration tests for Rustboot observability

use rustboot::observability::*;

#[test]
fn test_metrics_counter() {
    let metrics = InMemoryMetrics::new();
    
    metrics.increment_counter("requests");
    metrics.increment_counter("requests");
    metrics.increment_counter("requests");
    
    assert_eq!(metrics.get_counter("requests"), 3);
}

#[test]
fn test_metrics_gauge() {
    let metrics = InMemoryMetrics::new();
    
    metrics.set_gauge("temperature", 25.5);
    assert_eq!(metrics.get_gauge("temperature"), 25.5);
    
    metrics.set_gauge("temperature", 30.0);
    assert_eq!(metrics.get_gauge("temperature"), 30.0);
}

#[test]
fn test_metrics_multiple_counters() {
    let metrics = InMemoryMetrics::new();
    
    metrics.increment_counter("requests");
    metrics.increment_counter("errors");
    metrics.increment_counter("requests");
    
    assert_eq!(metrics.get_counter("requests"), 2);
    assert_eq!(metrics.get_counter("errors"), 1);
}

#[test]
fn test_logger_levels() {
    let logger = TracingLogger;
    
    logger.log(Level::Info, "Info message");
    logger.log(Level::Debug, "Debug message");
    logger.log(Level::Error, "Error message");
    
    // Logger should not panic on any level
}

#[test]
fn test_trace_context() {
    let ctx = TraceContext::new();
    
    assert!(!ctx.trace_id().is_empty());
    assert!(!ctx.span_id().is_empty());
}

#[test]
fn test_span_creation() {
    let span = Span::new("test_operation");
    
    assert_eq!(span.name(), "test_operation");
    assert!(!span.span_id().is_empty());
}
