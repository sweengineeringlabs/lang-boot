//! Health check traits and types (L4: Core - Health).
//!
//! Abstractions for application health monitoring with liveness and readiness probes.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

/// Health check errors.
#[derive(Debug, thiserror::Error)]
pub enum HealthError {
    /// Health check execution failed.
    #[error("Health check failed: {0}")]
    CheckFailed(String),

    /// Health check timed out.
    #[error("Health check timed out")]
    Timeout,

    /// Dependency unavailable.
    #[error("Dependency unavailable: {0}")]
    DependencyUnavailable(String),

    /// Custom error.
    #[error("{0}")]
    Custom(String),
}

/// Result type for health check operations.
pub type HealthResult<T> = Result<T, HealthError>;

/// Health status of a component or the overall application.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HealthStatus {
    /// Component is healthy and functioning normally.
    Healthy,

    /// Component is degraded but still functional.
    Degraded,

    /// Component is unhealthy and not functioning.
    Unhealthy,
}

impl fmt::Display for HealthStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HealthStatus::Healthy => write!(f, "healthy"),
            HealthStatus::Degraded => write!(f, "degraded"),
            HealthStatus::Unhealthy => write!(f, "unhealthy"),
        }
    }
}

impl HealthStatus {
    /// Returns true if the status is healthy.
    pub fn is_healthy(&self) -> bool {
        matches!(self, HealthStatus::Healthy)
    }

    /// Returns true if the status is degraded.
    pub fn is_degraded(&self) -> bool {
        matches!(self, HealthStatus::Degraded)
    }

    /// Returns true if the status is unhealthy.
    pub fn is_unhealthy(&self) -> bool {
        matches!(self, HealthStatus::Unhealthy)
    }

    /// Combines two health statuses, returning the worse of the two.
    pub fn combine(&self, other: &HealthStatus) -> HealthStatus {
        match (self, other) {
            (HealthStatus::Unhealthy, _) | (_, HealthStatus::Unhealthy) => HealthStatus::Unhealthy,
            (HealthStatus::Degraded, _) | (_, HealthStatus::Degraded) => HealthStatus::Degraded,
            _ => HealthStatus::Healthy,
        }
    }
}

/// Individual health check result with metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckResult {
    /// Name of the health check.
    pub name: String,

    /// Health status.
    pub status: HealthStatus,

    /// Optional message providing additional context.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    /// Optional additional metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,

    /// Timestamp when the check was performed (ISO 8601).
    pub timestamp: String,

    /// Duration of the health check in milliseconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_ms: Option<u64>,
}

impl CheckResult {
    /// Create a new healthy check result.
    pub fn healthy(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            status: HealthStatus::Healthy,
            message: None,
            metadata: None,
            timestamp: chrono::Utc::now().to_rfc3339(),
            duration_ms: None,
        }
    }

    /// Create a new degraded check result.
    pub fn degraded(name: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            status: HealthStatus::Degraded,
            message: Some(message.into()),
            metadata: None,
            timestamp: chrono::Utc::now().to_rfc3339(),
            duration_ms: None,
        }
    }

    /// Create a new unhealthy check result.
    pub fn unhealthy(name: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            status: HealthStatus::Unhealthy,
            message: Some(message.into()),
            metadata: None,
            timestamp: chrono::Utc::now().to_rfc3339(),
            duration_ms: None,
        }
    }

    /// Add a message to the check result.
    pub fn with_message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }

    /// Add metadata to the check result.
    pub fn with_metadata(mut self, key: impl Into<String>, value: serde_json::Value) -> Self {
        self.metadata
            .get_or_insert_with(HashMap::new)
            .insert(key.into(), value);
        self
    }

    /// Set the duration of the health check.
    pub fn with_duration(mut self, duration_ms: u64) -> Self {
        self.duration_ms = Some(duration_ms);
        self
    }
}

/// Trait for implementing health checks.
///
/// Health checks can be used for liveness probes (is the app running?)
/// or readiness probes (is the app ready to serve traffic?).
#[async_trait]
pub trait HealthCheck: Send + Sync {
    /// Get the name of this health check.
    fn name(&self) -> &str;

    /// Execute the health check.
    ///
    /// Returns a CheckResult indicating the health status.
    async fn check(&self) -> CheckResult;

    /// Returns true if this is a critical check.
    ///
    /// Critical checks affect the overall health status.
    /// Non-critical checks can fail without marking the application as unhealthy.
    fn is_critical(&self) -> bool {
        true
    }
}

/// Trait for liveness probes.
///
/// Liveness probes determine if the application is alive and should continue running.
/// If a liveness check fails, the application should be restarted.
#[async_trait]
pub trait LivenessCheck: HealthCheck {
    /// Perform the liveness check.
    async fn is_alive(&self) -> HealthResult<bool> {
        let result = self.check().await;
        Ok(result.status.is_healthy())
    }
}

/// Trait for readiness probes.
///
/// Readiness probes determine if the application is ready to serve traffic.
/// If a readiness check fails, traffic should not be routed to the application.
#[async_trait]
pub trait ReadinessCheck: HealthCheck {
    /// Perform the readiness check.
    async fn is_ready(&self) -> HealthResult<bool> {
        let result = self.check().await;
        Ok(result.status.is_healthy())
    }
}

/// Boxed health check.
pub type BoxedHealthCheck = Box<dyn HealthCheck>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_health_status_combine() {
        assert_eq!(
            HealthStatus::Healthy.combine(&HealthStatus::Healthy),
            HealthStatus::Healthy
        );
        assert_eq!(
            HealthStatus::Healthy.combine(&HealthStatus::Degraded),
            HealthStatus::Degraded
        );
        assert_eq!(
            HealthStatus::Healthy.combine(&HealthStatus::Unhealthy),
            HealthStatus::Unhealthy
        );
        assert_eq!(
            HealthStatus::Degraded.combine(&HealthStatus::Unhealthy),
            HealthStatus::Unhealthy
        );
    }

    #[test]
    fn test_check_result_creation() {
        let result = CheckResult::healthy("test");
        assert_eq!(result.name, "test");
        assert_eq!(result.status, HealthStatus::Healthy);
        assert!(result.message.is_none());

        let result = CheckResult::degraded("test", "warning");
        assert_eq!(result.status, HealthStatus::Degraded);
        assert_eq!(result.message, Some("warning".to_string()));

        let result = CheckResult::unhealthy("test", "error");
        assert_eq!(result.status, HealthStatus::Unhealthy);
        assert_eq!(result.message, Some("error".to_string()));
    }

    #[test]
    fn test_check_result_builder() {
        let result = CheckResult::healthy("test")
            .with_message("all good")
            .with_metadata("version", serde_json::json!("1.0.0"))
            .with_duration(100);

        assert_eq!(result.message, Some("all good".to_string()));
        assert_eq!(result.duration_ms, Some(100));
        assert!(result.metadata.is_some());
    }
}
