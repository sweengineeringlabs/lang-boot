//! Built-in health checks for common dependencies.
//!
//! Provides ready-to-use health checks for databases, caches, and other common services.

use crate::traits::{CheckResult, HealthCheck, LivenessCheck, ReadinessCheck};
use async_trait::async_trait;
use std::sync::Arc;
use std::time::{Duration, Instant};

/// Simple liveness check that always returns healthy.
///
/// Useful for basic liveness probes that just verify the application is running.
pub struct AlwaysHealthyCheck {
    name: String,
}

impl AlwaysHealthyCheck {
    /// Create a new always-healthy check.
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

#[async_trait]
impl HealthCheck for AlwaysHealthyCheck {
    fn name(&self) -> &str {
        &self.name
    }

    async fn check(&self) -> CheckResult {
        CheckResult::healthy(&self.name)
    }
}

#[async_trait]
impl LivenessCheck for AlwaysHealthyCheck {}

/// Custom function-based health check.
pub struct FunctionCheck<F, Fut>
where
    F: Fn() -> Fut + Send + Sync,
    Fut: std::future::Future<Output = CheckResult> + Send,
{
    name: String,
    check_fn: F,
    critical: bool,
}

impl<F, Fut> FunctionCheck<F, Fut>
where
    F: Fn() -> Fut + Send + Sync,
    Fut: std::future::Future<Output = CheckResult> + Send,
{
    /// Create a new function-based health check.
    pub fn new(name: impl Into<String>, check_fn: F) -> Self {
        Self {
            name: name.into(),
            check_fn,
            critical: true,
        }
    }

    /// Mark this check as non-critical.
    pub fn non_critical(mut self) -> Self {
        self.critical = false;
        self
    }
}

#[async_trait]
impl<F, Fut> HealthCheck for FunctionCheck<F, Fut>
where
    F: Fn() -> Fut + Send + Sync,
    Fut: std::future::Future<Output = CheckResult> + Send,
{
    fn name(&self) -> &str {
        &self.name
    }

    async fn check(&self) -> CheckResult {
        let start = Instant::now();
        let mut result = (self.check_fn)().await;
        result.name = self.name.clone();
        result.duration_ms = Some(start.elapsed().as_millis() as u64);
        result
    }

    fn is_critical(&self) -> bool {
        self.critical
    }
}

/// Health check that verifies a TCP connection can be established.
pub struct TcpConnectionCheck {
    name: String,
    host: String,
    port: u16,
    timeout: Duration,
    critical: bool,
}

impl TcpConnectionCheck {
    /// Create a new TCP connection health check.
    pub fn new(name: impl Into<String>, host: impl Into<String>, port: u16) -> Self {
        Self {
            name: name.into(),
            host: host.into(),
            port,
            timeout: Duration::from_secs(5),
            critical: true,
        }
    }

    /// Set the connection timeout.
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Mark this check as non-critical.
    pub fn non_critical(mut self) -> Self {
        self.critical = false;
        self
    }
}

#[async_trait]
impl HealthCheck for TcpConnectionCheck {
    fn name(&self) -> &str {
        &self.name
    }

    async fn check(&self) -> CheckResult {
        let start = Instant::now();
        let addr = format!("{}:{}", self.host, self.port);

        match tokio::time::timeout(self.timeout, tokio::net::TcpStream::connect(&addr)).await {
            Ok(Ok(_)) => CheckResult::healthy(&self.name)
                .with_message(format!("Connected to {}", addr))
                .with_duration(start.elapsed().as_millis() as u64),
            Ok(Err(e)) => CheckResult::unhealthy(&self.name, format!("Connection failed: {}", e))
                .with_duration(start.elapsed().as_millis() as u64),
            Err(_) => CheckResult::unhealthy(
                &self.name,
                format!("Connection timeout after {}s", self.timeout.as_secs()),
            )
            .with_duration(start.elapsed().as_millis() as u64),
        }
    }

    fn is_critical(&self) -> bool {
        self.critical
    }
}

#[async_trait]
impl ReadinessCheck for TcpConnectionCheck {}

/// Health check that verifies a URL is accessible.
pub struct PingCheck {
    name: String,
    check_fn: Arc<dyn Fn() -> std::pin::Pin<Box<dyn std::future::Future<Output = bool> + Send>> + Send + Sync>,
    critical: bool,
}

impl PingCheck {
    /// Create a new ping-style health check.
    pub fn new<F, Fut>(name: impl Into<String>, check_fn: F) -> Self
    where
        F: Fn() -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = bool> + Send + 'static,
    {
        Self {
            name: name.into(),
            check_fn: Arc::new(move || Box::pin(check_fn())),
            critical: true,
        }
    }

    /// Mark this check as non-critical.
    pub fn non_critical(mut self) -> Self {
        self.critical = false;
        self
    }
}

#[async_trait]
impl HealthCheck for PingCheck {
    fn name(&self) -> &str {
        &self.name
    }

    async fn check(&self) -> CheckResult {
        let start = Instant::now();
        let is_healthy = (self.check_fn)().await;
        let duration = start.elapsed().as_millis() as u64;

        if is_healthy {
            CheckResult::healthy(&self.name).with_duration(duration)
        } else {
            CheckResult::unhealthy(&self.name, "Ping failed").with_duration(duration)
        }
    }

    fn is_critical(&self) -> bool {
        self.critical
    }
}

#[async_trait]
impl ReadinessCheck for PingCheck {}

/// Composite health check that combines multiple checks.
pub struct CompositeCheck {
    name: String,
    checks: Vec<Box<dyn HealthCheck>>,
    critical: bool,
}

impl CompositeCheck {
    /// Create a new composite health check.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            checks: Vec::new(),
            critical: true,
        }
    }

    /// Add a child health check.
    pub fn add_check(mut self, check: Box<dyn HealthCheck>) -> Self {
        self.checks.push(check);
        self
    }

    /// Mark this check as non-critical.
    pub fn non_critical(mut self) -> Self {
        self.critical = false;
        self
    }
}

#[async_trait]
impl HealthCheck for CompositeCheck {
    fn name(&self) -> &str {
        &self.name
    }

    async fn check(&self) -> CheckResult {
        let start = Instant::now();
        let mut overall_status = crate::traits::HealthStatus::Healthy;
        let mut messages = Vec::new();

        for check in &self.checks {
            let result = check.check().await;
            overall_status = overall_status.combine(&result.status);

            if let Some(msg) = result.message {
                messages.push(format!("{}: {}", result.name, msg));
            }
        }

        let message = if messages.is_empty() {
            None
        } else {
            Some(messages.join("; "))
        };

        let mut result = match overall_status {
            crate::traits::HealthStatus::Healthy => CheckResult::healthy(&self.name),
            crate::traits::HealthStatus::Degraded => {
                CheckResult::degraded(&self.name, message.as_deref().unwrap_or_default())
            }
            crate::traits::HealthStatus::Unhealthy => {
                CheckResult::unhealthy(&self.name, message.as_deref().unwrap_or_default())
            }
        };

        if let Some(msg) = message {
            result = result.with_message(msg);
        }

        result.with_duration(start.elapsed().as_millis() as u64)
    }

    fn is_critical(&self) -> bool {
        self.critical
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_always_healthy_check() {
        let check = AlwaysHealthyCheck::new("test");
        let result = check.check().await;
        assert_eq!(result.status, crate::traits::HealthStatus::Healthy);
        assert_eq!(result.name, "test");
    }

    #[tokio::test]
    async fn test_function_check() {
        let check = FunctionCheck::new("test", || async {
            CheckResult::healthy("test")
        });

        let result = check.check().await;
        assert_eq!(result.status, crate::traits::HealthStatus::Healthy);
        assert!(result.duration_ms.is_some());
    }

    #[tokio::test]
    async fn test_function_check_non_critical() {
        let check = FunctionCheck::new("test", || async {
            CheckResult::healthy("test")
        })
        .non_critical();

        assert!(!check.is_critical());
    }

    #[tokio::test]
    async fn test_tcp_connection_check_invalid() {
        let check = TcpConnectionCheck::new("test", "localhost", 9999)
            .with_timeout(Duration::from_millis(100));

        let result = check.check().await;
        assert_eq!(result.status, crate::traits::HealthStatus::Unhealthy);
    }

    #[tokio::test]
    async fn test_ping_check() {
        let check = PingCheck::new("test", || async { true });
        let result = check.check().await;
        assert_eq!(result.status, crate::traits::HealthStatus::Healthy);

        let check = PingCheck::new("test", || async { false });
        let result = check.check().await;
        assert_eq!(result.status, crate::traits::HealthStatus::Unhealthy);
    }

    #[tokio::test]
    async fn test_composite_check() {
        let check = CompositeCheck::new("composite")
            .add_check(Box::new(AlwaysHealthyCheck::new("check1")))
            .add_check(Box::new(AlwaysHealthyCheck::new("check2")));

        let result = check.check().await;
        assert_eq!(result.status, crate::traits::HealthStatus::Healthy);
    }
}
