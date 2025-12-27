//! Rustboot Health - Health check infrastructure
//!
//! Provides comprehensive health check abstractions for monitoring application health.
//!
//! # Features
//!
//! - **Liveness Probes**: Determine if the application is alive and running
//! - **Readiness Probes**: Determine if the application is ready to serve traffic
//! - **Custom Health Checks**: Implement custom checks for any dependency
//! - **Health Aggregation**: Combine multiple checks into a single health status
//! - **JSON Responses**: Standard JSON format for health check responses
//! - **Built-in Checks**: Common checks for TCP connections, databases, caches, etc.
//!
//! # Example
//!
//! ```rust
//! use dev_engineeringlabs_rustboot_health::{
//!     HealthAggregator, AlwaysHealthyCheck, FunctionCheck, CheckResult,
//! };
//!
//! #[tokio::main]
//! async fn main() {
//!     // Create a health aggregator
//!     let health = HealthAggregator::new()
//!         .add_check(Box::new(AlwaysHealthyCheck::new("liveness")))
//!         .add_check(Box::new(FunctionCheck::new("database", || async {
//!             // Check database connection
//!             CheckResult::healthy("database")
//!         })))
//!         .with_version("1.0.0");
//!
//!     // Execute health checks
//!     let report = health.check().await;
//!     println!("Health status: {}", report.status);
//!     println!("JSON: {}", report.to_json().unwrap());
//! }
//! ```

pub mod aggregator;
pub mod built_in;
pub mod traits;

pub use aggregator::{HealthAggregator, HealthReport};
pub use built_in::{
    AlwaysHealthyCheck, CompositeCheck, FunctionCheck, PingCheck, TcpConnectionCheck,
};
pub use traits::{
    BoxedHealthCheck, CheckResult, HealthCheck, HealthError, HealthResult, HealthStatus,
    LivenessCheck, ReadinessCheck,
};
