//! Rustboot End-to-End Example Application
//!
//! This example demonstrates a complete web application built with the Rustboot framework,
//! showcasing:
//!
//! - HTTP server with Axum integration
//! - Dependency injection container
//! - Input validation
//! - Resilience patterns (circuit breaker, retry)
//! - Rate limiting
//! - Caching
//! - Health checks
//! - State machine for order processing
//! - Structured logging and observability
//!
//! Run with: cargo run --package rustboot-e2e-example

use anyhow::Result;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use chrono::{DateTime, Utc};
use dev_engineeringlabs_rustboot_cache::{Cache, InMemoryCache};
use dev_engineeringlabs_rustboot_di::Container;
use dev_engineeringlabs_rustboot_health::{HealthCheck, HealthStatus};
use dev_engineeringlabs_rustboot_ratelimit::{RateLimiter, SlidingWindowLimiter};
use dev_engineeringlabs_rustboot_resilience::{CircuitBreaker, CircuitBreakerConfig, RetryPolicy};
use dev_engineeringlabs_rustboot_state_machine::StateMachine;
use dev_engineeringlabs_rustboot_validation::{Validate, ValidationErrors};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing::{info, warn, Level};
use uuid::Uuid;

// ============================================================================
// Domain Models
// ============================================================================

/// User creation request with validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

impl Validate for CreateUserRequest {
    fn validate(&self) -> Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();

        if self.username.len() < 3 {
            errors.add_error("username", "Username must be at least 3 characters");
        }
        if self.username.len() > 50 {
            errors.add_error("username", "Username must be at most 50 characters");
        }
        if !self.email.contains('@') {
            errors.add_error("email", "Invalid email format");
        }
        if self.password.len() < 8 {
            errors.add_error("password", "Password must be at least 8 characters");
        }

        errors.into_result()
    }
}

/// User entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
}

/// Order creation request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOrderRequest {
    pub user_id: Uuid,
    pub items: Vec<OrderItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderItem {
    pub product_id: String,
    pub quantity: u32,
    pub price: f64,
}

/// Order states for state machine
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OrderState {
    Pending,
    Confirmed,
    Processing,
    Shipped,
    Delivered,
    Cancelled,
}

impl fmt::Display for OrderState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OrderState::Pending => write!(f, "pending"),
            OrderState::Confirmed => write!(f, "confirmed"),
            OrderState::Processing => write!(f, "processing"),
            OrderState::Shipped => write!(f, "shipped"),
            OrderState::Delivered => write!(f, "delivered"),
            OrderState::Cancelled => write!(f, "cancelled"),
        }
    }
}

/// Order events
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OrderEvent {
    Confirm,
    Process,
    Ship,
    Deliver,
    Cancel,
}

/// Order entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: Uuid,
    pub user_id: Uuid,
    pub items: Vec<OrderItem>,
    pub total: f64,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

/// API error response
#[derive(Debug, Serialize)]
pub struct ApiError {
    pub error: String,
    pub message: String,
    pub code: u16,
}

/// Health check response
#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
    pub checks: HashMap<String, String>,
}

// ============================================================================
// Application State (Dependency Injection)
// ============================================================================

/// Application services container
#[derive(Clone)]
pub struct AppState {
    pub container: Arc<Container>,
    pub users: Arc<RwLock<HashMap<Uuid, User>>>,
    pub orders: Arc<RwLock<HashMap<Uuid, Order>>>,
    pub cache: Arc<InMemoryCache<String, String>>,
    pub rate_limiter: Arc<SlidingWindowLimiter>,
    pub circuit_breaker: Arc<CircuitBreaker>,
}

impl AppState {
    pub fn new() -> Self {
        let container = Container::new();

        // Configure circuit breaker for external services
        let cb_config = CircuitBreakerConfig {
            failure_threshold: 5,
            timeout: Duration::from_secs(30),
            success_threshold: 2,
        };

        // Configure rate limiter: 100 requests per minute
        let rate_limiter = SlidingWindowLimiter::new(100, Duration::from_secs(60));

        Self {
            container: Arc::new(container),
            users: Arc::new(RwLock::new(HashMap::new())),
            orders: Arc::new(RwLock::new(HashMap::new())),
            cache: Arc::new(InMemoryCache::new()),
            rate_limiter: Arc::new(rate_limiter),
            circuit_breaker: Arc::new(CircuitBreaker::new(cb_config)),
        }
    }

    /// Create an order state machine with all transitions
    pub fn create_order_state_machine() -> StateMachine<OrderState, OrderEvent> {
        let mut sm = StateMachine::new(OrderState::Pending);

        // Define valid transitions
        sm.add_transition(OrderState::Pending, OrderEvent::Confirm, OrderState::Confirmed);
        sm.add_transition(OrderState::Confirmed, OrderEvent::Process, OrderState::Processing);
        sm.add_transition(OrderState::Processing, OrderEvent::Ship, OrderState::Shipped);
        sm.add_transition(OrderState::Shipped, OrderEvent::Deliver, OrderState::Delivered);

        // Cancellation from any non-final state
        sm.add_transition(OrderState::Pending, OrderEvent::Cancel, OrderState::Cancelled);
        sm.add_transition(OrderState::Confirmed, OrderEvent::Cancel, OrderState::Cancelled);
        sm.add_transition(OrderState::Processing, OrderEvent::Cancel, OrderState::Cancelled);

        sm
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Health Checks
// ============================================================================

struct DatabaseHealthCheck;

impl HealthCheck for DatabaseHealthCheck {
    fn check(&self) -> HealthStatus {
        // Simulate database health check
        HealthStatus::Healthy
    }

    fn name(&self) -> &str {
        "database"
    }
}

struct CacheHealthCheck;

impl HealthCheck for CacheHealthCheck {
    fn check(&self) -> HealthStatus {
        HealthStatus::Healthy
    }

    fn name(&self) -> &str {
        "cache"
    }
}

// ============================================================================
// HTTP Handlers
// ============================================================================

/// Health check endpoint
async fn health_check(_state: State<AppState>) -> Json<HealthResponse> {
    let mut checks = HashMap::new();

    // Run health checks
    let db_check = DatabaseHealthCheck;
    let cache_check = CacheHealthCheck;

    match db_check.check() {
        HealthStatus::Healthy => {
            checks.insert("database".to_string(), "OK".to_string());
        }
        HealthStatus::Unhealthy => {
            checks.insert("database".to_string(), "UNHEALTHY".to_string());
        }
        HealthStatus::Degraded => {
            checks.insert("database".to_string(), "DEGRADED".to_string());
        }
    };

    match cache_check.check() {
        HealthStatus::Healthy => {
            checks.insert("cache".to_string(), "OK".to_string());
        }
        HealthStatus::Unhealthy => {
            checks.insert("cache".to_string(), "UNHEALTHY".to_string());
        }
        HealthStatus::Degraded => {
            checks.insert("cache".to_string(), "DEGRADED".to_string());
        }
    };

    let all_healthy = checks.values().all(|v| v == "OK");

    Json(HealthResponse {
        status: if all_healthy {
            "healthy".to_string()
        } else {
            "unhealthy".to_string()
        },
        version: env!("CARGO_PKG_VERSION").to_string(),
        checks,
    })
}

/// Readiness probe endpoint
async fn ready() -> StatusCode {
    StatusCode::OK
}

/// Create a new user with validation
async fn create_user(
    State(state): State<AppState>,
    Json(req): Json<CreateUserRequest>,
) -> Result<Json<User>, (StatusCode, Json<ApiError>)> {
    // Rate limiting check
    if !state.rate_limiter.try_acquire() {
        return Err((
            StatusCode::TOO_MANY_REQUESTS,
            Json(ApiError {
                error: "rate_limit_exceeded".to_string(),
                message: "Too many requests. Please try again later.".to_string(),
                code: 429,
            }),
        ));
    }

    // Validate input
    if let Err(e) = req.validate() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ApiError {
                error: "validation_error".to_string(),
                message: e.to_string(),
                code: 400,
            }),
        ));
    }

    // Check for duplicate username (with caching)
    let cache_key = format!("user:username:{}", req.username);
    if state.cache.get(&cache_key).await.is_some() {
        return Err((
            StatusCode::CONFLICT,
            Json(ApiError {
                error: "duplicate_username".to_string(),
                message: "Username already exists".to_string(),
                code: 409,
            }),
        ));
    }

    // Create user
    let user = User {
        id: Uuid::new_v4(),
        username: req.username.clone(),
        email: req.email,
        created_at: Utc::now(),
    };

    // Store user
    {
        let mut users = state.users.write().await;
        users.insert(user.id, user.clone());
    }

    // Cache the username for duplicate detection
    state
        .cache
        .set(
            cache_key,
            user.id.to_string(),
            Some(Duration::from_secs(3600)),
        )
        .await;

    info!(user_id = %user.id, username = %user.username, "User created");

    Ok(Json(user))
}

/// Get user by ID
async fn get_user(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<User>, (StatusCode, Json<ApiError>)> {
    // Try cache first
    let cache_key = format!("user:id:{}", user_id);
    if let Some(cached) = state.cache.get(&cache_key).await {
        if let Ok(user) = serde_json::from_str::<User>(&cached) {
            info!(user_id = %user_id, "Cache hit for user");
            return Ok(Json(user));
        }
    }

    // Fetch from storage
    let users = state.users.read().await;
    match users.get(&user_id) {
        Some(user) => {
            // Cache the result
            if let Ok(json) = serde_json::to_string(user) {
                state
                    .cache
                    .set(cache_key, json, Some(Duration::from_secs(300)))
                    .await;
            }
            Ok(Json(user.clone()))
        }
        None => Err((
            StatusCode::NOT_FOUND,
            Json(ApiError {
                error: "user_not_found".to_string(),
                message: format!("User with ID {} not found", user_id),
                code: 404,
            }),
        )),
    }
}

/// Create a new order with state machine
async fn create_order(
    State(state): State<AppState>,
    Json(req): Json<CreateOrderRequest>,
) -> Result<Json<Order>, (StatusCode, Json<ApiError>)> {
    // Rate limiting
    if !state.rate_limiter.try_acquire() {
        return Err((
            StatusCode::TOO_MANY_REQUESTS,
            Json(ApiError {
                error: "rate_limit_exceeded".to_string(),
                message: "Too many requests".to_string(),
                code: 429,
            }),
        ));
    }

    // Validate user exists
    {
        let users = state.users.read().await;
        if !users.contains_key(&req.user_id) {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(ApiError {
                    error: "invalid_user".to_string(),
                    message: "User does not exist".to_string(),
                    code: 400,
                }),
            ));
        }
    }

    // Calculate total
    let total: f64 = req.items.iter().map(|i| i.price * i.quantity as f64).sum();

    // Create order with initial state
    let order = Order {
        id: Uuid::new_v4(),
        user_id: req.user_id,
        items: req.items,
        total,
        status: "pending".to_string(),
        created_at: Utc::now(),
    };

    // Store order
    {
        let mut orders = state.orders.write().await;
        orders.insert(order.id, order.clone());
    }

    info!(order_id = %order.id, user_id = %order.user_id, total = order.total, "Order created");

    Ok(Json(order))
}

/// Get order by ID
async fn get_order(
    State(state): State<AppState>,
    Path(order_id): Path<Uuid>,
) -> Result<Json<Order>, (StatusCode, Json<ApiError>)> {
    let orders = state.orders.read().await;
    match orders.get(&order_id) {
        Some(order) => Ok(Json(order.clone())),
        None => Err((
            StatusCode::NOT_FOUND,
            Json(ApiError {
                error: "order_not_found".to_string(),
                message: format!("Order with ID {} not found", order_id),
                code: 404,
            }),
        )),
    }
}

/// Update order status using state machine
async fn update_order_status(
    State(state): State<AppState>,
    Path((order_id, action)): Path<(Uuid, String)>,
) -> Result<Json<Order>, (StatusCode, Json<ApiError>)> {
    let event = match action.as_str() {
        "confirm" => OrderEvent::Confirm,
        "process" => OrderEvent::Process,
        "ship" => OrderEvent::Ship,
        "deliver" => OrderEvent::Deliver,
        "cancel" => OrderEvent::Cancel,
        _ => {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(ApiError {
                    error: "invalid_action".to_string(),
                    message: format!("Invalid action: {}", action),
                    code: 400,
                }),
            ));
        }
    };

    let mut orders = state.orders.write().await;
    let order = orders.get_mut(&order_id).ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(ApiError {
                error: "order_not_found".to_string(),
                message: format!("Order with ID {} not found", order_id),
                code: 404,
            }),
        )
    })?;

    // Parse current state
    let current_state = match order.status.as_str() {
        "pending" => OrderState::Pending,
        "confirmed" => OrderState::Confirmed,
        "processing" => OrderState::Processing,
        "shipped" => OrderState::Shipped,
        "delivered" => OrderState::Delivered,
        "cancelled" => OrderState::Cancelled,
        _ => OrderState::Pending,
    };

    // Create state machine and check if transition is valid
    let sm = AppState::create_order_state_machine();

    if !sm.can_trigger(&event) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ApiError {
                error: "invalid_transition".to_string(),
                message: format!("Cannot {} order in {} state", action, order.status),
                code: 400,
            }),
        ));
    }

    // Apply the transition
    let new_status = match event {
        OrderEvent::Confirm => "confirmed",
        OrderEvent::Process => "processing",
        OrderEvent::Ship => "shipped",
        OrderEvent::Deliver => "delivered",
        OrderEvent::Cancel => "cancelled",
    };

    let old_status = order.status.clone();
    order.status = new_status.to_string();

    info!(
        order_id = %order_id,
        old_status = %old_status,
        new_status = %order.status,
        "Order status updated"
    );

    Ok(Json(order.clone()))
}

/// Simulate an external API call with resilience patterns
async fn external_api_call(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, Json<ApiError>)> {
    // Use circuit breaker for external call
    let cb = state.circuit_breaker.clone();

    // Configure retry policy
    let retry = RetryPolicy::new(3);

    let result = retry
        .execute(|| {
            let cb_inner = cb.clone();
            async move {
                cb_inner
                    .execute(|| async {
                        // Simulate external API call
                        tokio::time::sleep(Duration::from_millis(100)).await;
                        Ok::<_, String>(serde_json::json!({
                            "message": "External API response",
                            "timestamp": Utc::now().to_rfc3339()
                        }))
                    })
                    .await
            }
        })
        .await;

    match result {
        Ok(data) => Ok(Json(data)),
        Err(e) => {
            warn!(error = %e, "External API call failed");
            Err((
                StatusCode::SERVICE_UNAVAILABLE,
                Json(ApiError {
                    error: "external_api_error".to_string(),
                    message: "External service unavailable".to_string(),
                    code: 503,
                }),
            ))
        }
    }
}

// ============================================================================
// Application Setup
// ============================================================================

fn create_router(state: AppState) -> Router {
    Router::new()
        // Health endpoints
        .route("/health", get(health_check))
        .route("/ready", get(ready))
        // User endpoints
        .route("/api/users", post(create_user))
        .route("/api/users/:id", get(get_user))
        // Order endpoints
        .route("/api/orders", post(create_order))
        .route("/api/orders/:id", get(get_order))
        .route("/api/orders/:id/:action", post(update_order_status))
        // External API simulation
        .route("/api/external", get(external_api_call))
        // Middleware
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .with_state(state)
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .with_target(true)
        .json()
        .init();

    info!("Starting Rustboot E2E Example Application");

    // Create application state
    let state = AppState::new();

    // Create router
    let app = create_router(state);

    // Start server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    info!("Server listening on http://0.0.0.0:8080");

    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║        Rustboot E2E Example Application                      ║");
    println!("╠══════════════════════════════════════════════════════════════╣");
    println!("║  Server running at: http://localhost:8080                    ║");
    println!("║                                                              ║");
    println!("║  Endpoints:                                                  ║");
    println!("║    GET  /health           - Health check                     ║");
    println!("║    GET  /ready            - Readiness probe                  ║");
    println!("║    POST /api/users        - Create user                      ║");
    println!("║    GET  /api/users/:id    - Get user                         ║");
    println!("║    POST /api/orders       - Create order                     ║");
    println!("║    GET  /api/orders/:id   - Get order                        ║");
    println!("║    POST /api/orders/:id/:action - Update order status        ║");
    println!("║    GET  /api/external     - External API (with resilience)   ║");
    println!("║                                                              ║");
    println!("║  Features demonstrated:                                      ║");
    println!("║    - Dependency injection                                    ║");
    println!("║    - Input validation                                        ║");
    println!("║    - Rate limiting                                           ║");
    println!("║    - Caching                                                 ║");
    println!("║    - Circuit breaker                                         ║");
    println!("║    - Retry with backoff                                      ║");
    println!("║    - State machine (order status)                            ║");
    println!("║    - Health checks                                           ║");
    println!("║    - Structured logging                                      ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    axum::serve(listener, app).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_health_check() {
        let state = AppState::new();
        let app = create_router(state);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_ready_check() {
        let state = AppState::new();
        let app = create_router(state);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/ready")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_create_user_validation() {
        let state = AppState::new();
        let app = create_router(state);

        // Invalid request (short username)
        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/users")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        r#"{"username":"ab","email":"test@test.com","password":"password123"}"#,
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_create_user_success() {
        let state = AppState::new();
        let app = create_router(state);

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/users")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        r#"{"username":"testuser","email":"test@example.com","password":"password123"}"#,
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
