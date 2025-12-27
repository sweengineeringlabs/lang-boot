//! Advanced health check example.
//!
//! Demonstrates advanced features including:
//! - Custom health checks
//! - Non-critical checks
//! - Degraded states
//! - TCP connection checks
//! - Composite checks

use dev_engineeringlabs_rustboot_health::{
    CheckResult, CompositeCheck, FunctionCheck, HealthAggregator, HealthCheck, PingCheck,
    TcpConnectionCheck,
};
use std::time::Duration;

#[tokio::main]
async fn main() {
    println!("=== Advanced Health Check Example ===\n");

    // Example 1: Mix of critical and non-critical checks
    println!("1. Critical vs Non-Critical Checks:");
    let health = HealthAggregator::new()
        .add_check(Box::new(FunctionCheck::new("critical_service", || async {
            CheckResult::healthy("critical_service")
        })))
        .add_check(Box::new(
            FunctionCheck::new("optional_feature", || async {
                CheckResult::unhealthy("optional_feature", "Feature disabled")
            })
            .non_critical(),
        ))
        .with_version("1.0.0");

    let report = health.check().await;
    println!("Overall Status: {} (should be healthy)", report.status);
    println!("Critical service is up, optional feature is down but doesn't affect overall health\n");

    // Example 2: Degraded state
    println!("2. Degraded State Example:");
    let health = HealthAggregator::new()
        .add_check(Box::new(FunctionCheck::new("api", || async {
            CheckResult::degraded("api", "High response time: 2.5s")
                .with_metadata("avg_response_ms", serde_json::json!(2500))
                .with_metadata("threshold_ms", serde_json::json!(1000))
        })))
        .add_check(Box::new(FunctionCheck::new("database", || async {
            CheckResult::healthy("database")
        })));

    let report = health.check().await;
    println!("{}\n", report.to_json().unwrap());

    // Example 3: TCP connection checks
    println!("3. TCP Connection Checks:");
    let health = HealthAggregator::new()
        .add_check(Box::new(
            TcpConnectionCheck::new("postgres", "localhost", 5432)
                .with_timeout(Duration::from_secs(2))
                .non_critical(), // Non-critical because it might not be running in this example
        ))
        .add_check(Box::new(
            TcpConnectionCheck::new("redis", "localhost", 6379)
                .with_timeout(Duration::from_secs(2))
                .non_critical(),
        ));

    let report = health.check().await;
    for (name, result) in &report.checks {
        println!(
            "  {}: {} - {}",
            name,
            result.status,
            result.message.as_deref().unwrap_or("OK")
        );
    }
    println!();

    // Example 4: Composite checks
    println!("4. Composite Checks:");
    let database_checks = CompositeCheck::new("database_system")
        .add_check(Box::new(FunctionCheck::new("primary_db", || async {
            CheckResult::healthy("primary_db")
                .with_message("Primary database operational")
        })))
        .add_check(Box::new(FunctionCheck::new("replica_db", || async {
            CheckResult::healthy("replica_db")
                .with_message("Replica in sync")
        })));

    let cache_checks = CompositeCheck::new("cache_system")
        .add_check(Box::new(FunctionCheck::new("redis_master", || async {
            CheckResult::healthy("redis_master")
        })))
        .add_check(Box::new(FunctionCheck::new("redis_slave", || async {
            CheckResult::degraded("redis_slave", "Replication lag: 500ms")
        })));

    let health = HealthAggregator::new()
        .add_check(Box::new(database_checks))
        .add_check(Box::new(cache_checks));

    let report = health.check().await;
    println!("{}\n", report.to_json().unwrap());

    // Example 5: Ping-style checks
    println!("5. Ping-style Checks:");
    let health = HealthAggregator::new()
        .add_check(Box::new(PingCheck::new("external_api", || async {
            // Simulate external API ping
            tokio::time::sleep(Duration::from_millis(50)).await;
            true
        })))
        .add_check(Box::new(PingCheck::new("message_queue", || async {
            // Simulate message queue check
            true
        })));

    let report = health.check().await;
    println!("Ping checks completed in: {}ms", report.duration_ms.unwrap_or(0));
    println!("Status: {}\n", report.status);

    // Example 6: Parallel execution
    println!("6. Sequential vs Parallel Execution:");
    let slow_check = || async {
        tokio::time::sleep(Duration::from_millis(200)).await;
        CheckResult::healthy("slow")
    };

    let health = HealthAggregator::new()
        .add_check(Box::new(FunctionCheck::new("service1", slow_check)))
        .add_check(Box::new(FunctionCheck::new("service2", slow_check)))
        .add_check(Box::new(FunctionCheck::new("service3", slow_check)));

    let start = std::time::Instant::now();
    let _ = health.check().await;
    let sequential_time = start.elapsed();

    let start = std::time::Instant::now();
    let _ = health.check_parallel().await;
    let parallel_time = start.elapsed();

    println!("Sequential execution: {}ms", sequential_time.as_millis());
    println!("Parallel execution: {}ms", parallel_time.as_millis());
    println!(
        "Speedup: {:.2}x\n",
        sequential_time.as_millis() as f64 / parallel_time.as_millis() as f64
    );

    // Example 7: Realistic application health check
    println!("7. Realistic Application Example:");
    let app_health = HealthAggregator::new()
        // Liveness: Just checks if the app is running
        .add_check(Box::new(FunctionCheck::new("liveness", || async {
            CheckResult::healthy("liveness")
        })))
        // Readiness: Checks if app can serve traffic
        .add_check(Box::new(FunctionCheck::new("database", || async {
            // In real app, this would check actual database connection
            CheckResult::healthy("database")
                .with_metadata("connection_count", serde_json::json!(5))
        })))
        .add_check(Box::new(FunctionCheck::new("cache", || async {
            CheckResult::healthy("cache")
                .with_metadata("hit_rate", serde_json::json!(0.92))
        })))
        // Non-critical external dependencies
        .add_check(Box::new(
            FunctionCheck::new("analytics_service", || async {
                CheckResult::unhealthy("analytics_service", "Service unavailable")
            })
            .non_critical(),
        ))
        .with_version("2.1.0");

    let report = app_health.check().await;
    println!("Application Health Report:");
    println!("{}", report.to_json().unwrap());
}
