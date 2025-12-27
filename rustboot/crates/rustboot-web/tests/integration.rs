//! Integration tests for rustboot-web.

use dev_engineeringlabs_rustboot_web::{
    HandlerContext, Headers, Json, JsonResponse, Query, Response, Router, RouterBuilder,
    RouteMethod, StatusCode, WebError, WebResult,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct TestUser {
    id: u64,
    name: String,
}

// Helper handlers for tests
async fn test_handler(_ctx: HandlerContext) -> WebResult<Response> {
    Ok(Response::ok().with_text("test response"))
}

async fn echo_handler(ctx: HandlerContext) -> WebResult<Response> {
    let message = ctx.param("message").unwrap_or("empty");
    Ok(Response::ok().with_text(message))
}

async fn json_handler(ctx: HandlerContext) -> WebResult<Response> {
    let user: TestUser = ctx.json()?;
    JsonResponse::ok(&user).map_err(Into::into)
}

#[tokio::test]
async fn test_router_basic_routing() {
    let router = Router::builder()
        .get("/", test_handler)
        .get("/test", test_handler)
        .build();

    // Test root route
    let ctx = HandlerContext::new("GET".to_string(), "/".to_string());
    let response = router.handle(ctx).await.unwrap();
    assert_eq!(response.status, StatusCode::Ok);
    assert_eq!(response.body, b"test response");

    // Test /test route
    let ctx = HandlerContext::new("GET".to_string(), "/test".to_string());
    let response = router.handle(ctx).await.unwrap();
    assert_eq!(response.status, StatusCode::Ok);
}

#[tokio::test]
async fn test_router_path_parameters() {
    let router = Router::builder()
        .get("/echo/:message", echo_handler)
        .build();

    let ctx = HandlerContext::new("GET".to_string(), "/echo/hello".to_string());
    let response = router.handle(ctx).await.unwrap();
    assert_eq!(response.status, StatusCode::Ok);
    assert_eq!(response.body, b"hello");

    let ctx = HandlerContext::new("GET".to_string(), "/echo/world".to_string());
    let response = router.handle(ctx).await.unwrap();
    assert_eq!(response.body, b"world");
}

#[tokio::test]
async fn test_router_multiple_path_parameters() {
    let router = Router::builder()
        .get("/users/:user_id/posts/:post_id", |ctx: HandlerContext| async move {
            let user_id = ctx.param("user_id").unwrap_or("?");
            let post_id = ctx.param("post_id").unwrap_or("?");
            Ok(Response::ok().with_text(format!("{}-{}", user_id, post_id)))
        })
        .build();

    let ctx = HandlerContext::new("GET".to_string(), "/users/42/posts/99".to_string());
    let response = router.handle(ctx).await.unwrap();
    assert_eq!(response.body, b"42-99");
}

#[tokio::test]
async fn test_router_not_found() {
    let router = Router::builder().get("/exists", test_handler).build();

    let ctx = HandlerContext::new("GET".to_string(), "/does-not-exist".to_string());
    let result = router.handle(ctx).await;

    assert!(result.is_err());
    match result.unwrap_err() {
        WebError::NotFound(_) => {}
        _ => panic!("Expected NotFound error"),
    }
}

#[tokio::test]
async fn test_router_method_routing() {
    let router = Router::builder()
        .get("/resource", |_| async { Ok(Response::ok().with_text("GET")) })
        .post("/resource", |_| async { Ok(Response::ok().with_text("POST")) })
        .put("/resource", |_| async { Ok(Response::ok().with_text("PUT")) })
        .delete("/resource", |_| async { Ok(Response::ok().with_text("DELETE")) })
        .patch("/resource", |_| async { Ok(Response::ok().with_text("PATCH")) })
        .build();

    // Test GET
    let ctx = HandlerContext::new("GET".to_string(), "/resource".to_string());
    let response = router.handle(ctx).await.unwrap();
    assert_eq!(response.body, b"GET");

    // Test POST
    let ctx = HandlerContext::new("POST".to_string(), "/resource".to_string());
    let response = router.handle(ctx).await.unwrap();
    assert_eq!(response.body, b"POST");

    // Test PUT
    let ctx = HandlerContext::new("PUT".to_string(), "/resource".to_string());
    let response = router.handle(ctx).await.unwrap();
    assert_eq!(response.body, b"PUT");

    // Test DELETE
    let ctx = HandlerContext::new("DELETE".to_string(), "/resource".to_string());
    let response = router.handle(ctx).await.unwrap();
    assert_eq!(response.body, b"DELETE");

    // Test PATCH
    let ctx = HandlerContext::new("PATCH".to_string(), "/resource".to_string());
    let response = router.handle(ctx).await.unwrap();
    assert_eq!(response.body, b"PATCH");
}

#[tokio::test]
async fn test_handler_context_params() {
    let mut ctx = HandlerContext::new("GET".to_string(), "/test".to_string());
    ctx.set_param("id", "123");
    ctx.set_param("name", "alice");

    assert_eq!(ctx.param("id"), Some("123"));
    assert_eq!(ctx.param("name"), Some("alice"));
    assert_eq!(ctx.param("missing"), None);
}

#[tokio::test]
async fn test_handler_context_query() {
    let mut ctx = HandlerContext::new("GET".to_string(), "/search".to_string());
    ctx.set_query("q", "rust");
    ctx.set_query("limit", "10");

    assert_eq!(ctx.query_param("q"), Some("rust"));
    assert_eq!(ctx.query_param("limit"), Some("10"));
    assert_eq!(ctx.query_param("missing"), None);
}

#[tokio::test]
async fn test_handler_context_headers() {
    let mut ctx = HandlerContext::new("GET".to_string(), "/".to_string());
    ctx.set_header("content-type", "application/json");
    ctx.set_header("authorization", "Bearer token");

    assert_eq!(ctx.header("content-type"), Some("application/json"));
    assert_eq!(ctx.header("authorization"), Some("Bearer token"));
    assert_eq!(ctx.header("missing"), None);
}

#[tokio::test]
async fn test_handler_context_json() {
    let user = TestUser {
        id: 1,
        name: "Alice".to_string(),
    };

    let mut ctx = HandlerContext::new("POST".to_string(), "/users".to_string());
    ctx.set_body(serde_json::to_vec(&user).unwrap());

    let parsed: TestUser = ctx.json().unwrap();
    assert_eq!(parsed, user);
}

#[tokio::test]
async fn test_handler_context_text() {
    let mut ctx = HandlerContext::new("POST".to_string(), "/test".to_string());
    ctx.set_body(b"Hello, World!".to_vec());

    let text = ctx.text().unwrap();
    assert_eq!(text, "Hello, World!");
}

#[tokio::test]
async fn test_response_builder() {
    let response = Response::ok()
        .with_text("Hello")
        .with_header("x-custom", "value");

    assert_eq!(response.status, StatusCode::Ok);
    assert_eq!(response.body, b"Hello");
    assert_eq!(response.headers.get("x-custom"), Some(&"value".to_string()));
    assert_eq!(
        response.headers.get("content-type"),
        Some(&"text/plain".to_string())
    );
}

#[tokio::test]
async fn test_response_json() {
    let user = TestUser {
        id: 1,
        name: "Alice".to_string(),
    };

    let response = Response::ok().with_json(&user).unwrap();

    assert_eq!(response.status, StatusCode::Ok);
    assert_eq!(
        response.headers.get("content-type"),
        Some(&"application/json".to_string())
    );

    let parsed: TestUser = serde_json::from_slice(&response.body).unwrap();
    assert_eq!(parsed, user);
}

#[tokio::test]
async fn test_json_response_helper() {
    let user = TestUser {
        id: 1,
        name: "Alice".to_string(),
    };

    let response = JsonResponse::ok(&user).unwrap();
    assert_eq!(response.status, StatusCode::Ok);

    let response = JsonResponse::created(&user).unwrap();
    assert_eq!(response.status, StatusCode::Created);

    let response = JsonResponse::bad_request(&user).unwrap();
    assert_eq!(response.status, StatusCode::BadRequest);
}

#[tokio::test]
async fn test_extractors_json() {
    let user = TestUser {
        id: 1,
        name: "Alice".to_string(),
    };

    let mut ctx = HandlerContext::new("POST".to_string(), "/users".to_string());
    ctx.set_body(serde_json::to_vec(&user).unwrap());

    let extracted = Json::<TestUser>::from_context(&ctx).unwrap();
    assert_eq!(extracted.id, 1);
    assert_eq!(extracted.name, "Alice");
}

#[tokio::test]
async fn test_extractors_query() {
    let mut ctx = HandlerContext::new("GET".to_string(), "/search".to_string());
    ctx.set_query("q", "rust");
    ctx.set_query("limit", "10");

    let query = Query::<HashMap<String, String>>::from_context(&ctx).unwrap();
    assert_eq!(query.get("q"), Some(&"rust".to_string()));
    assert_eq!(query.get("limit"), Some(&"10".to_string()));
}

#[tokio::test]
async fn test_extractors_headers() {
    let mut ctx = HandlerContext::new("GET".to_string(), "/".to_string());
    ctx.set_header("Content-Type", "application/json");
    ctx.set_header("Authorization", "Bearer token");

    let headers = Headers::from_context(&ctx).unwrap();
    assert_eq!(headers.get("Content-Type"), Some("application/json"));
    assert_eq!(headers.get("Authorization"), Some("Bearer token"));
}

#[tokio::test]
async fn test_headers_case_insensitive() {
    let mut ctx = HandlerContext::new("GET".to_string(), "/".to_string());
    ctx.set_header("Content-Type", "application/json");

    let headers = Headers::from_context(&ctx).unwrap();
    assert_eq!(
        headers.get_case_insensitive("content-type"),
        Some("application/json")
    );
    assert_eq!(
        headers.get_case_insensitive("CONTENT-TYPE"),
        Some("application/json")
    );
    assert_eq!(
        headers.get_case_insensitive("Content-Type"),
        Some("application/json")
    );
}

#[tokio::test]
async fn test_status_code_helpers() {
    assert_eq!(StatusCode::Ok.as_u16(), 200);
    assert_eq!(StatusCode::Created.as_u16(), 201);
    assert_eq!(StatusCode::BadRequest.as_u16(), 400);
    assert_eq!(StatusCode::NotFound.as_u16(), 404);
    assert_eq!(StatusCode::InternalServerError.as_u16(), 500);

    assert!(StatusCode::Ok.is_success());
    assert!(!StatusCode::BadRequest.is_success());

    assert!(StatusCode::BadRequest.is_client_error());
    assert!(!StatusCode::Ok.is_client_error());

    assert!(StatusCode::InternalServerError.is_server_error());
    assert!(!StatusCode::Ok.is_server_error());
}

#[tokio::test]
async fn test_web_error_status_codes() {
    assert_eq!(WebError::NotFound("test".to_string()).status_code(), 404);
    assert_eq!(
        WebError::MethodNotAllowed("test".to_string()).status_code(),
        405
    );
    assert_eq!(
        WebError::InvalidRequest("test".to_string()).status_code(),
        400
    );
    assert_eq!(WebError::HandlerError("test".to_string()).status_code(), 500);
}

#[tokio::test]
async fn test_router_builder() {
    let router = RouterBuilder::new()
        .get("/", test_handler)
        .post("/users", test_handler)
        .put("/users/:id", test_handler)
        .delete("/users/:id", test_handler)
        .patch("/users/:id", test_handler)
        .build();

    let routes = router.routes();
    assert_eq!(routes.len(), 5);

    // Verify all routes are registered
    assert!(routes.contains(&(RouteMethod::Get, "/".to_string())));
    assert!(routes.contains(&(RouteMethod::Post, "/users".to_string())));
    assert!(routes.contains(&(RouteMethod::Put, "/users/:id".to_string())));
    assert!(routes.contains(&(RouteMethod::Delete, "/users/:id".to_string())));
    assert!(routes.contains(&(RouteMethod::Patch, "/users/:id".to_string())));
}

#[tokio::test]
async fn test_integration_json_api() {
    let router = Router::builder().post("/api/users", json_handler).build();

    let user = TestUser {
        id: 123,
        name: "Bob".to_string(),
    };

    let mut ctx = HandlerContext::new("POST".to_string(), "/api/users".to_string());
    ctx.set_body(serde_json::to_vec(&user).unwrap());

    let response = router.handle(ctx).await.unwrap();
    assert_eq!(response.status, StatusCode::Ok);

    let parsed: TestUser = serde_json::from_slice(&response.body).unwrap();
    assert_eq!(parsed.id, 123);
    assert_eq!(parsed.name, "Bob");
}
