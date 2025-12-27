//! Basic router example demonstrating core router functionality.

use dev_engineeringlabs_rustboot_web::{
    Handler, HandlerContext, Response, Router, RouterBuilder, StatusCode, WebResult,
};

async fn home_handler(_ctx: HandlerContext) -> WebResult<Response> {
    Ok(Response::ok().with_text("Welcome to Rustboot Web!"))
}

async fn about_handler(_ctx: HandlerContext) -> WebResult<Response> {
    Ok(Response::ok().with_text("This is a Rustboot Web application"))
}

async fn user_handler(ctx: HandlerContext) -> WebResult<Response> {
    let user_id = ctx.param("id").unwrap_or("unknown");
    Ok(Response::ok().with_text(format!("User ID: {}", user_id)))
}

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Build the router
    let router = Router::builder()
        .get("/", home_handler)
        .get("/about", about_handler)
        .get("/users/:id", user_handler)
        .build();

    println!("Router created with {} routes:", router.routes().len());
    for (method, path) in router.routes() {
        println!("  {} {}", method.as_str(), path);
    }

    // Test the router
    let ctx = HandlerContext::new("GET".to_string(), "/".to_string());
    match router.handle(ctx).await {
        Ok(response) => {
            println!("\nHome route test:");
            println!("Status: {}", response.status.as_u16());
            println!("Body: {}", String::from_utf8_lossy(&response.body));
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    // Test user route
    let mut ctx = HandlerContext::new("GET".to_string(), "/users/123".to_string());
    match router.handle(ctx).await {
        Ok(response) => {
            println!("\nUser route test:");
            println!("Status: {}", response.status.as_u16());
            println!("Body: {}", String::from_utf8_lossy(&response.body));
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
