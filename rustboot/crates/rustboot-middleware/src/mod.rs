//! Middleware framework (L4: Core).
//!
//! Composable middleware pattern for request/response processing pipelines.

pub mod built_in;
pub mod chain;
pub mod traits;

// Re-export main types
pub use built_in::{LoggingMiddleware, MetricsMiddleware, TimingMiddleware};
pub use chain::Pipeline;
pub use traits::{Middleware, MiddlewareError, MiddlewareResult};
