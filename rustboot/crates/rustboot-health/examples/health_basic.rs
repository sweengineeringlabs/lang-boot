//! Basic health check example.
//!
//! Demonstrates simple health check setup with liveness and readiness probes.

use dev_engineeringlabs_rustboot_health::{
    AlwaysHealthyCheck, CheckResult, FunctionCheck, HealthAggregator, HealthStatus,
};

#[tokio::main]
async fn main() {
    println!("=== Basic Health Check Example ===\n");

    // Create a simple health aggregator with liveness check
    let health = HealthAggregator::new()
        .add_check(Box::new(AlwaysHealthyCheck::new("liveness")))
        .with_version("1.0.0");

    let report = health.check().await;
    println!("Liveness Check:");
    println!("{}\n", report.to_json().unwrap());

    // Create a health aggregator with multiple checks
    let health = HealthAggregator::new()
        .add_check(Box::new(AlwaysHealthyCheck::new("liveness")))
        .add_check(Box::new(FunctionCheck::new("database", || async {
            // Simulate database health check
            CheckResult::healthy("database")
                .with_message("Connection pool: 10/100")
                .with_metadata("pool_size", serde_json::json!(100))
                .with_metadata("active", serde_json::json!(10))
        })))
        .add_check(Box::new(FunctionCheck::new("cache", || async {
            // Simulate cache health check
            CheckResult::healthy("cache")
                .with_message("Memory usage: 45%")
        })))
        .with_version("1.0.0");

    println!("Full Health Check:");
    let report = health.check().await;
    println!("{}\n", report.to_json().unwrap());

    println!("Status: {}", report.status);
    println!("Number of checks: {}", report.checks.len());
    println!("Duration: {}ms", report.duration_ms.unwrap_or(0));
}
