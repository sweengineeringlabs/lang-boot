//! Axum server example demonstrating integration with axum web framework.

use dev_engineeringlabs_rustboot_web::{
    AxumRouterBuilder, HandlerContext, JsonResponse, Response, WebError, WebResult,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: u64,
    name: String,
    email: String,
}

#[derive(Debug, Serialize)]
struct ApiResponse<T> {
    success: bool,
    data: Option<T>,
    message: String,
}

async fn home_handler(_ctx: HandlerContext) -> WebResult<Response> {
    Ok(Response::ok().with_text("Welcome to Rustboot Web with Axum!"))
}

async fn health_handler(_ctx: HandlerContext) -> WebResult<Response> {
    let response = ApiResponse {
        success: true,
        data: Some("healthy"),
        message: "Service is running".to_string(),
    };

    JsonResponse::ok(&response).map_err(Into::into)
}

async fn get_user(ctx: HandlerContext) -> WebResult<Response> {
    let user_id = ctx
        .param("id")
        .and_then(|id| id.parse::<u64>().ok())
        .unwrap_or(0);

    let user = User {
        id: user_id,
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
    };

    let response = ApiResponse {
        success: true,
        data: Some(user),
        message: "User retrieved successfully".to_string(),
    };

    JsonResponse::ok(&response).map_err(Into::into)
}

async fn list_users(_ctx: HandlerContext) -> WebResult<Response> {
    let users = vec![
        User {
            id: 1,
            name: "Alice".to_string(),
            email: "alice@example.com".to_string(),
        },
        User {
            id: 2,
            name: "Bob".to_string(),
            email: "bob@example.com".to_string(),
        },
        User {
            id: 3,
            name: "Charlie".to_string(),
            email: "charlie@example.com".to_string(),
        },
    ];

    let response = ApiResponse {
        success: true,
        data: Some(users),
        message: "Users retrieved successfully".to_string(),
    };

    JsonResponse::ok(&response).map_err(Into::into)
}

async fn echo_handler(ctx: HandlerContext) -> WebResult<Response> {
    let message = ctx.param("message").unwrap_or("No message provided");
    Ok(Response::ok().with_text(format!("Echo: {}", message)))
}

#[tokio::main]
async fn main() -> WebResult<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Build the Axum router using Rustboot handlers
    let app = AxumRouterBuilder::new()
        .get("/", home_handler)
        .get("/health", health_handler)
        .get("/api/users", list_users)
        .get("/api/users/:id", get_user)
        .get("/echo/:message", echo_handler)
        .build();

    // Get the server address from environment or use default
    let addr = std::env::var("SERVER_ADDR").unwrap_or_else(|_| "127.0.0.1:3000".to_string());

    println!("Starting Rustboot Web server...");
    println!("Listening on http://{}", addr);
    println!("\nAvailable routes:");
    println!("  GET  /");
    println!("  GET  /health");
    println!("  GET  /api/users");
    println!("  GET  /api/users/:id");
    println!("  GET  /echo/:message");
    println!("\nPress Ctrl+C to stop the server");

    // Create listener and serve
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .map_err(|e| WebError::ServerError(format!("Failed to bind: {}", e)))?;

    axum::serve(listener, app)
        .await
        .map_err(|e| WebError::ServerError(format!("Server error: {}", e)))?;

    Ok(())
}
