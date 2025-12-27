//! Extractors demonstration showing how to use request extractors.

use dev_engineeringlabs_rustboot_web::{
    HandlerContext, Headers, Json, JsonResponse, Path, Query, Response, Router, WebResult,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
struct CreateUserRequest {
    name: String,
    email: String,
}

#[derive(Debug, Serialize)]
struct UserResponse {
    id: u64,
    name: String,
    email: String,
}

#[derive(Debug, Deserialize)]
struct SearchQuery {
    q: String,
    limit: Option<u32>,
}

async fn json_handler(ctx: HandlerContext) -> WebResult<Response> {
    // Extract JSON from request body
    let user_data: CreateUserRequest = ctx.json()?;

    println!("Received user data: {:?}", user_data);

    let response = UserResponse {
        id: 123,
        name: user_data.name,
        email: user_data.email,
    };

    JsonResponse::ok(&response).map_err(Into::into)
}

async fn path_handler(ctx: HandlerContext) -> WebResult<Response> {
    // Extract path parameters
    let user_id = ctx.param("user_id").unwrap_or("unknown");
    let post_id = ctx.param("post_id").unwrap_or("unknown");

    println!("Path params - user_id: {}, post_id: {}", user_id, post_id);

    Ok(Response::ok().with_text(format!(
        "User {} - Post {}",
        user_id, post_id
    )))
}

async fn query_handler(ctx: HandlerContext) -> WebResult<Response> {
    // Extract query parameters
    let query = Query::<HashMap<String, String>>::from_context(&ctx)?;

    println!("Query params: {:?}", *query);

    let search_term = query.get("q").map(|s| s.as_str()).unwrap_or("none");
    let limit = query
        .get("limit")
        .and_then(|s| s.parse::<u32>().ok())
        .unwrap_or(10);

    Ok(Response::ok().with_text(format!(
        "Search: '{}', Limit: {}",
        search_term, limit
    )))
}

async fn headers_handler(ctx: HandlerContext) -> WebResult<Response> {
    // Extract headers
    let headers = Headers::from_context(&ctx)?;

    println!("Request headers:");
    for (key, value) in headers.iter() {
        println!("  {}: {}", key, value);
    }

    let content_type = headers
        .get_case_insensitive("content-type")
        .unwrap_or("not set");
    let auth = headers
        .get_case_insensitive("authorization")
        .unwrap_or("not set");

    Ok(Response::ok().with_text(format!(
        "Content-Type: {}\nAuthorization: {}",
        content_type, auth
    )))
}

async fn combined_handler(ctx: HandlerContext) -> WebResult<Response> {
    // Extract multiple things from the context
    let user_id = ctx.param("id").unwrap_or("unknown");
    let headers = Headers::from_context(&ctx)?;
    let query = Query::<HashMap<String, String>>::from_context(&ctx)?;

    let include_details = query
        .get("details")
        .map(|s| s == "true")
        .unwrap_or(false);
    let auth_header = headers
        .get_case_insensitive("authorization")
        .unwrap_or("none");

    let response_text = format!(
        "User ID: {}\nInclude Details: {}\nAuth: {}",
        user_id, include_details, auth_header
    );

    Ok(Response::ok().with_text(response_text))
}

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Build the router
    let router = Router::builder()
        .post("/api/users", json_handler)
        .get("/users/:user_id/posts/:post_id", path_handler)
        .get("/search", query_handler)
        .get("/headers", headers_handler)
        .get("/users/:id", combined_handler)
        .build();

    println!("Extractors Demo");
    println!("===============\n");

    // Test JSON extraction
    println!("=== Testing JSON Extractor ===");
    let user_data = CreateUserRequest {
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
    };
    let mut ctx = HandlerContext::new("POST".to_string(), "/api/users".to_string());
    ctx.set_body(serde_json::to_vec(&user_data).unwrap());
    ctx.set_header("content-type", "application/json");

    match router.handle(ctx).await {
        Ok(response) => {
            println!("Status: {}", response.status.as_u16());
            println!("Body: {}", String::from_utf8_lossy(&response.body));
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    // Test Path extraction
    println!("\n=== Testing Path Extractor ===");
    let ctx = HandlerContext::new("GET".to_string(), "/users/42/posts/99".to_string());
    match router.handle(ctx).await {
        Ok(response) => {
            println!("Status: {}", response.status.as_u16());
            println!("Body: {}", String::from_utf8_lossy(&response.body));
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    // Test Query extraction
    println!("\n=== Testing Query Extractor ===");
    let mut ctx = HandlerContext::new("GET".to_string(), "/search".to_string());
    ctx.set_query("q", "rust");
    ctx.set_query("limit", "20");

    match router.handle(ctx).await {
        Ok(response) => {
            println!("Status: {}", response.status.as_u16());
            println!("Body: {}", String::from_utf8_lossy(&response.body));
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    // Test Headers extraction
    println!("\n=== Testing Headers Extractor ===");
    let mut ctx = HandlerContext::new("GET".to_string(), "/headers".to_string());
    ctx.set_header("Content-Type", "application/json");
    ctx.set_header("Authorization", "Bearer token123");
    ctx.set_header("User-Agent", "Rustboot/1.0");

    match router.handle(ctx).await {
        Ok(response) => {
            println!("Status: {}", response.status.as_u16());
            println!("Body: {}", String::from_utf8_lossy(&response.body));
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    // Test combined extraction
    println!("\n=== Testing Combined Extractors ===");
    let mut ctx = HandlerContext::new("GET".to_string(), "/users/123".to_string());
    ctx.set_query("details", "true");
    ctx.set_header("Authorization", "Bearer secret-token");

    match router.handle(ctx).await {
        Ok(response) => {
            println!("Status: {}", response.status.as_u16());
            println!("Body: {}", String::from_utf8_lossy(&response.body));
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
