# Health Check Integration Guide

This guide shows how to integrate rustboot-health with popular web frameworks and deployment platforms.

## Table of Contents

1. [Axum Integration](#axum-integration)
2. [Actix-Web Integration](#actix-web-integration)
3. [Kubernetes Configuration](#kubernetes-configuration)
4. [Docker Health Checks](#docker-health-checks)
5. [AWS ELB/ALB Configuration](#aws-elbalb-configuration)
6. [Custom Database Checks](#custom-database-checks)
7. [Custom Cache Checks](#custom-cache-checks)

## Axum Integration

### Basic Setup

```rust
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::get,
    Router,
};
use dev_engineeringlabs_rustboot_health::{
    HealthAggregator, AlwaysHealthyCheck, FunctionCheck, HealthStatus,
};
use std::sync::Arc;

// Shared health aggregator
type SharedHealth = Arc<HealthAggregator>;

#[tokio::main]
async fn main() {
    // Create health aggregator once
    let health = Arc::new(
        HealthAggregator::new()
            .add_check(Box::new(AlwaysHealthyCheck::new("liveness")))
            .add_check(Box::new(FunctionCheck::new("database", || async {
                // Your database check here
                CheckResult::healthy("database")
            })))
            .with_version("1.0.0")
    );

    // Build router
    let app = Router::new()
        .route("/healthz", get(liveness_check))
        .route("/readyz", get(readiness_check))
        .route("/health", get(health_check))
        .with_state(health);

    // Run server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

// Liveness endpoint
async fn liveness_check(State(health): State<SharedHealth>) -> impl IntoResponse {
    let liveness = HealthAggregator::new()
        .add_check(Box::new(AlwaysHealthyCheck::new("liveness")));

    let report = liveness.check().await;

    let status = if report.status == HealthStatus::Healthy {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    };

    (status, Json(report))
}

// Readiness endpoint
async fn readiness_check(State(health): State<SharedHealth>) -> impl IntoResponse {
    let report = health.check().await;

    let status = if report.status == HealthStatus::Healthy {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    };

    (status, Json(report))
}

// Full health endpoint
async fn health_check(State(health): State<SharedHealth>) -> impl IntoResponse {
    let report = health.check_parallel().await;

    let status = match report.status {
        HealthStatus::Healthy => StatusCode::OK,
        HealthStatus::Degraded => StatusCode::OK, // Still serving traffic
        HealthStatus::Unhealthy => StatusCode::SERVICE_UNAVAILABLE,
    };

    (status, Json(report))
}
```

## Actix-Web Integration

```rust
use actix_web::{get, web, App, HttpResponse, HttpServer};
use dev_engineeringlabs_rustboot_health::{
    HealthAggregator, AlwaysHealthyCheck, FunctionCheck, HealthStatus,
};
use std::sync::Arc;

struct AppState {
    health: Arc<HealthAggregator>,
}

#[get("/healthz")]
async fn liveness_check() -> HttpResponse {
    let health = HealthAggregator::new()
        .add_check(Box::new(AlwaysHealthyCheck::new("liveness")));

    let report = health.check().await;

    if report.status == HealthStatus::Healthy {
        HttpResponse::Ok().json(report)
    } else {
        HttpResponse::ServiceUnavailable().json(report)
    }
}

#[get("/readyz")]
async fn readiness_check(data: web::Data<AppState>) -> HttpResponse {
    let report = data.health.check().await;

    if report.status == HealthStatus::Healthy {
        HttpResponse::Ok().json(report)
    } else {
        HttpResponse::ServiceUnavailable().json(report)
    }
}

#[get("/health")]
async fn health_check(data: web::Data<AppState>) -> HttpResponse {
    let report = data.health.check_parallel().await;

    match report.status {
        HealthStatus::Healthy | HealthStatus::Degraded => {
            HttpResponse::Ok().json(report)
        }
        HealthStatus::Unhealthy => {
            HttpResponse::ServiceUnavailable().json(report)
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let health = Arc::new(
        HealthAggregator::new()
            .add_check(Box::new(AlwaysHealthyCheck::new("liveness")))
            .with_version("1.0.0")
    );

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                health: health.clone(),
            }))
            .service(liveness_check)
            .service(readiness_check)
            .service(health_check)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
```

## Kubernetes Configuration

### Deployment with Health Checks

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: my-app
  labels:
    app: my-app
spec:
  replicas: 3
  selector:
    matchLabels:
      app: my-app
  template:
    metadata:
      labels:
        app: my-app
    spec:
      containers:
      - name: app
        image: my-app:1.0.0
        ports:
        - containerPort: 8080
          name: http

        # Liveness probe - is the app alive?
        livenessProbe:
          httpGet:
            path: /healthz
            port: http
          initialDelaySeconds: 10
          periodSeconds: 10
          timeoutSeconds: 5
          failureThreshold: 3
          successThreshold: 1

        # Readiness probe - is the app ready for traffic?
        readinessProbe:
          httpGet:
            path: /readyz
            port: http
          initialDelaySeconds: 5
          periodSeconds: 5
          timeoutSeconds: 3
          failureThreshold: 2
          successThreshold: 1

        # Startup probe - did the app start successfully?
        startupProbe:
          httpGet:
            path: /healthz
            port: http
          initialDelaySeconds: 0
          periodSeconds: 2
          timeoutSeconds: 3
          failureThreshold: 30
          successThreshold: 1

        resources:
          requests:
            memory: "256Mi"
            cpu: "250m"
          limits:
            memory: "512Mi"
            cpu: "500m"
```

### Service

```yaml
apiVersion: v1
kind: Service
metadata:
  name: my-app
spec:
  selector:
    app: my-app
  ports:
  - name: http
    port: 80
    targetPort: 8080
  type: LoadBalancer
```

## Docker Health Checks

### Dockerfile with HEALTHCHECK

```dockerfile
FROM rust:1.75 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y curl && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/my-app /usr/local/bin/my-app

EXPOSE 8080

# Docker health check
HEALTHCHECK --interval=30s --timeout=5s --start-period=10s --retries=3 \
  CMD curl -f http://localhost:8080/healthz || exit 1

CMD ["my-app"]
```

### Docker Compose

```yaml
version: '3.8'
services:
  app:
    build: .
    ports:
      - "8080:8080"
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8080/healthz"]
      interval: 30s
      timeout: 5s
      retries: 3
      start_period: 10s
    depends_on:
      postgres:
        condition: service_healthy
      redis:
        condition: service_healthy

  postgres:
    image: postgres:15
    environment:
      POSTGRES_PASSWORD: password
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U postgres"]
      interval: 10s
      timeout: 5s
      retries: 5

  redis:
    image: redis:7
    healthcheck:
      test: ["CMD", "redis-cli", "ping"]
      interval: 10s
      timeout: 3s
      retries: 5
```

## AWS ELB/ALB Configuration

### Application Load Balancer (ALB)

```hcl
# Terraform configuration
resource "aws_lb_target_group" "app" {
  name     = "my-app-tg"
  port     = 8080
  protocol = "HTTP"
  vpc_id   = aws_vpc.main.id

  health_check {
    enabled             = true
    path                = "/health"
    port                = "traffic-port"
    protocol            = "HTTP"
    healthy_threshold   = 2
    unhealthy_threshold = 3
    timeout             = 5
    interval            = 30
    matcher             = "200"
  }

  deregistration_delay = 30
}
```

### Network Load Balancer (NLB)

```hcl
resource "aws_lb_target_group" "app" {
  name     = "my-app-tg"
  port     = 8080
  protocol = "TCP"
  vpc_id   = aws_vpc.main.id

  health_check {
    enabled             = true
    port                = 8080
    protocol            = "HTTP"
    path                = "/healthz"
    healthy_threshold   = 2
    unhealthy_threshold = 2
    timeout             = 5
    interval            = 10
  }
}
```

## Custom Database Checks

### PostgreSQL with sqlx

```rust
use dev_engineeringlabs_rustboot_health::{FunctionCheck, CheckResult};
use sqlx::PgPool;
use std::sync::Arc;

fn create_postgres_check(pool: Arc<PgPool>) -> FunctionCheck<impl Fn() -> _, _> {
    FunctionCheck::new("postgres", move || {
        let pool = pool.clone();
        async move {
            match sqlx::query("SELECT 1")
                .fetch_one(pool.as_ref())
                .await
            {
                Ok(_) => {
                    let pool_size = pool.size();
                    let idle = pool.num_idle();

                    CheckResult::healthy("postgres")
                        .with_message(format!("Pool: {}/{} active", pool_size - idle, pool_size))
                        .with_metadata("pool_size", serde_json::json!(pool_size))
                        .with_metadata("active", serde_json::json!(pool_size - idle))
                        .with_metadata("idle", serde_json::json!(idle))
                }
                Err(e) => CheckResult::unhealthy("postgres", e.to_string()),
            }
        }
    })
}
```

### MongoDB

```rust
use dev_engineeringlabs_rustboot_health::{FunctionCheck, CheckResult};
use mongodb::Client;
use std::sync::Arc;

fn create_mongodb_check(client: Arc<Client>) -> FunctionCheck<impl Fn() -> _, _> {
    FunctionCheck::new("mongodb", move || {
        let client = client.clone();
        async move {
            match client
                .database("admin")
                .run_command(mongodb::bson::doc! { "ping": 1 }, None)
                .await
            {
                Ok(_) => CheckResult::healthy("mongodb")
                    .with_message("Connection successful"),
                Err(e) => CheckResult::unhealthy("mongodb", e.to_string()),
            }
        }
    })
}
```

## Custom Cache Checks

### Redis

```rust
use dev_engineeringlabs_rustboot_health::{FunctionCheck, CheckResult};
use redis::aio::MultiplexedConnection;
use redis::AsyncCommands;
use std::sync::Arc;
use tokio::sync::Mutex;

fn create_redis_check(
    conn: Arc<Mutex<MultiplexedConnection>>
) -> FunctionCheck<impl Fn() -> _, _> {
    FunctionCheck::new("redis", move || {
        let conn = conn.clone();
        async move {
            match conn.lock().await.ping::<String>().await {
                Ok(_) => CheckResult::healthy("redis")
                    .with_message("PING successful"),
                Err(e) => CheckResult::unhealthy("redis", e.to_string()),
            }
        }
    })
}
```

### Memcached

```rust
use dev_engineeringlabs_rustboot_health::{FunctionCheck, CheckResult};
use std::sync::Arc;

fn create_memcached_check(
    client: Arc<memcache::Client>
) -> FunctionCheck<impl Fn() -> _, _> {
    FunctionCheck::new("memcached", move || {
        let client = client.clone();
        async move {
            match client.version() {
                Ok(versions) => {
                    let version_str = versions
                        .iter()
                        .map(|(addr, ver)| format!("{}:{}", addr, ver))
                        .collect::<Vec<_>>()
                        .join(", ");

                    CheckResult::healthy("memcached")
                        .with_message(format!("Versions: {}", version_str))
                }
                Err(e) => CheckResult::unhealthy("memcached", e.to_string()),
            }
        }
    })
}
```

## Complete Application Example

```rust
use axum::{Router, routing::get};
use dev_engineeringlabs_rustboot_health::{
    HealthAggregator, AlwaysHealthyCheck, FunctionCheck, TcpConnectionCheck,
};
use std::sync::Arc;
use std::time::Duration;

#[tokio::main]
async fn main() {
    // Initialize your dependencies
    let db_pool = init_database().await;
    let redis_pool = init_redis().await;

    // Create health aggregator with all checks
    let health = Arc::new(
        HealthAggregator::new()
            // Liveness - just checks app is running
            .add_check(Box::new(AlwaysHealthyCheck::new("liveness")))

            // Critical checks - affect readiness
            .add_check(Box::new(create_postgres_check(db_pool.clone())))
            .add_check(Box::new(create_redis_check(redis_pool.clone())))

            // Non-critical external dependencies
            .add_check(Box::new(
                TcpConnectionCheck::new("external_api", "api.example.com", 443)
                    .with_timeout(Duration::from_secs(2))
                    .non_critical()
            ))

            .with_version(env!("CARGO_PKG_VERSION"))
    );

    // Build application
    let app = Router::new()
        .route("/healthz", get(liveness_check))
        .route("/readyz", get(readiness_check))
        .route("/health", get(health_check))
        .with_state(health);

    // Start server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .unwrap();

    println!("Server running on http://0.0.0.0:8080");
    axum::serve(listener, app).await.unwrap();
}
```

## Best Practices

1. **Keep liveness simple** - Just check if the app is running
2. **Make readiness comprehensive** - Check all critical dependencies
3. **Use non-critical for optional features** - External APIs, analytics, etc.
4. **Add metadata** - Include connection counts, response times, etc.
5. **Set appropriate timeouts** - Don't let health checks hang
6. **Use parallel execution** - Faster health checks with `check_parallel()`
7. **Cache expensive checks** - Store results for expensive operations
8. **Version your health checks** - Include app version in responses
9. **Monitor health check duration** - Alert if checks take too long
10. **Test your health checks** - Ensure they work in all scenarios

## Troubleshooting

### Health checks timing out
- Reduce timeout values
- Use parallel execution
- Cache expensive checks
- Add logging to identify slow checks

### False negatives
- Adjust threshold values
- Check for network issues
- Verify credentials and permissions
- Add retry logic for flaky dependencies

### False positives
- Make checks more thorough
- Add actual dependency verification
- Don't just check TCP connection
- Verify data can be read/written

## Resources

- [Kubernetes Liveness and Readiness Probes](https://kubernetes.io/docs/tasks/configure-pod-container/configure-liveness-readiness-startup-probes/)
- [Docker HEALTHCHECK](https://docs.docker.com/engine/reference/builder/#healthcheck)
- [AWS Target Group Health Checks](https://docs.aws.amazon.com/elasticloadbalancing/latest/application/target-group-health-checks.html)
