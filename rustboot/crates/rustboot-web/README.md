# Rustboot Web

Web router integration layer for the Rustboot framework, providing seamless integration with the [axum](https://github.com/tokio-rs/axum) web framework.

## Features

- **Router Abstraction**: Framework-agnostic router with support for multiple HTTP methods
- **Axum Integration**: First-class support for axum web framework (enabled by default)
- **Path Parameters**: Dynamic route matching with named parameters (e.g., `/users/:id`)
- **Request Extractors**: Type-safe extractors for JSON, path params, query params, and headers
- **Response Builders**: Ergonomic response builders with JSON support
- **Middleware Bridge**: Integration with rustboot-middleware for composable request processing
- **Built-in Middleware**: Request logging, timing, and CORS middleware included

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
rustboot-web = "0.1.0"
```

## Quick Start

### Basic Router

```rust
use rustboot_web::{Router, HandlerContext, Response, WebResult};

async fn hello_handler(_ctx: HandlerContext) -> WebResult<Response> {
    Ok(Response::ok().with_text("Hello, World!"))
}

async fn user_handler(ctx: HandlerContext) -> WebResult<Response> {
    let user_id = ctx.param("id").unwrap_or("unknown");
    Ok(Response::ok().with_text(format!("User ID: {}", user_id)))
}

#[tokio::main]
async fn main() {
    let router = Router::builder()
        .get("/", hello_handler)
        .get("/users/:id", user_handler)
        .build();
}
```

### JSON API

```rust
use rustboot_web::{Router, HandlerContext, JsonResponse, Response, WebResult};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct User {
    id: u64,
    name: String,
}

async fn get_user(ctx: HandlerContext) -> WebResult<Response> {
    let user_id = ctx.param("id")
        .and_then(|id| id.parse().ok())
        .unwrap_or(0);

    let user = User {
        id: user_id,
        name: "Alice".to_string(),
    };

    JsonResponse::ok(&user).map_err(Into::into)
}

async fn create_user(ctx: HandlerContext) -> WebResult<Response> {
    let user: User = ctx.json()?;
    // Save user to database...
    JsonResponse::created(&user).map_err(Into::into)
}

#[tokio::main]
async fn main() {
    let router = Router::builder()
        .get("/api/users/:id", get_user)
        .post("/api/users", create_user)
        .build();
}
```

### Axum Server

```rust
use rustboot_web::{AxumRouterBuilder, HandlerContext, Response, WebResult};

async fn handler(_ctx: HandlerContext) -> WebResult<Response> {
    Ok(Response::ok().with_text("Hello from Axum!"))
}

#[tokio::main]
async fn main() -> WebResult<()> {
    AxumRouterBuilder::new()
        .get("/", handler)
        .serve("127.0.0.1:3000")
        .await
}
```

### Middleware

```rust
use rustboot_web::{
    Router, HandlerContext, Response, WebResult,
    RequestLoggingMiddleware, RequestTimingMiddleware, CorsMiddleware,
};

#[tokio::main]
async fn main() {
    let router = Router::builder()
        .get("/", |_ctx| async { Ok(Response::ok().with_text("Hello")) })
        .middleware(RequestLoggingMiddleware)
        .middleware(RequestTimingMiddleware)
        .middleware(CorsMiddleware::new())
        .build();
}
```

### Request Extractors

```rust
use rustboot_web::{HandlerContext, Json, Path, Query, Headers, Response, WebResult};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
struct CreateUser {
    name: String,
    email: String,
}

async fn handler(ctx: HandlerContext) -> WebResult<Response> {
    // Extract JSON body
    let user_data: CreateUser = ctx.json()?;

    // Extract path parameters
    let user_id = ctx.param("id").unwrap_or("unknown");

    // Extract query parameters
    let query = Query::<HashMap<String, String>>::from_context(&ctx)?;
    let search = query.get("q");

    // Extract headers
    let headers = Headers::from_context(&ctx)?;
    let auth = headers.get("authorization");

    Ok(Response::ok().with_text("Processed"))
}
```

## Available HTTP Methods

- `GET` - `router.get(path, handler)`
- `POST` - `router.post(path, handler)`
- `PUT` - `router.put(path, handler)`
- `DELETE` - `router.delete(path, handler)`
- `PATCH` - `router.patch(path, handler)`
- `HEAD` - `router.route(RouteMethod::Head, path, handler)`
- `OPTIONS` - `router.route(RouteMethod::Options, path, handler)`

## Response Helpers

```rust
// Text responses
Response::ok().with_text("Hello")
Response::not_found().with_text("Not found")

// JSON responses
Response::ok().with_json(&data)?
JsonResponse::ok(&data)?
JsonResponse::created(&data)?
JsonResponse::bad_request(&error)?

// Custom responses
Response::new(StatusCode::Ok)
    .with_body(vec![1, 2, 3])
    .with_header("content-type", "application/octet-stream")
```

## Middleware

Built-in middleware:

- **RequestLoggingMiddleware**: Logs incoming requests
- **RequestTimingMiddleware**: Measures request duration
- **CorsMiddleware**: Adds CORS headers

Integration with rustboot-middleware:

```rust
use rustboot_middleware::Middleware;

struct CustomMiddleware;

impl Middleware<HandlerContext> for CustomMiddleware {
    fn handle(&self, ctx: HandlerContext, next: Next<HandlerContext>)
        -> Pin<Box<dyn Future<Output = MiddlewareResult<HandlerContext>> + Send>>
    {
        Box::pin(async move {
            // Pre-processing
            let result = next(ctx).await;
            // Post-processing
            result
        })
    }
}
```

## Examples

See the `examples/` directory for complete working examples:

- `basic_router.rs` - Basic routing with path parameters
- `json_api.rs` - JSON API with CRUD operations
- `axum_server.rs` - Full axum web server
- `middleware_demo.rs` - Middleware usage examples
- `extractors_demo.rs` - Request extractor examples

Run examples:

```bash
cargo run --example basic_router
cargo run --example json_api
cargo run --example axum_server
cargo run --example middleware_demo
cargo run --example extractors_demo
```

## Feature Flags

- `axum` (default) - Enables axum integration

```toml
# Default features (includes axum)
rustboot-web = "0.1.0"

# No features
rustboot-web = { version = "0.1.0", default-features = false }
```

## Testing

Run tests:

```bash
cargo test
cargo test --all-features
```

## Architecture

The crate is organized into several modules:

- `router` - Core routing logic and route matching
- `handler` - Handler traits and context
- `response` - Response types and builders
- `error` - Error types and handling
- `extractors` - Request data extractors
- `middleware_bridge` - Integration with rustboot-middleware
- `axum_integration` - Axum framework integration (feature-gated)

## License

MIT
