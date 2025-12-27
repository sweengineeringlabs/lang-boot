# Rustboot Health

Health check infrastructure with liveness and readiness probes for the Rustboot framework.

## Overview

The `rustboot-health` crate provides comprehensive health check abstractions for monitoring application health. It supports both liveness probes (is the application running?) and readiness probes (is the application ready to serve traffic?), along with custom health checks for dependencies like databases, caches, and external services.

## Features

- **Liveness Probes**: Determine if the application is alive and should continue running
- **Readiness Probes**: Determine if the application is ready to serve traffic
- **Custom Health Checks**: Implement custom checks for any dependency
- **Health Aggregation**: Combine multiple checks into a single health status
- **JSON Responses**: Standard JSON format for health check responses
- **Built-in Checks**: Common checks for TCP connections, databases, caches, etc.
- **Critical vs Non-Critical**: Distinguish between critical checks that affect overall health and non-critical checks
- **Parallel Execution**: Execute health checks in parallel for better performance
- **Rich Metadata**: Include custom metadata in health check responses

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
dev-engineeringlabs-rustboot-health = "0.1.0"
```

## Quick Start

```rust
use dev_engineeringlabs_rustboot_health::{
    HealthAggregator, AlwaysHealthyCheck, FunctionCheck, CheckResult,
};

#[tokio::main]
async fn main() {
    // Create a health aggregator
    let health = HealthAggregator::new()
        .add_check(Box::new(AlwaysHealthyCheck::new("liveness")))
        .add_check(Box::new(FunctionCheck::new("database", || async {
            // Check database connection
            CheckResult::healthy("database")
        })))
        .with_version("1.0.0");

    // Execute health checks
    let report = health.check().await;
    println!("Health status: {}", report.status);
    println!("JSON: {}", report.to_json().unwrap());
}
```

## Core Concepts

### Health Status

Health checks can return three statuses:

- `Healthy`: Component is functioning normally
- `Degraded`: Component is functional but not optimal
- `Unhealthy`: Component is not functioning

### Liveness vs Readiness

- **Liveness**: Checks if the application is alive. If liveness fails, the application should be restarted.
- **Readiness**: Checks if the application can serve traffic. If readiness fails, traffic should not be routed to the application.

### Critical vs Non-Critical Checks

- **Critical**: Failures affect the overall health status
- **Non-Critical**: Failures are recorded but don't affect overall health

## Usage Examples

### Basic Liveness Check

```rust
use dev_engineeringlabs_rustboot_health::{AlwaysHealthyCheck, HealthCheck};

let check = AlwaysHealthyCheck::new("liveness");
let result = check.check().await;
assert_eq!(result.status, HealthStatus::Healthy);
```

### Custom Function Check

```rust
use dev_engineeringlabs_rustboot_health::{FunctionCheck, CheckResult};

let check = FunctionCheck::new("database", || async {
    // Perform actual database check
    match database.ping().await {
        Ok(_) => CheckResult::healthy("database")
            .with_message("Connection successful"),
        Err(e) => CheckResult::unhealthy("database", e.to_string()),
    }
});

let result = check.check().await;
```

### TCP Connection Check

```rust
use dev_engineeringlabs_rustboot_health::TcpConnectionCheck;
use std::time::Duration;

let check = TcpConnectionCheck::new("postgres", "localhost", 5432)
    .with_timeout(Duration::from_secs(5));

let result = check.check().await;
```

### Health Aggregation

```rust
use dev_engineeringlabs_rustboot_health::HealthAggregator;

let health = HealthAggregator::new()
    .add_check(Box::new(liveness_check))
    .add_check(Box::new(database_check))
    .add_check(Box::new(cache_check))
    .with_version("1.0.0");

// Sequential execution
let report = health.check().await;

// Parallel execution (faster)
let report = health.check_parallel().await;
```

### Non-Critical Checks

```rust
let health = HealthAggregator::new()
    .add_check(Box::new(critical_service_check))
    .add_check(Box::new(
        optional_feature_check.non_critical()
    ));

let report = health.check().await;
// Overall status is healthy even if optional feature fails
```

### Health Check with Metadata

```rust
let check = FunctionCheck::new("database", || async {
    CheckResult::healthy("database")
        .with_metadata("pool_size", serde_json::json!(20))
        .with_metadata("active_connections", serde_json::json!(5))
        .with_metadata("idle_connections", serde_json::json!(15))
});
```

### Composite Checks

```rust
use dev_engineeringlabs_rustboot_health::CompositeCheck;

let database_system = CompositeCheck::new("database_system")
    .add_check(Box::new(primary_db_check))
    .add_check(Box::new(replica_db_check));

let health = HealthAggregator::new()
    .add_check(Box::new(database_system));
```

## HTTP Integration

### Kubernetes Health Probes

```rust
// Liveness probe - GET /healthz
async fn liveness() -> Response {
    let health = HealthAggregator::new()
        .add_check(Box::new(AlwaysHealthyCheck::new("liveness")));

    let report = health.check().await;
    let status = if report.status.is_healthy() { 200 } else { 503 };

    (status, report.to_json().unwrap())
}

// Readiness probe - GET /readyz
async fn readiness() -> Response {
    let health = HealthAggregator::new()
        .add_check(Box::new(database_check))
        .add_check(Box::new(cache_check));

    let report = health.check().await;
    let status = if report.status.is_healthy() { 200 } else { 503 };

    (status, report.to_json().unwrap())
}
```

### Example Kubernetes Configuration

```yaml
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
```

## JSON Response Format

Health check responses follow a standard JSON format:

```json
{
  "status": "healthy",
  "checks": {
    "database": {
      "name": "database",
      "status": "healthy",
      "message": "Connection pool: 5/20",
      "metadata": {
        "pool_size": 20,
        "active": 5
      },
      "timestamp": "2024-12-24T10:30:00Z",
      "duration_ms": 45
    },
    "cache": {
      "name": "cache",
      "status": "healthy",
      "timestamp": "2024-12-24T10:30:00Z",
      "duration_ms": 12
    }
  },
  "timestamp": "2024-12-24T10:30:00Z",
  "duration_ms": 58,
  "version": "1.0.0"
}
```

## Best Practices

1. **Keep Liveness Simple**: Liveness checks should be fast and simple. They just verify the app is running.

2. **Make Readiness Comprehensive**: Readiness checks should verify all dependencies are available before serving traffic.

3. **Use Non-Critical for Optional Features**: Mark optional external dependencies as non-critical to avoid false negatives.

4. **Add Metadata**: Include useful debugging information in metadata (connection counts, response times, etc.).

5. **Set Appropriate Timeouts**: Configure reasonable timeouts to avoid health checks blocking too long.

6. **Use Parallel Execution**: For multiple checks, use `check_parallel()` to reduce total check time.

7. **Version Your APIs**: Include version information in health reports for debugging.

## Performance

The health check system is designed to be performant:

- Parallel execution of independent checks
- Configurable timeouts
- Minimal overhead for simple checks
- Efficient aggregation

Example performance with 3 slow checks (200ms each):
- Sequential: ~600ms
- Parallel: ~200ms (3x faster)

## Examples

See the `examples/` directory for complete examples:

- `health_basic.rs` - Basic usage
- `health_advanced.rs` - Advanced features
- `health_http_integration.rs` - HTTP endpoint integration

## Testing

Run the tests:

```bash
cargo test
```

Run a specific example:

```bash
cargo run --example health_basic
cargo run --example health_advanced
cargo run --example health_http_integration
```

## License

MIT
