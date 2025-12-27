//! Rustboot Observability - Metrics, logging, and tracing
//!
//! This crate provides observability utilities including:
//! - Structured logging via the `Logger` trait
//! - Metrics collection via the `Metrics` trait
//! - Distributed tracing context propagation
//!
//! The `tracing` module re-exports key items from the `tracing` crate
//! for use with `#[traced]` macro from `rustboot-macros`.

pub mod logging;
pub mod metrics;
pub mod tracing;

pub use logging::{Logger, TracingLogger};
// Note: logging::Level is our custom enum, tracing::Level is from tracing crate
pub use logging::Level as LogLevel;
pub use metrics::{Counter, Gauge, InMemoryMetrics, Metrics};
pub use tracing::{TimedSpan, TraceContext};
