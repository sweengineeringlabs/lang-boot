//! Integration tests for rustboot-health.

use dev_engineeringlabs_rustboot_health::{
    AlwaysHealthyCheck, CheckResult, CompositeCheck, FunctionCheck, HealthAggregator,
    HealthCheck, HealthStatus, LivenessCheck, PingCheck, ReadinessCheck, TcpConnectionCheck,
};
use std::time::Duration;

#[tokio::test]
async fn test_basic_liveness_check() {
    let check = AlwaysHealthyCheck::new("liveness");
    let result = check.check().await;

    assert_eq!(result.name, "liveness");
    assert_eq!(result.status, HealthStatus::Healthy);
    assert!(result.message.is_none());
}

#[tokio::test]
async fn test_liveness_trait() {
    let check = AlwaysHealthyCheck::new("liveness");
    let is_alive = check.is_alive().await.unwrap();
    assert!(is_alive);
}

#[tokio::test]
async fn test_custom_function_check() {
    let check = FunctionCheck::new("custom", || async {
        CheckResult::healthy("custom").with_message("All systems operational")
    });

    let result = check.check().await;
    assert_eq!(result.status, HealthStatus::Healthy);
    assert_eq!(
        result.message,
        Some("All systems operational".to_string())
    );
    assert!(result.duration_ms.is_some());
}

#[tokio::test]
async fn test_unhealthy_check() {
    let check = FunctionCheck::new("failing", || async {
        CheckResult::unhealthy("failing", "Connection lost")
    });

    let result = check.check().await;
    assert_eq!(result.status, HealthStatus::Unhealthy);
    assert_eq!(result.message, Some("Connection lost".to_string()));
}

#[tokio::test]
async fn test_degraded_check() {
    let check = FunctionCheck::new("degraded", || async {
        CheckResult::degraded("degraded", "High latency detected")
    });

    let result = check.check().await;
    assert_eq!(result.status, HealthStatus::Degraded);
    assert!(result.message.is_some());
}

#[tokio::test]
async fn test_tcp_connection_check_failure() {
    let check = TcpConnectionCheck::new("database", "localhost", 9999)
        .with_timeout(Duration::from_millis(100));

    let result = check.check().await;
    assert_eq!(result.status, HealthStatus::Unhealthy);
    assert!(result.message.is_some());
    assert!(result.duration_ms.is_some());
}

#[tokio::test]
async fn test_ping_check_success() {
    let check = PingCheck::new("service", || async { true });
    let result = check.check().await;
    assert_eq!(result.status, HealthStatus::Healthy);
}

#[tokio::test]
async fn test_ping_check_failure() {
    let check = PingCheck::new("service", || async { false });
    let result = check.check().await;
    assert_eq!(result.status, HealthStatus::Unhealthy);
}

#[tokio::test]
async fn test_readiness_trait() {
    let check = PingCheck::new("readiness", || async { true });
    let is_ready = check.is_ready().await.unwrap();
    assert!(is_ready);

    let check = PingCheck::new("readiness", || async { false });
    let is_ready = check.is_ready().await.unwrap();
    assert!(!is_ready);
}

#[tokio::test]
async fn test_composite_check_all_healthy() {
    let check = CompositeCheck::new("system")
        .add_check(Box::new(AlwaysHealthyCheck::new("service1")))
        .add_check(Box::new(AlwaysHealthyCheck::new("service2")))
        .add_check(Box::new(AlwaysHealthyCheck::new("service3")));

    let result = check.check().await;
    assert_eq!(result.status, HealthStatus::Healthy);
}

#[tokio::test]
async fn test_composite_check_with_failure() {
    let check = CompositeCheck::new("system")
        .add_check(Box::new(AlwaysHealthyCheck::new("service1")))
        .add_check(Box::new(FunctionCheck::new("service2", || async {
            CheckResult::unhealthy("service2", "Database unreachable")
        })));

    let result = check.check().await;
    assert_eq!(result.status, HealthStatus::Unhealthy);
    assert!(result.message.is_some());
}

#[tokio::test]
async fn test_health_aggregator_sequential() {
    let aggregator = HealthAggregator::new()
        .add_check(Box::new(AlwaysHealthyCheck::new("liveness")))
        .add_check(Box::new(FunctionCheck::new("database", || async {
            CheckResult::healthy("database")
        })))
        .add_check(Box::new(FunctionCheck::new("cache", || async {
            CheckResult::healthy("cache")
        })))
        .with_version("1.0.0");

    let report = aggregator.check().await;

    assert_eq!(report.status, HealthStatus::Healthy);
    assert_eq!(report.checks.len(), 3);
    assert_eq!(report.version, Some("1.0.0".to_string()));
    assert!(report.duration_ms.is_some());
    assert!(report.checks.contains_key("liveness"));
    assert!(report.checks.contains_key("database"));
    assert!(report.checks.contains_key("cache"));
}

#[tokio::test]
async fn test_health_aggregator_parallel() {
    let aggregator = HealthAggregator::new()
        .add_check(Box::new(AlwaysHealthyCheck::new("check1")))
        .add_check(Box::new(AlwaysHealthyCheck::new("check2")))
        .add_check(Box::new(AlwaysHealthyCheck::new("check3")));

    let report = aggregator.check_parallel().await;

    assert_eq!(report.status, HealthStatus::Healthy);
    assert_eq!(report.checks.len(), 3);
}

#[tokio::test]
async fn test_health_aggregator_with_failures() {
    let aggregator = HealthAggregator::new()
        .add_check(Box::new(AlwaysHealthyCheck::new("healthy")))
        .add_check(Box::new(FunctionCheck::new("unhealthy", || async {
            CheckResult::unhealthy("unhealthy", "Service down")
        })));

    let report = aggregator.check().await;

    assert_eq!(report.status, HealthStatus::Unhealthy);
    assert_eq!(report.checks.len(), 2);
}

#[tokio::test]
async fn test_non_critical_checks_dont_affect_status() {
    let aggregator = HealthAggregator::new()
        .add_check(Box::new(AlwaysHealthyCheck::new("critical")))
        .add_check(Box::new(
            FunctionCheck::new("non_critical", || async {
                CheckResult::unhealthy("non_critical", "Optional service down")
            })
            .non_critical(),
        ));

    let report = aggregator.check().await;

    // Overall status should be healthy because the failing check is non-critical
    assert_eq!(report.status, HealthStatus::Healthy);
    assert_eq!(report.checks.len(), 2);

    // But the individual check should still show as unhealthy
    let non_critical = report.checks.get("non_critical").unwrap();
    assert_eq!(non_critical.status, HealthStatus::Unhealthy);
}

#[tokio::test]
async fn test_health_report_json_serialization() {
    let aggregator = HealthAggregator::new()
        .add_check(Box::new(AlwaysHealthyCheck::new("test")))
        .with_version("1.0.0");

    let report = aggregator.check().await;
    let json = report.to_json().unwrap();

    assert!(json.contains("\"status\": \"healthy\"") || json.contains("\"status\":\"healthy\""));
    assert!(json.contains("\"version\": \"1.0.0\"") || json.contains("\"version\":\"1.0.0\""));
    assert!(json.contains("\"test\""));
}

#[tokio::test]
async fn test_health_report_json_compact() {
    let aggregator = HealthAggregator::new()
        .add_check(Box::new(AlwaysHealthyCheck::new("test")));

    let report = aggregator.check().await;
    let json_compact = report.to_json_compact().unwrap();

    // Compact JSON should not have extra whitespace
    assert!(!json_compact.contains('\n'));
}

#[tokio::test]
async fn test_check_result_with_metadata() {
    let check = FunctionCheck::new("database", || async {
        CheckResult::healthy("database")
            .with_metadata("connection_pool_size", serde_json::json!(10))
            .with_metadata("active_connections", serde_json::json!(3))
    });

    let result = check.check().await;
    assert!(result.metadata.is_some());

    let metadata = result.metadata.unwrap();
    assert_eq!(metadata.get("connection_pool_size").unwrap(), &serde_json::json!(10));
    assert_eq!(metadata.get("active_connections").unwrap(), &serde_json::json!(3));
}

#[tokio::test]
async fn test_multiple_degraded_checks() {
    let aggregator = HealthAggregator::new()
        .add_check(Box::new(FunctionCheck::new("service1", || async {
            CheckResult::degraded("service1", "High latency")
        })))
        .add_check(Box::new(FunctionCheck::new("service2", || async {
            CheckResult::degraded("service2", "Low memory")
        })));

    let report = aggregator.check().await;
    assert_eq!(report.status, HealthStatus::Degraded);
}

#[tokio::test]
async fn test_health_status_combination() {
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

#[tokio::test]
async fn test_aggregator_check_count() {
    let aggregator = HealthAggregator::new()
        .add_check(Box::new(AlwaysHealthyCheck::new("check1")))
        .add_check(Box::new(AlwaysHealthyCheck::new("check2")));

    assert_eq!(aggregator.check_count(), 2);
}

#[tokio::test]
async fn test_async_check_with_delay() {
    let check = FunctionCheck::new("slow_check", || async {
        tokio::time::sleep(Duration::from_millis(50)).await;
        CheckResult::healthy("slow_check")
    });

    let result = check.check().await;
    assert_eq!(result.status, HealthStatus::Healthy);
    assert!(result.duration_ms.unwrap() >= 50);
}

#[tokio::test]
async fn test_parallel_checks_faster_than_sequential() {
    let slow_check = || async {
        tokio::time::sleep(Duration::from_millis(100)).await;
        CheckResult::healthy("slow")
    };

    let aggregator = HealthAggregator::new()
        .add_check(Box::new(FunctionCheck::new("slow1", slow_check)))
        .add_check(Box::new(FunctionCheck::new("slow2", slow_check)))
        .add_check(Box::new(FunctionCheck::new("slow3", slow_check)));

    // Parallel execution should be faster
    let start = std::time::Instant::now();
    let report = aggregator.check_parallel().await;
    let parallel_duration = start.elapsed();

    assert_eq!(report.status, HealthStatus::Healthy);
    assert_eq!(report.checks.len(), 3);

    // Should complete in roughly 100ms (all running in parallel)
    // rather than 300ms (sequential)
    assert!(parallel_duration.as_millis() < 250);
}
