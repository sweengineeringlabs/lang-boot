//! JSON API example demonstrating JSON request/response handling.

use dev_engineeringlabs_rustboot_web::{
    HandlerContext, Json, JsonResponse, Response, Router, StatusCode, WebResult,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: u64,
    name: String,
    email: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct CreateUserRequest {
    name: String,
    email: String,
}

#[derive(Debug, Serialize)]
struct ApiResponse<T> {
    success: bool,
    data: Option<T>,
    message: String,
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
    ];

    let response = ApiResponse {
        success: true,
        data: Some(users),
        message: "Users retrieved successfully".to_string(),
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

async fn create_user(ctx: HandlerContext) -> WebResult<Response> {
    // Parse JSON body
    let create_request: CreateUserRequest = ctx.json()?;

    let user = User {
        id: 123, // In a real app, this would be generated
        name: create_request.name,
        email: create_request.email,
    };

    let response = ApiResponse {
        success: true,
        data: Some(user),
        message: "User created successfully".to_string(),
    };

    JsonResponse::created(&response).map_err(Into::into)
}

async fn update_user(ctx: HandlerContext) -> WebResult<Response> {
    let user_id = ctx
        .param("id")
        .and_then(|id| id.parse::<u64>().ok())
        .unwrap_or(0);

    // Parse JSON body
    let update_request: CreateUserRequest = ctx.json()?;

    let user = User {
        id: user_id,
        name: update_request.name,
        email: update_request.email,
    };

    let response = ApiResponse {
        success: true,
        data: Some(user),
        message: "User updated successfully".to_string(),
    };

    JsonResponse::ok(&response).map_err(Into::into)
}

async fn delete_user(ctx: HandlerContext) -> WebResult<Response> {
    let user_id = ctx.param("id").unwrap_or("unknown");

    let response = ApiResponse::<()> {
        success: true,
        data: None,
        message: format!("User {} deleted successfully", user_id),
    };

    JsonResponse::ok(&response).map_err(Into::into)
}

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Build the router
    let router = Router::builder()
        .get("/api/users", list_users)
        .get("/api/users/:id", get_user)
        .post("/api/users", create_user)
        .put("/api/users/:id", update_user)
        .delete("/api/users/:id", delete_user)
        .build();

    println!("JSON API Router created with {} routes:", router.routes().len());
    for (method, path) in router.routes() {
        println!("  {} {}", method.as_str(), path);
    }

    // Test listing users
    println!("\n=== Testing GET /api/users ===");
    let ctx = HandlerContext::new("GET".to_string(), "/api/users".to_string());
    match router.handle(ctx).await {
        Ok(response) => {
            println!("Status: {}", response.status.as_u16());
            println!("Body: {}", String::from_utf8_lossy(&response.body));
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    // Test getting a specific user
    println!("\n=== Testing GET /api/users/1 ===");
    let ctx = HandlerContext::new("GET".to_string(), "/api/users/1".to_string());
    match router.handle(ctx).await {
        Ok(response) => {
            println!("Status: {}", response.status.as_u16());
            println!("Body: {}", String::from_utf8_lossy(&response.body));
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    // Test creating a user
    println!("\n=== Testing POST /api/users ===");
    let create_request = CreateUserRequest {
        name: "Charlie".to_string(),
        email: "charlie@example.com".to_string(),
    };
    let mut ctx = HandlerContext::new("POST".to_string(), "/api/users".to_string());
    ctx.set_body(serde_json::to_vec(&create_request).unwrap());
    match router.handle(ctx).await {
        Ok(response) => {
            println!("Status: {}", response.status.as_u16());
            println!("Body: {}", String::from_utf8_lossy(&response.body));
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
