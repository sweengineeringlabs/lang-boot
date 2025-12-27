//! Rustboot Full Application Example - TODO List REST API
//!
//! This comprehensive example demonstrates integration of multiple Rustboot crates:
//! - rustboot-web: Axum-based web server and routing
//! - rustboot-database: SQLite database with migrations
//! - rustboot-session: In-memory session management
//! - rustboot-health: Health check endpoints
//! - rustboot-middleware: CORS, security headers, and logging
//! - rustboot-validation: Request validation
//! - rustboot-config: Configuration management
//!
//! Run with: cargo run --bin todo-api

use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::{Json, Router};
use chrono::{DateTime, Utc};
// Configuration management (using defaults for this example)
// use dev_engineeringlabs_rustboot_config::{ConfigLoader, EnvSource, FileSource};
use dev_engineeringlabs_rustboot_database::{Database, SqlxDatabase, Value};
use dev_engineeringlabs_rustboot_health::{
    AlwaysHealthyCheck, CheckResult, FunctionCheck, HealthAggregator,
};
// Middleware imports (can be added for CORS and security headers)
// use dev_engineeringlabs_rustboot_middleware::{
//     CorsConfig, CorsMiddleware, HttpContext, Pipeline, SecurityHeadersConfig,
//     SecurityHeadersMiddleware,
// };
use dev_engineeringlabs_rustboot_session::{MemorySessionStore, SessionConfig, SessionManager};
use dev_engineeringlabs_rustboot_validation::{StringValidationBuilder, Validator};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;
use tower_http::trace::TraceLayer;
use tracing::{error, info, warn};
use uuid::Uuid;

// ============================================================================
// Configuration
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
struct AppConfig {
    server: ServerConfig,
    database: DatabaseConfig,
    session: SessionSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ServerConfig {
    host: String,
    port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DatabaseConfig {
    url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SessionSettings {
    ttl_seconds: u64,
    cookie_name: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                host: "127.0.0.1".to_string(),
                port: 3000,
            },
            database: DatabaseConfig {
                url: "sqlite::memory:".to_string(),
            },
            session: SessionSettings {
                ttl_seconds: 3600,
                cookie_name: "todo_session".to_string(),
            },
        }
    }
}

// ============================================================================
// Domain Models
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Todo {
    id: String,
    title: String,
    description: Option<String>,
    completed: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    user_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CreateTodoRequest {
    title: String,
    description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct UpdateTodoRequest {
    title: Option<String>,
    description: Option<String>,
    completed: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct User {
    id: String,
    username: String,
    email: String,
    created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RegisterRequest {
    username: String,
    email: String,
    password: String,
}

// ============================================================================
// API Response Types
// ============================================================================

#[derive(Debug, Serialize)]
struct ApiResponse<T> {
    success: bool,
    data: Option<T>,
    message: String,
}

impl<T: Serialize> ApiResponse<T> {
    fn ok(data: T, message: impl Into<String>) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: message.into(),
        }
    }

    fn created(data: T, message: impl Into<String>) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: message.into(),
        }
    }
}

impl ApiResponse<()> {
    fn error(message: impl Into<String>) -> Self {
        Self {
            success: false,
            data: None,
            message: message.into(),
        }
    }

    fn no_content(message: impl Into<String>) -> Self {
        Self {
            success: true,
            data: None,
            message: message.into(),
        }
    }
}

// ============================================================================
// Application State
// ============================================================================

#[derive(Clone)]
struct AppState {
    db: Arc<SqlxDatabase>,
    session_manager: Arc<SessionManager<MemorySessionStore>>,
    health: Arc<HealthAggregator>,
    config: Arc<AppConfig>,
}

// ============================================================================
// Error Handling
// ============================================================================

#[derive(Debug, thiserror::Error)]
enum ApiError {
    #[error("Database error: {0}")]
    Database(String),
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Validation error: {0}")]
    Validation(String),
    #[error("Session error: {0}")]
    Session(String),
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
    #[error("Internal server error: {0}")]
    Internal(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ApiError::Database(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            ApiError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            ApiError::Validation(msg) => (StatusCode::BAD_REQUEST, msg),
            ApiError::Session(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            ApiError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
            ApiError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        let body = Json(ApiResponse::<()>::error(message));
        (status, body).into_response()
    }
}

// ============================================================================
// Database Setup and Migrations
// ============================================================================

async fn setup_database(db: &SqlxDatabase) -> Result<(), ApiError> {
    info!("Setting up database schema...");

    // Create users table
    db.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id TEXT PRIMARY KEY,
            username TEXT NOT NULL UNIQUE,
            email TEXT NOT NULL UNIQUE,
            password_hash TEXT NOT NULL,
            created_at TEXT NOT NULL
        )",
    )
    .await
    .map_err(|e| ApiError::Database(format!("Failed to create users table: {}", e)))?;

    // Create todos table
    db.execute(
        "CREATE TABLE IF NOT EXISTS todos (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            description TEXT,
            completed INTEGER NOT NULL DEFAULT 0,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            user_id TEXT,
            FOREIGN KEY (user_id) REFERENCES users(id)
        )",
    )
    .await
    .map_err(|e| ApiError::Database(format!("Failed to create todos table: {}", e)))?;

    // Create indexes
    db.execute("CREATE INDEX IF NOT EXISTS idx_todos_user_id ON todos(user_id)")
        .await
        .map_err(|e| ApiError::Database(format!("Failed to create index: {}", e)))?;

    db.execute("CREATE INDEX IF NOT EXISTS idx_todos_completed ON todos(completed)")
        .await
        .map_err(|e| ApiError::Database(format!("Failed to create index: {}", e)))?;

    info!("Database schema setup complete");
    Ok(())
}

// ============================================================================
// Validation Functions
// ============================================================================

fn validate_create_todo(req: &CreateTodoRequest) -> Result<(), ApiError> {
    let title_validator = StringValidationBuilder::new("title")
        .not_empty()
        .min_length(1)
        .max_length(200)
        .build();

    title_validator
        .validate(&req.title)
        .map_err(|e| ApiError::Validation(format!("Invalid title: {:?}", e)))?;

    if let Some(desc) = &req.description {
        let desc_validator = StringValidationBuilder::new("description")
            .max_length(1000)
            .build();

        desc_validator
            .validate(desc)
            .map_err(|e| ApiError::Validation(format!("Invalid description: {:?}", e)))?;
    }

    Ok(())
}

fn validate_register_request(req: &RegisterRequest) -> Result<(), ApiError> {
    // Username validation
    let username_validator = StringValidationBuilder::new("username")
        .not_empty()
        .min_length(3)
        .max_length(50)
        .build();

    username_validator
        .validate(&req.username)
        .map_err(|e| ApiError::Validation(format!("Invalid username: {:?}", e)))?;

    // Email validation
    let email_validator = StringValidationBuilder::new("email")
        .not_empty()
        .email()
        .build();

    email_validator
        .validate(&req.email)
        .map_err(|e| ApiError::Validation(format!("Invalid email: {:?}", e)))?;

    // Password validation
    let password_validator = StringValidationBuilder::new("password")
        .not_empty()
        .min_length(8)
        .build();

    password_validator
        .validate(&req.password)
        .map_err(|e| ApiError::Validation(format!("Invalid password: {:?}", e)))?;

    Ok(())
}

// ============================================================================
// Session Helpers
// ============================================================================

async fn get_session_user_id(
    session_manager: &SessionManager<MemorySessionStore>,
    headers: &HeaderMap,
) -> Result<Option<String>, ApiError> {
    // Extract session ID from cookie header
    let cookie = headers
        .get("cookie")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    if cookie.is_empty() {
        return Ok(None);
    }

    // Parse session ID from cookie (simplified - in production use proper cookie parsing)
    let session_id = cookie
        .split(';')
        .filter_map(|part| {
            let parts: Vec<&str> = part.trim().splitn(2, '=').collect();
            if parts.len() == 2 && parts[0] == "todo_session" {
                Some(parts[1].to_string())
            } else {
                None
            }
        })
        .next();

    if let Some(sid) = session_id {
        use dev_engineeringlabs_rustboot_session::SessionId;
        match SessionId::from_string(sid) {
            Ok(session_id) => match session_manager.load(&session_id).await {
                Ok(Some(session)) => {
                    let user_id: Option<String> = session.get("user_id").ok().flatten();
                    Ok(user_id)
                }
                Ok(None) => Ok(None),
                Err(e) => {
                    warn!("Session load error: {}", e);
                    Ok(None)
                }
            },
            Err(e) => {
                warn!("Invalid session ID: {}", e);
                Ok(None)
            }
        }
    } else {
        Ok(None)
    }
}

// ============================================================================
// API Handlers
// ============================================================================

// Health Check Handler
async fn health_handler(State(state): State<AppState>) -> impl IntoResponse {
    let report = state.health.check().await;
    let json = report.to_json().unwrap_or_else(|_| "{}".to_string());

    let status = if report.status == dev_engineeringlabs_rustboot_health::HealthStatus::Healthy {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    };

    (status, json)
}

// Root Handler
async fn root_handler() -> impl IntoResponse {
    Json(ApiResponse::ok(
        serde_json::json!({
            "name": "Rustboot TODO API",
            "version": "1.0.0",
            "endpoints": {
                "health": "GET /health",
                "auth": {
                    "register": "POST /api/auth/register",
                    "login": "POST /api/auth/login",
                    "logout": "POST /api/auth/logout"
                },
                "todos": {
                    "list": "GET /api/todos",
                    "get": "GET /api/todos/:id",
                    "create": "POST /api/todos",
                    "update": "PUT /api/todos/:id",
                    "delete": "DELETE /api/todos/:id"
                }
            }
        }),
        "Welcome to Rustboot TODO API",
    ))
}

// Register Handler
async fn register_handler(
    State(state): State<AppState>,
    Json(req): Json<RegisterRequest>,
) -> Result<impl IntoResponse, ApiError> {
    info!("Registering new user: {}", req.username);

    // Validate request
    validate_register_request(&req)?;

    // Check if username already exists
    let existing = state
        .db
        .query("SELECT id FROM users WHERE username = ?")
        .await
        .map_err(|e| ApiError::Database(e.to_string()))?;

    if !existing.is_empty() {
        return Err(ApiError::Validation("Username already exists".to_string()));
    }

    // Create user (in production, hash the password properly!)
    let user_id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();

    state
        .db
        .execute(&format!(
            "INSERT INTO users (id, username, email, password_hash, created_at)
             VALUES ('{}', '{}', '{}', '{}', '{}')",
            user_id, req.username, req.email, req.password, now
        ))
        .await
        .map_err(|e| ApiError::Database(e.to_string()))?;

    let user = User {
        id: user_id,
        username: req.username,
        email: req.email,
        created_at: Utc::now(),
    };

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse::created(user, "User registered successfully")),
    ))
}

// Login Handler
async fn login_handler(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> Result<impl IntoResponse, ApiError> {
    info!("User login attempt: {}", req.username);

    // Find user
    let rows = state
        .db
        .query(&format!(
            "SELECT id, username, email, password_hash, created_at FROM users WHERE username = '{}'",
            req.username
        ))
        .await
        .map_err(|e| ApiError::Database(e.to_string()))?;

    if rows.is_empty() {
        return Err(ApiError::Unauthorized("Invalid credentials".to_string()));
    }

    let row = &rows[0];
    let user_id = match row.get("id") {
        Some(Value::String(s)) => s.clone(),
        _ => return Err(ApiError::Internal("Invalid user data".to_string())),
    };

    let stored_password = match row.get("password_hash") {
        Some(Value::String(s)) => s.clone(),
        _ => return Err(ApiError::Internal("Invalid user data".to_string())),
    };

    // Verify password (simplified - use proper hashing in production!)
    if stored_password != req.password {
        return Err(ApiError::Unauthorized("Invalid credentials".to_string()));
    }

    // Create session
    let (session_id, _) = state
        .session_manager
        .create()
        .await
        .map_err(|e| ApiError::Session(e.to_string()))?;

    state
        .session_manager
        .update(&session_id, |data| {
            data.set("user_id", user_id.clone())?;
            Ok(())
        })
        .await
        .map_err(|e| ApiError::Session(e.to_string()))?;

    info!("User logged in successfully: {}", req.username);

    Ok(Json(ApiResponse::ok(
        serde_json::json!({
            "session_id": session_id.to_string(),
            "user_id": user_id
        }),
        "Login successful",
    )))
}

// Logout Handler
async fn logout_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<impl IntoResponse, ApiError> {
    let user_id = get_session_user_id(&state.session_manager, &headers).await?;

    if user_id.is_none() {
        return Err(ApiError::Unauthorized("Not logged in".to_string()));
    }

    info!("User logged out");

    Ok(Json(ApiResponse::no_content("Logged out successfully")))
}

// List Todos Handler
async fn list_todos_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<impl IntoResponse, ApiError> {
    let user_id = get_session_user_id(&state.session_manager, &headers).await?;

    // Query todos (filter by user if logged in)
    let query = if let Some(uid) = user_id {
        format!("SELECT * FROM todos WHERE user_id = '{}'", uid)
    } else {
        "SELECT * FROM todos WHERE user_id IS NULL".to_string()
    };

    let rows = state
        .db
        .query(&query)
        .await
        .map_err(|e| ApiError::Database(e.to_string()))?;

    let todos: Vec<Todo> = rows
        .iter()
        .filter_map(|row| {
            Some(Todo {
                id: match row.get("id")? {
                    Value::String(s) => s.clone(),
                    _ => return None,
                },
                title: match row.get("title")? {
                    Value::String(s) => s.clone(),
                    _ => return None,
                },
                description: match row.get("description") {
                    Some(Value::String(s)) => Some(s.clone()),
                    _ => None,
                },
                completed: match row.get("completed")? {
                    Value::Int(i) => *i != 0,
                    _ => false,
                },
                created_at: match row.get("created_at") {
                    Some(Value::String(s)) => DateTime::parse_from_rfc3339(s)
                        .ok()?
                        .with_timezone(&Utc),
                    _ => Utc::now(),
                },
                updated_at: match row.get("updated_at") {
                    Some(Value::String(s)) => DateTime::parse_from_rfc3339(s)
                        .ok()?
                        .with_timezone(&Utc),
                    _ => Utc::now(),
                },
                user_id: match row.get("user_id") {
                    Some(Value::String(s)) => Some(s.clone()),
                    _ => None,
                },
            })
        })
        .collect();

    Ok(Json(ApiResponse::ok(todos, "Todos retrieved successfully")))
}

// Get Todo Handler
async fn get_todo_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    let rows = state
        .db
        .query(&format!("SELECT * FROM todos WHERE id = '{}'", id))
        .await
        .map_err(|e| ApiError::Database(e.to_string()))?;

    if rows.is_empty() {
        return Err(ApiError::NotFound(format!("Todo with id {} not found", id)));
    }

    let row = &rows[0];
    let todo = Todo {
        id: match row.get("id") {
            Some(Value::String(s)) => s.clone(),
            _ => return Err(ApiError::Internal("Invalid data".to_string())),
        },
        title: match row.get("title") {
            Some(Value::String(s)) => s.clone(),
            _ => return Err(ApiError::Internal("Invalid data".to_string())),
        },
        description: match row.get("description") {
            Some(Value::String(s)) => Some(s.clone()),
            _ => None,
        },
        completed: match row.get("completed") {
            Some(Value::Int(i)) => *i != 0,
            _ => false,
        },
        created_at: match row.get("created_at") {
            Some(Value::String(s)) => {
                DateTime::parse_from_rfc3339(s).unwrap().with_timezone(&Utc)
            }
            _ => Utc::now(),
        },
        updated_at: match row.get("updated_at") {
            Some(Value::String(s)) => {
                DateTime::parse_from_rfc3339(s).unwrap().with_timezone(&Utc)
            }
            _ => Utc::now(),
        },
        user_id: match row.get("user_id") {
            Some(Value::String(s)) => Some(s.clone()),
            _ => None,
        },
    };

    Ok(Json(ApiResponse::ok(todo, "Todo retrieved successfully")))
}

// Create Todo Handler
async fn create_todo_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<CreateTodoRequest>,
) -> Result<impl IntoResponse, ApiError> {
    info!("Creating new todo: {}", req.title);

    // Validate request
    validate_create_todo(&req)?;

    let user_id = get_session_user_id(&state.session_manager, &headers).await?;

    let todo_id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();

    let user_id_clause = if let Some(uid) = &user_id {
        format!("'{}'", uid)
    } else {
        "NULL".to_string()
    };

    let desc_clause = if let Some(desc) = &req.description {
        format!("'{}'", desc)
    } else {
        "NULL".to_string()
    };

    state
        .db
        .execute(&format!(
            "INSERT INTO todos (id, title, description, completed, created_at, updated_at, user_id)
             VALUES ('{}', '{}', {}, 0, '{}', '{}', {})",
            todo_id, req.title, desc_clause, now, now, user_id_clause
        ))
        .await
        .map_err(|e| ApiError::Database(e.to_string()))?;

    let todo = Todo {
        id: todo_id,
        title: req.title,
        description: req.description,
        completed: false,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        user_id,
    };

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse::created(todo, "Todo created successfully")),
    ))
}

// Update Todo Handler
async fn update_todo_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(req): Json<UpdateTodoRequest>,
) -> Result<impl IntoResponse, ApiError> {
    info!("Updating todo: {}", id);

    // Check if todo exists
    let rows = state
        .db
        .query(&format!("SELECT id FROM todos WHERE id = '{}'", id))
        .await
        .map_err(|e| ApiError::Database(e.to_string()))?;

    if rows.is_empty() {
        return Err(ApiError::NotFound(format!("Todo with id {} not found", id)));
    }

    // Build update query
    let mut updates = Vec::new();

    if let Some(title) = &req.title {
        updates.push(format!("title = '{}'", title));
    }

    if let Some(description) = &req.description {
        updates.push(format!("description = '{}'", description));
    }

    if let Some(completed) = req.completed {
        updates.push(format!("completed = {}", if completed { 1 } else { 0 }));
    }

    if updates.is_empty() {
        return Err(ApiError::Validation("No fields to update".to_string()));
    }

    let now = Utc::now().to_rfc3339();
    updates.push(format!("updated_at = '{}'", now));

    state
        .db
        .execute(&format!(
            "UPDATE todos SET {} WHERE id = '{}'",
            updates.join(", "),
            id
        ))
        .await
        .map_err(|e| ApiError::Database(e.to_string()))?;

    Ok(Json(ApiResponse::no_content("Todo updated successfully")))
}

// Delete Todo Handler
async fn delete_todo_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    info!("Deleting todo: {}", id);

    let affected = state
        .db
        .execute(&format!("DELETE FROM todos WHERE id = '{}'", id))
        .await
        .map_err(|e| ApiError::Database(e.to_string()))?;

    if affected == 0 {
        return Err(ApiError::NotFound(format!("Todo with id {} not found", id)));
    }

    Ok(Json(ApiResponse::no_content("Todo deleted successfully")))
}

// ============================================================================
// Application Setup
// ============================================================================

async fn create_health_aggregator(db: Arc<SqlxDatabase>) -> HealthAggregator {
    HealthAggregator::new()
        .add_check(Box::new(AlwaysHealthyCheck::new("liveness")))
        .add_check(Box::new(FunctionCheck::new("database", move || {
            let db = db.clone();
            async move {
                match db.query("SELECT 1").await {
                    Ok(_) => CheckResult::healthy("database")
                        .with_message("Database connection is healthy"),
                    Err(e) => CheckResult::unhealthy(
                        "database",
                        format!("Database connection failed: {}", e),
                    ),
                }
            }
        })))
        .with_version("1.0.0")
}

fn create_app_router(state: AppState) -> Router {
    Router::new()
        // Root endpoint
        .route("/", axum::routing::get(root_handler))
        // Health check
        .route("/health", axum::routing::get(health_handler))
        // Auth endpoints
        .route("/api/auth/register", axum::routing::post(register_handler))
        .route("/api/auth/login", axum::routing::post(login_handler))
        .route("/api/auth/logout", axum::routing::post(logout_handler))
        // Todo endpoints
        .route(
            "/api/todos",
            axum::routing::get(list_todos_handler).post(create_todo_handler),
        )
        .route(
            "/api/todos/:id",
            axum::routing::get(get_todo_handler)
                .put(update_todo_handler)
                .delete(delete_todo_handler),
        )
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

// ============================================================================
// Main Entry Point
// ============================================================================

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    info!("Starting Rustboot Full Application Example - TODO API");

    // Load configuration
    let config = AppConfig::default();
    info!(
        "Server configuration: {}:{}",
        config.server.host, config.server.port
    );

    // Setup database
    info!("Connecting to database: {}", config.database.url);
    let db = SqlxDatabase::connect_sqlite(&config.database.url).await?;
    setup_database(&db).await?;
    let db = Arc::new(db);

    // Setup session management
    info!("Initializing session manager");
    let session_store = MemorySessionStore::new();
    let session_config = SessionConfig::default()
        .with_ttl(Duration::from_secs(config.session.ttl_seconds))
        .with_cookie_name(&config.session.cookie_name)
        .with_cookie_secure(false); // Set to true in production with HTTPS
    let session_manager = Arc::new(SessionManager::new(session_store, session_config));

    // Setup health checks
    info!("Configuring health checks");
    let health = Arc::new(create_health_aggregator(db.clone()).await);

    // Create application state
    let state = AppState {
        db,
        session_manager,
        health,
        config: Arc::new(config.clone()),
    };

    // Create router
    let app = create_app_router(state);

    // Start server
    let addr = format!("{}:{}", config.server.host, config.server.port);
    info!("Starting server on {}", addr);
    info!("");
    info!("Available endpoints:");
    info!("  GET    /                      - API information");
    info!("  GET    /health                - Health check");
    info!("  POST   /api/auth/register     - Register new user");
    info!("  POST   /api/auth/login        - Login");
    info!("  POST   /api/auth/logout       - Logout");
    info!("  GET    /api/todos             - List todos");
    info!("  POST   /api/todos             - Create todo");
    info!("  GET    /api/todos/:id         - Get todo by ID");
    info!("  PUT    /api/todos/:id         - Update todo");
    info!("  DELETE /api/todos/:id         - Delete todo");
    info!("");
    info!("Press Ctrl+C to stop the server");

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
