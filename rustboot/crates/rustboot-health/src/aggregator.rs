//! Health check aggregation and reporting.
//!
//! Aggregates multiple health checks into a single health status report.

use crate::traits::{BoxedHealthCheck, CheckResult, HealthStatus};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;

/// Aggregated health report containing all check results.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthReport {
    /// Overall health status.
    pub status: HealthStatus,

    /// Individual check results.
    pub checks: HashMap<String, CheckResult>,

    /// Timestamp when the report was generated (ISO 8601).
    pub timestamp: String,

    /// Total duration to execute all checks in milliseconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_ms: Option<u64>,

    /// Application version or identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

impl HealthReport {
    /// Create a new health report.
    pub fn new(status: HealthStatus, checks: HashMap<String, CheckResult>) -> Self {
        Self {
            status,
            checks,
            timestamp: chrono::Utc::now().to_rfc3339(),
            duration_ms: None,
            version: None,
        }
    }

    /// Set the version of the application.
    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }

    /// Set the total duration.
    pub fn with_duration(mut self, duration_ms: u64) -> Self {
        self.duration_ms = Some(duration_ms);
        self
    }

    /// Convert the health report to JSON.
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Convert the health report to compact JSON.
    pub fn to_json_compact(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}

/// Health check aggregator that manages and executes multiple health checks.
pub struct HealthAggregator {
    checks: Vec<Arc<BoxedHealthCheck>>,
    version: Option<String>,
}

impl HealthAggregator {
    /// Create a new health aggregator.
    pub fn new() -> Self {
        Self {
            checks: Vec::new(),
            version: None,
        }
    }

    /// Add a health check to the aggregator.
    pub fn add_check(mut self, check: BoxedHealthCheck) -> Self {
        self.checks.push(Arc::new(check));
        self
    }

    /// Set the application version.
    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }

    /// Execute all health checks and aggregate results.
    pub async fn check(&self) -> HealthReport {
        let start = Instant::now();
        let mut results = HashMap::new();
        let mut overall_status = HealthStatus::Healthy;

        for check in &self.checks {
            let result = check.check().await;

            // Only critical checks affect overall status
            if check.is_critical() {
                overall_status = overall_status.combine(&result.status);
            }

            results.insert(result.name.clone(), result);
        }

        let duration = start.elapsed();
        let mut report = HealthReport::new(overall_status, results)
            .with_duration(duration.as_millis() as u64);

        if let Some(version) = &self.version {
            report = report.with_version(version.clone());
        }

        report
    }

    /// Execute all health checks in parallel and aggregate results.
    pub async fn check_parallel(&self) -> HealthReport {
        let start = Instant::now();
        let mut handles = Vec::new();

        for check in &self.checks {
            let check = Arc::clone(check);
            let handle = tokio::spawn(async move {
                let result = check.check().await;
                let is_critical = check.is_critical();
                (result, is_critical)
            });
            handles.push(handle);
        }

        let mut results = HashMap::new();
        let mut overall_status = HealthStatus::Healthy;

        for handle in handles {
            if let Ok((result, is_critical)) = handle.await {
                // Only critical checks affect overall status
                if is_critical {
                    overall_status = overall_status.combine(&result.status);
                }
                results.insert(result.name.clone(), result);
            }
        }

        let duration = start.elapsed();
        let mut report = HealthReport::new(overall_status, results)
            .with_duration(duration.as_millis() as u64);

        if let Some(version) = &self.version {
            report = report.with_version(version.clone());
        }

        report
    }

    /// Get the number of registered health checks.
    pub fn check_count(&self) -> usize {
        self.checks.len()
    }
}

impl Default for HealthAggregator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::HealthCheck;
    use async_trait::async_trait;

    struct AlwaysHealthyCheck;

    #[async_trait]
    impl HealthCheck for AlwaysHealthyCheck {
        fn name(&self) -> &str {
            "always_healthy"
        }

        async fn check(&self) -> CheckResult {
            CheckResult::healthy(self.name())
        }
    }

    struct AlwaysHealthyCheck2;

    #[async_trait]
    impl HealthCheck for AlwaysHealthyCheck2 {
        fn name(&self) -> &str {
            "always_healthy_2"
        }

        async fn check(&self) -> CheckResult {
            CheckResult::healthy(self.name())
        }
    }

    struct AlwaysUnhealthyCheck;

    #[async_trait]
    impl HealthCheck for AlwaysUnhealthyCheck {
        fn name(&self) -> &str {
            "always_unhealthy"
        }

        async fn check(&self) -> CheckResult {
            CheckResult::unhealthy(self.name(), "always fails")
        }
    }

    struct NonCriticalCheck;

    #[async_trait]
    impl HealthCheck for NonCriticalCheck {
        fn name(&self) -> &str {
            "non_critical"
        }

        async fn check(&self) -> CheckResult {
            CheckResult::unhealthy(self.name(), "non-critical failure")
        }

        fn is_critical(&self) -> bool {
            false
        }
    }

    #[tokio::test]
    async fn test_aggregator_all_healthy() {
        let aggregator = HealthAggregator::new()
            .add_check(Box::new(AlwaysHealthyCheck))
            .with_version("1.0.0");

        let report = aggregator.check().await;
        assert_eq!(report.status, HealthStatus::Healthy);
        assert_eq!(report.checks.len(), 1);
        assert_eq!(report.version, Some("1.0.0".to_string()));
    }

    #[tokio::test]
    async fn test_aggregator_with_unhealthy() {
        let aggregator = HealthAggregator::new()
            .add_check(Box::new(AlwaysHealthyCheck))
            .add_check(Box::new(AlwaysUnhealthyCheck));

        let report = aggregator.check().await;
        assert_eq!(report.status, HealthStatus::Unhealthy);
        assert_eq!(report.checks.len(), 2);
    }

    #[tokio::test]
    async fn test_aggregator_non_critical_check() {
        let aggregator = HealthAggregator::new()
            .add_check(Box::new(AlwaysHealthyCheck))
            .add_check(Box::new(NonCriticalCheck));

        let report = aggregator.check().await;
        // Non-critical checks don't affect overall status
        assert_eq!(report.status, HealthStatus::Healthy);
        assert_eq!(report.checks.len(), 2);
    }

    #[tokio::test]
    async fn test_aggregator_parallel() {
        let aggregator = HealthAggregator::new()
            .add_check(Box::new(AlwaysHealthyCheck))
            .add_check(Box::new(AlwaysHealthyCheck2));

        let report = aggregator.check_parallel().await;
        assert_eq!(report.status, HealthStatus::Healthy);
        assert_eq!(report.checks.len(), 2);
    }

    #[test]
    fn test_health_report_json() {
        let mut checks = HashMap::new();
        checks.insert(
            "test".to_string(),
            CheckResult::healthy("test"),
        );

        let report = HealthReport::new(HealthStatus::Healthy, checks)
            .with_version("1.0.0");

        let json = report.to_json().unwrap();
        println!("JSON output: {}", json);
        assert!(json.contains("\"status\": \"healthy\"") || json.contains("\"status\":\"healthy\""));
        assert!(json.contains("\"version\": \"1.0.0\"") || json.contains("\"version\":\"1.0.0\""));
    }
}
