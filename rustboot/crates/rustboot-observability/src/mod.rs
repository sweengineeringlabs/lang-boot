//! Observability framework (L4: Core).
//!
//! Metrics, logging, and distributed tracing support.

pub mod logging;
pub mod metrics;
pub mod tracing;

// Re-export main types
pub use logging::{Level, Logger, TracingLogger};
pub use metrics::{Counter, Gauge, InMemoryMetrics, Metrics};
pub use tracing::{Span, TraceContext};
