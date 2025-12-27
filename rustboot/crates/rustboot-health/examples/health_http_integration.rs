//! HTTP integration example for health checks.
//!
//! Demonstrates how to expose health check endpoints in a web application.
//! This example shows the pattern, but doesn't require an actual HTTP server.

use dev_engineeringlabs_rustboot_health::{
    AlwaysHealthyCheck, CheckResult, FunctionCheck, HealthAggregator, HealthStatus,
};

/// Simulated HTTP handler for liveness endpoint.
async fn liveness_handler() -> (u16, String) {
    let health = HealthAggregator::new()
        .add_check(Box::new(AlwaysHealthyCheck::new("liveness")));

    let report = health.check().await;

    let status_code = if report.status == HealthStatus::Healthy {
        200
    } else {
        503
    };

    (status_code, report.to_json().unwrap())
}

/// Simulated HTTP handler for readiness endpoint.
async fn readiness_handler() -> (u16, String) {
    let health = HealthAggregator::new()
        .add_check(Box::new(FunctionCheck::new("database", || async {
            // Check if database is ready
            CheckResult::healthy("database")
        })))
        .add_check(Box::new(FunctionCheck::new("cache", || async {
            // Check if cache is ready
            CheckResult::healthy("cache")
        })))
        .add_check(Box::new(FunctionCheck::new("migrations", || async {
            // Check if database migrations are complete
            CheckResult::healthy("migrations")
                .with_message("All migrations applied")
        })));

    let report = health.check().await;

    let status_code = if report.status == HealthStatus::Healthy {
        200
    } else {
        503
    };

    (status_code, report.to_json().unwrap())
}

/// Simulated HTTP handler for full health endpoint.
async fn health_handler() -> (u16, String) {
    let health = HealthAggregator::new()
        .add_check(Box::new(AlwaysHealthyCheck::new("liveness")))
        .add_check(Box::new(FunctionCheck::new("database", || async {
            CheckResult::healthy("database")
                .with_metadata("pool_size", serde_json::json!(20))
                .with_metadata("idle", serde_json::json!(15))
                .with_metadata("active", serde_json::json!(5))
        })))
        .add_check(Box::new(FunctionCheck::new("cache", || async {
            CheckResult::healthy("cache")
                .with_metadata("memory_usage_mb", serde_json::json!(256))
                .with_metadata("hit_rate", serde_json::json!(0.95))
        })))
        .add_check(Box::new(FunctionCheck::new("api_latency", || async {
            CheckResult::degraded("api_latency", "P99 latency above threshold")
                .with_metadata("p50_ms", serde_json::json!(45))
                .with_metadata("p99_ms", serde_json::json!(1200))
                .with_metadata("threshold_ms", serde_json::json!(1000))
        })))
        .add_check(Box::new(
            FunctionCheck::new("external_api", || async {
                CheckResult::unhealthy("external_api", "Connection refused")
            })
            .non_critical(),
        ))
        .with_version("1.2.3");

    let report = health.check().await;

    // Return 200 for healthy, 503 for degraded/unhealthy
    let status_code = match report.status {
        HealthStatus::Healthy => 200,
        HealthStatus::Degraded => 200, // Still serving traffic
        HealthStatus::Unhealthy => 503,
    };

    (status_code, report.to_json().unwrap())
}

#[tokio::main]
async fn main() {
    println!("=== HTTP Health Check Integration Example ===\n");
    println!("This example demonstrates how to structure health check endpoints");
    println!("for Kubernetes or load balancer health probes.\n");

    // Simulate /healthz (liveness probe)
    println!("GET /healthz (Liveness Probe):");
    let (status, body) = liveness_handler().await;
    println!("Status: {}", status);
    println!("{}\n", body);

    // Simulate /readyz (readiness probe)
    println!("GET /readyz (Readiness Probe):");
    let (status, body) = readiness_handler().await;
    println!("Status: {}", status);
    println!("{}\n", body);

    // Simulate /health (full health check)
    println!("GET /health (Full Health Check):");
    let (status, body) = health_handler().await;
    println!("Status: {}", status);
    println!("{}\n", body);

    println!("=== Kubernetes Health Probe Configuration ===\n");
    println!(
        r#"
# Example Kubernetes deployment with health checks:
apiVersion: v1
kind: Pod
metadata:
  name: my-app
spec:
  containers:
  - name: app
    image: my-app:latest
    livenessProbe:
      httpGet:
        path: /healthz
        port: 8080
      initialDelaySeconds: 10
      periodSeconds: 10
      failureThreshold: 3
    readinessProbe:
      httpGet:
        path: /readyz
        port: 8080
      initialDelaySeconds: 5
      periodSeconds: 5
      failureThreshold: 2
"#
    );

    println!("=== Load Balancer Health Check Configuration ===\n");
    println!(
        r#"
# Example AWS ALB/ELB target group health check:
- Protocol: HTTP
- Path: /health
- Port: 8080
- Healthy threshold: 2
- Unhealthy threshold: 3
- Timeout: 5 seconds
- Interval: 30 seconds
- Success codes: 200
"#
    );
}
