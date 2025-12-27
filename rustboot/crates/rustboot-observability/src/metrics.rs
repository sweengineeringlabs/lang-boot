//! Metrics traits and implementations (L4: Core - Observability).
//!
//! Abstraction for metrics collection.

extern crate self as rustboot_observability;

use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

/// Trait for metrics collection.
pub trait Metrics: Send + Sync {
    /// Create or get a counter.
    fn counter(&self, name: &str) -> Box<dyn Counter>;
    
    /// Create or get a gauge.
    fn gauge(&self, name: &str) -> Box<dyn Gauge>;
    
    /// Record a histogram value.
    fn histogram(&self, name: &str, value: f64);
}

/// A counter metric that only increases.
pub trait Counter: Send + Sync {
    /// Increment the counter by 1.
    fn inc(&self);
    
    /// Increment the counter by a specific amount.
    fn add(&self, value: u64);
    
    /// Get the current value.
    fn get(&self) -> u64;
}

/// A gauge metric that can increase or decrease.
pub trait Gauge: Send + Sync {
    /// Set the gauge to a specific value.
    fn set(&self, value: f64);
    
    /// Increment the gauge.
    fn inc(&self);
    
    /// Decrement the gauge.
    fn dec(&self);
    
    /// Add to the gauge.
    fn add(&self, value: f64);
    
    /// Subtract from the gauge.
    fn sub(&self, value: f64);
    
    /// Get the current value.
    fn get(&self) -> f64;
}

/// Simple in-memory counter implementation.
#[derive(Debug, Default)]
pub struct InMemoryCounter {
    value: Arc<AtomicU64>,
}

impl InMemoryCounter {
    /// Create a new in-memory counter.
    pub fn new() -> Self {
        Self {
            value: Arc::new(AtomicU64::new(0)),
        }
    }
}

impl Counter for InMemoryCounter {
    fn inc(&self) {
        self.value.fetch_add(1, Ordering::Relaxed);
    }
    
    fn add(&self, value: u64) {
        self.value.fetch_add(value, Ordering::Relaxed);
    }
    
    fn get(&self) -> u64 {
        self.value.load(Ordering::Relaxed)
    }
}

/// Simple in-memory gauge implementation.
#[derive(Debug)]
pub struct InMemoryGauge {
    // Using u64 to represent f64 bits for atomic operations
    value: Arc<AtomicU64>,
}

impl InMemoryGauge {
    /// Create a new in-memory gauge.
    pub fn new() -> Self {
        Self {
            value: Arc::new(AtomicU64::new(0)),
        }
    }
}

impl Default for InMemoryGauge {
    fn default() -> Self {
        Self::new()
    }
}

impl Gauge for InMemoryGauge {
    fn set(&self, value: f64) {
        self.value.store(value.to_bits(), Ordering::Relaxed);
    }
    
    fn inc(&self) {
        self.add(1.0);
    }
    
    fn dec(&self) {
        self.sub(1.0);
    }
    
    fn add(&self, value: f64) {
        let current = self.get();
        self.set(current + value);
    }
    
    fn sub(&self, value: f64) {
        let current = self.get();
        self.set(current - value);
    }
    
    fn get(&self) -> f64 {
        f64::from_bits(self.value.load(Ordering::Relaxed))
    }
}

/// Simple in-memory metrics implementation.
#[derive(Debug, Default)]
pub struct InMemoryMetrics {
    // In a real implementation, you'd store these in a concurrent hashmap
}

impl InMemoryMetrics {
    /// Create a new in-memory metrics collector.
    pub fn new() -> Self {
        Self::default()
    }
}

impl Metrics for InMemoryMetrics {
    fn counter(&self, _name: &str) -> Box<dyn Counter> {
        Box::new(InMemoryCounter::new())
    }

    fn gauge(&self, _name: &str) -> Box<dyn Gauge> {
        Box::new(InMemoryGauge::new())
    }


    fn histogram(&self, name: &str, value: f64) {
        tracing::trace!("Histogram {}: {}", name, value);
    }
}

/// Record a duration for a named operation.
///
/// This is a convenience function for use with the `#[timed]` macro.
/// It logs the duration at trace level.
pub fn record_duration(name: &str, duration: std::time::Duration) {
    tracing::trace!(
        operation = name,
        duration_ms = duration.as_millis() as u64,
        "operation completed"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter() {
        let counter = InMemoryCounter::new();
        assert_eq!(counter.get(), 0);
        
        counter.inc();
        assert_eq!(counter.get(), 1);
        
        counter.add(5);
        assert_eq!(counter.get(), 6);
    }

    #[test]
    fn test_gauge() {
        let gauge = InMemoryGauge::new();
        assert_eq!(gauge.get(), 0.0);
        
        gauge.set(10.5);
        assert_eq!(gauge.get(), 10.5);
        
        gauge.inc();
        assert_eq!(gauge.get(), 11.5);
        
        gauge.sub(5.0);
        assert_eq!(gauge.get(), 6.5);
    }

    #[test]
    fn test_metrics() {
        let metrics = InMemoryMetrics::new();
        
        let counter = metrics.counter("requests");
        counter.inc();
        assert_eq!(counter.get(), 1);
        
        let gauge = metrics.gauge("active");
        gauge.set(10.0);
        assert_eq!(gauge.get(), 10.0);
    }
}
