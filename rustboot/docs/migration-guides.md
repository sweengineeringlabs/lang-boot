# Migration Guides

This guide helps you migrate from other Rust frameworks to Rustboot.

## Table of Contents

1. [From Actix-web](#from-actix-web)
2. [From Rocket](#from-rocket)
3. [From Warp](#from-warp)
4. [From Tower/Axum](#from-toweraxum)
5. [General Migration Tips](#general-migration-tips)

---

## From Actix-web

### HTTP Server Setup

**Actix-web:**
```rust
use actix_web::{web, App, HttpServer, HttpResponse};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/users", web::post().to(create_user))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hello World")
}
```

**Rustboot (with Axum integration):**
```rust
use axum::{routing::{get, post}, Router};
use dev_engineeringlabs_rustboot_middleware::*;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .route("/users", post(create_user));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn index() -> &'static str {
    "Hello World"
}
```

### Request Extractors

**Actix-web:**
```rust
use actix_web::{web, HttpRequest};

async fn handler(
    req: HttpRequest,
    path: web::Path<(u32, String)>,
    query: web::Query<QueryParams>,
    body: web::Json<CreateUser>,
) -> impl Responder {
    // ...
}
```

**Rustboot:**
```rust
use axum::{
    extract::{Path, Query, State, Json},
    http::Request,
};

async fn handler(
    Path((id, name)): Path<(u32, String)>,
    Query(params): Query<QueryParams>,
    Json(body): Json<CreateUser>,
) -> impl IntoResponse {
    // ...
}
```

### Middleware

**Actix-web:**
```rust
use actix_web::middleware::Logger;

App::new()
    .wrap(Logger::default())
    .wrap(Cors::default())
```

**Rustboot:**
```rust
use dev_engineeringlabs_rustboot_middleware::{
    cors::CorsMiddleware,
    logging::HttpLoggingMiddleware,
};
use tower_http::trace::TraceLayer;

Router::new()
    .layer(TraceLayer::new_for_http())
    .layer(CorsLayer::permissive())
```

### Dependency Injection

**Actix-web:**
```rust
use actix_web::web::Data;

let db = Database::connect().await;
App::new().app_data(Data::new(db))

async fn handler(db: Data<Database>) -> impl Responder {
    // Use db
}
```

**Rustboot:**
```rust
use dev_engineeringlabs_rustboot_di::Container;
use axum::extract::State;

let container = Container::new();
container.register(Database::connect().await);

let app = Router::new()
    .route("/", get(handler))
    .with_state(container);

async fn handler(State(container): State<Container>) -> impl IntoResponse {
    let db = container.resolve::<Database>().unwrap();
    // Use db
}
```

---

## From Rocket

### Route Definition

**Rocket:**
```rust
#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/users", data = "<user>")]
fn create_user(user: Json<User>) -> Status {
    Status::Created
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, create_user])
}
```

**Rustboot:**
```rust
use axum::{routing::{get, post}, Router, Json};
use dev_engineeringlabs_rustboot_validation::Validator;

async fn index() -> &'static str {
    "Hello, world!"
}

async fn create_user(Json(user): Json<User>) -> StatusCode {
    user.validate()?;
    StatusCode::CREATED
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .route("/users", post(create_user));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

### Request Guards

**Rocket:**
```rust
#[derive(Debug)]
struct AdminUser(User);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AdminUser {
    type Error = AuthError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // Validate admin
    }
}

#[get("/admin")]
fn admin_panel(user: AdminUser) -> &'static str {
    "Admin Panel"
}
```

**Rustboot:**
```rust
use axum::{
    async_trait,
    extract::FromRequestParts,
    http::request::Parts,
};
use dev_engineeringlabs_rustboot_security::*;

struct AdminUser(User);

#[async_trait]
impl<S> FromRequestParts<S> for AdminUser
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Validate admin using rustboot-security
        let token = parts.headers.get("Authorization")
            .ok_or((StatusCode::UNAUTHORIZED, "Missing token"))?;
        // Validate token...
        Ok(AdminUser(user))
    }
}
```

### Database

**Rocket:**
```rust
use rocket_db_pools::{Database, Connection};

#[derive(Database)]
#[database("sqlite_logs")]
struct Logs(sqlx::SqlitePool);

#[get("/")]
async fn index(mut db: Connection<Logs>) -> String {
    sqlx::query("SELECT ...").fetch_one(&mut *db).await
}
```

**Rustboot:**
```rust
use dev_engineeringlabs_rustboot_database::{Pool, SqlxDriver};

async fn setup_db() -> Pool<SqlxDriver> {
    PoolBuilder::new()
        .connection_string(&database_url)
        .build::<SqlxDriver>()
        .await
        .unwrap()
}

async fn index(State(pool): State<Pool<SqlxDriver>>) -> String {
    let conn = pool.acquire().await.unwrap();
    sqlx::query("SELECT ...").fetch_one(&conn).await
}
```

---

## From Warp

### Filter Composition

**Warp:**
```rust
use warp::Filter;

let hello = warp::path!("hello" / String)
    .map(|name| format!("Hello, {}!", name));

let api = warp::path("api")
    .and(
        warp::path("users").and(warp::get()).and_then(list_users)
            .or(warp::path("users").and(warp::post()).and_then(create_user))
    );

warp::serve(hello.or(api)).run(([127, 0, 0, 1], 3030)).await;
```

**Rustboot:**
```rust
use axum::{routing::{get, post}, Router};

async fn hello(Path(name): Path<String>) -> String {
    format!("Hello, {}!", name)
}

let app = Router::new()
    .route("/hello/:name", get(hello))
    .nest("/api", Router::new()
        .route("/users", get(list_users))
        .route("/users", post(create_user))
    );

let listener = tokio::net::TcpListener::bind("0.0.0.0:3030").await.unwrap();
axum::serve(listener, app).await.unwrap();
```

### Filter Extraction

**Warp:**
```rust
let json_body = warp::body::json::<User>();
let query = warp::query::<QueryParams>();

let route = warp::path("users")
    .and(warp::post())
    .and(json_body)
    .and(query)
    .and_then(create_user);
```

**Rustboot:**
```rust
use axum::extract::{Json, Query};
use dev_engineeringlabs_rustboot_validation::Validator;

async fn create_user(
    Json(user): Json<User>,
    Query(params): Query<QueryParams>,
) -> impl IntoResponse {
    user.validate()?;
    // Create user...
}

let app = Router::new()
    .route("/users", post(create_user));
```

### Rejection Handling

**Warp:**
```rust
use warp::reject::{Reject, custom};

#[derive(Debug)]
struct CustomError;
impl Reject for CustomError {}

async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    if err.is_not_found() {
        Ok(warp::reply::with_status("NOT_FOUND", StatusCode::NOT_FOUND))
    } else {
        Ok(warp::reply::with_status("INTERNAL_ERROR", StatusCode::INTERNAL_SERVER_ERROR))
    }
}
```

**Rustboot:**
```rust
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

// Use with Result<T, AppError> in handlers
```

---

## From Tower/Axum

Rustboot is designed to work seamlessly with Axum, so migration is straightforward.

### Adding Rustboot Features

**Before (vanilla Axum):**
```rust
use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "Hello" }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

**After (with Rustboot):**
```rust
use axum::{Router, routing::get, extract::State};
use dev_engineeringlabs_rustboot_di::Container;
use dev_engineeringlabs_rustboot_cache::InMemoryCache;
use dev_engineeringlabs_rustboot_health::{HealthCheck, HealthStatus};
use dev_engineeringlabs_rustboot_ratelimit::SlidingWindowLimiter;
use std::sync::Arc;

#[derive(Clone)]
struct AppState {
    container: Arc<Container>,
    cache: Arc<InMemoryCache<String, String>>,
    rate_limiter: Arc<SlidingWindowLimiter>,
}

#[tokio::main]
async fn main() {
    let state = AppState {
        container: Arc::new(Container::new()),
        cache: Arc::new(InMemoryCache::new()),
        rate_limiter: Arc::new(SlidingWindowLimiter::new(100, Duration::from_secs(60))),
    };

    let app = Router::new()
        .route("/", get(index))
        .route("/health", get(health))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn index(State(state): State<AppState>) -> &'static str {
    if !state.rate_limiter.try_acquire() {
        return "Rate limited";
    }
    "Hello"
}
```

### Adding Validation

**Before:**
```rust
async fn create_user(Json(user): Json<CreateUser>) -> impl IntoResponse {
    // Manual validation
    if user.email.is_empty() {
        return (StatusCode::BAD_REQUEST, "Email required");
    }
    // Create user
}
```

**After:**
```rust
use dev_engineeringlabs_rustboot_validation::{Validator, ValidationError};

impl Validator for CreateUser {
    fn validate(&self) -> Result<(), ValidationError> {
        if self.email.is_empty() {
            return Err(ValidationError::field("email", "Email is required"));
        }
        Ok(())
    }
}

async fn create_user(Json(user): Json<CreateUser>) -> Result<impl IntoResponse, AppError> {
    user.validate()?;
    // Create user
    Ok(StatusCode::CREATED)
}
```

---

## General Migration Tips

### 1. Start with Core Infrastructure

Migrate in this order:
1. HTTP routing and handlers
2. Request/response types
3. Middleware
4. Database connections
5. Caching
6. Authentication

### 2. Use Rustboot DI Container

Replace framework-specific state management:

```rust
use dev_engineeringlabs_rustboot_di::Container;

// Register all services
let container = Container::new();
container.register(database_pool);
container.register(cache);
container.register(config);

// Inject via Container in handlers
async fn handler(State(container): State<Arc<Container>>) -> impl IntoResponse {
    let db = container.resolve::<Pool>().unwrap();
    let cache = container.resolve::<Cache>().unwrap();
    // Use services
}
```

### 3. Add Validation Layer

Replace ad-hoc validation with `rustboot-validation`:

```rust
use dev_engineeringlabs_rustboot_validation::{Validator, ValidationError};

impl Validator for YourRequest {
    fn validate(&self) -> Result<(), ValidationError> {
        // Centralized validation logic
    }
}
```

### 4. Implement Resilience Patterns

Add circuit breakers and retries for external calls:

```rust
use dev_engineeringlabs_rustboot_resilience::{CircuitBreaker, RetryPolicy};

let cb = CircuitBreaker::new(config);
let retry = RetryPolicy::new(3);

// Wrap external calls
let result = retry.execute(|| async {
    cb.execute(|| async {
        external_api.call().await
    }).await
}).await;
```

### 5. Use Consistent Error Handling

Create a unified error type:

```rust
use thiserror::Error;
use dev_engineeringlabs_rustboot_validation::ValidationError;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Validation error: {0}")]
    Validation(#[from] ValidationError),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Not found: {0}")]
    NotFound(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::Validation(e) => (StatusCode::BAD_REQUEST, e.to_string()),
            AppError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error".to_string()),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
        };

        (status, Json(json!({ "error": message }))).into_response()
    }
}
```

### 6. Add Health Checks

Implement health endpoints using `rustboot-health`:

```rust
use dev_engineeringlabs_rustboot_health::{HealthCheck, HealthStatus};

struct DatabaseHealth { pool: Pool };

#[async_trait]
impl HealthCheck for DatabaseHealth {
    async fn check(&self) -> HealthStatus {
        match self.pool.acquire().await {
            Ok(_) => HealthStatus::Healthy("Connected".to_string()),
            Err(e) => HealthStatus::Unhealthy(e.to_string()),
        }
    }

    fn name(&self) -> &str { "database" }
}
```

### 7. Enable Observability

Add structured logging and tracing:

```rust
use tracing::{info, warn, error, instrument};
use dev_engineeringlabs_rustboot_observability::*;

#[instrument(skip(state))]
async fn handler(State(state): State<AppState>) -> impl IntoResponse {
    info!(user_id = %user.id, "Processing request");

    match do_something().await {
        Ok(result) => {
            info!("Success");
            Ok(Json(result))
        }
        Err(e) => {
            error!(error = %e, "Failed");
            Err(AppError::from(e))
        }
    }
}
```

---

## Migration Checklist

- [ ] Set up Rustboot dependencies in `Cargo.toml`
- [ ] Configure DI container with services
- [ ] Migrate route definitions
- [ ] Implement `Validator` for request types
- [ ] Add middleware (CORS, logging, security headers)
- [ ] Set up health check endpoints
- [ ] Configure rate limiting
- [ ] Add caching layer
- [ ] Implement resilience patterns for external calls
- [ ] Set up structured logging
- [ ] Update error handling
- [ ] Write/update tests
- [ ] Update documentation
