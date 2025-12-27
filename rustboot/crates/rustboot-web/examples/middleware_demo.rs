//! Middleware demonstration example showing how to use middleware with routes.

use dev_engineeringlabs_rustboot_web::{
    HandlerContext, RequestLoggingMiddleware, RequestTimingMiddleware, CorsMiddleware,
    Response, Router, WebResult,
};
use rustboot_middleware::{Middleware, MiddlewareError};
use std::future::Future;
use std::pin::Pin;

// Helper type for middleware next functions
type NextFn = std::sync::Arc<
    dyn Fn(HandlerContext) -> Pin<Box<dyn Future<Output = Result<HandlerContext, MiddlewareError>> + Send>>
        + Send
        + Sync,
>;

async fn hello_handler(_ctx: HandlerContext) -> WebResult<Response> {
    // Simulate some work
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    Ok(Response::ok().with_text("Hello, World!"))
}

async fn api_handler(ctx: HandlerContext) -> WebResult<Response> {
    let endpoint = ctx.param("endpoint").unwrap_or("unknown");
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
    Ok(Response::ok().with_text(format!("API endpoint: {}", endpoint)))
}

#[tokio::main]
async fn main() {
    // Initialize tracing to see middleware logs
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    // Create middleware instances
    let logging_middleware = RequestLoggingMiddleware;
    let timing_middleware = RequestTimingMiddleware;
    let cors_middleware = CorsMiddleware::new()
        .allow_origin("https://example.com".to_string())
        .allow_methods(vec!["GET".to_string(), "POST".to_string()])
        .allow_headers(vec!["content-type".to_string(), "authorization".to_string()]);

    println!("Middleware Demo");
    println!("===============\n");

    // Test logging middleware
    println!("Testing RequestLoggingMiddleware:");
    let ctx = HandlerContext::new("GET".to_string(), "/hello".to_string());
    let next: NextFn = std::sync::Arc::new(|ctx| {
        Box::pin(async move { Ok(ctx) }) as Pin<Box<dyn Future<Output = _> + Send>>
    });
    let _ = logging_middleware.handle(ctx, next).await;

    println!("\nTesting RequestTimingMiddleware:");
    let ctx = HandlerContext::new("GET".to_string(), "/api/test".to_string());
    let next: NextFn = std::sync::Arc::new(|ctx| {
        Box::pin(async move {
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            Ok(ctx)
        }) as Pin<Box<dyn Future<Output = _> + Send>>
    });
    let _ = timing_middleware.handle(ctx, next).await;

    println!("\nTesting CorsMiddleware:");
    let ctx = HandlerContext::new("GET".to_string(), "/api/data".to_string());
    let next: NextFn = std::sync::Arc::new(|ctx: HandlerContext| {
        Box::pin(async move { Ok(ctx) }) as Pin<Box<dyn Future<Output = _> + Send>>
    });
    match cors_middleware.handle(ctx, next).await {
        Ok(ctx) => {
            println!("CORS headers added:");
            for (key, value) in &ctx.headers {
                if key.starts_with("Access-Control") {
                    println!("  {}: {}", key, value);
                }
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    // Build a router (middleware integration with router would need to be implemented)
    println!("\nBuilding router with handlers:");
    let router = Router::builder()
        .get("/hello", hello_handler)
        .get("/api/:endpoint", api_handler)
        .build();

    println!("Router created with {} routes:", router.routes().len());
    for (method, path) in router.routes() {
        println!("  {} {}", method.as_str(), path);
    }

    // Test the router
    println!("\nTesting router:");
    let ctx = HandlerContext::new("GET".to_string(), "/hello".to_string());
    match router.handle(ctx).await {
        Ok(response) => {
            println!("Response: {}", String::from_utf8_lossy(&response.body));
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
