//! Axum integration for the Rustboot web router.

use crate::{Handler, HandlerContext, Response, Router, WebError, WebResult};
use axum::{
    body::Body,
    extract::{Path as AxumPath, State},
    http::{HeaderMap, Method, StatusCode as AxumStatusCode},
    response::{IntoResponse, Response as AxumResponse},
    routing::{delete, get, patch, post, put},
    Router as AxumRouter,
};
use std::collections::HashMap;
use std::sync::Arc;

/// Convert Rustboot Response to Axum Response.
impl IntoResponse for Response {
    fn into_response(self) -> AxumResponse {
        let mut response = AxumResponse::new(Body::from(self.body));

        // Set status code
        *response.status_mut() = AxumStatusCode::from_u16(self.status.as_u16())
            .unwrap_or(AxumStatusCode::INTERNAL_SERVER_ERROR);

        // Set headers
        let headers = response.headers_mut();
        for (key, value) in self.headers {
            if let Ok(header_name) = axum::http::HeaderName::from_bytes(key.as_bytes()) {
                if let Ok(header_value) = axum::http::HeaderValue::from_str(&value) {
                    headers.insert(header_name, header_value);
                }
            }
        }

        response
    }
}

/// Convert WebError to Axum Response.
impl IntoResponse for WebError {
    fn into_response(self) -> AxumResponse {
        let status = AxumStatusCode::from_u16(self.status_code())
            .unwrap_or(AxumStatusCode::INTERNAL_SERVER_ERROR);

        let body = serde_json::json!({
            "error": self.to_string(),
        });

        let mut response = AxumResponse::new(Body::from(serde_json::to_vec(&body).unwrap()));
        *response.status_mut() = status;

        let headers = response.headers_mut();
        headers.insert(
            axum::http::header::CONTENT_TYPE,
            axum::http::HeaderValue::from_static("application/json"),
        );

        response
    }
}

/// Convert Axum request to HandlerContext.
async fn request_to_context(
    method: Method,
    uri: axum::http::Uri,
    headers: HeaderMap,
    body: Body,
) -> WebResult<HandlerContext> {
    let mut ctx = HandlerContext::new(method.to_string(), uri.path().to_string());

    // Extract query parameters
    if let Some(query) = uri.query() {
        for (key, value) in url::form_urlencoded::parse(query.as_bytes()) {
            ctx.set_query(key.to_string(), value.to_string());
        }
    }

    // Extract headers
    for (key, value) in headers.iter() {
        if let Ok(value_str) = value.to_str() {
            ctx.set_header(key.to_string(), value_str.to_string());
        }
    }

    // Extract body
    let body_bytes = axum::body::to_bytes(body, usize::MAX)
        .await
        .map_err(|e| WebError::invalid_request(format!("Failed to read body: {}", e)))?;
    ctx.set_body(body_bytes.to_vec());

    Ok(ctx)
}

/// Axum handler wrapper for Rustboot handlers.
async fn axum_handler<H>(
    State(handler): State<Arc<H>>,
    method: Method,
    uri: axum::http::Uri,
    headers: HeaderMap,
    body: Body,
) -> Result<AxumResponse, WebError>
where
    H: Handler + 'static,
{
    let ctx = request_to_context(method, uri, headers, body).await?;
    let response = handler.handle(ctx).await?;
    Ok(response.into_response())
}

/// Axum handler wrapper with path parameters.
async fn axum_handler_with_path<H>(
    State(handler): State<Arc<H>>,
    AxumPath(params): AxumPath<HashMap<String, String>>,
    method: Method,
    uri: axum::http::Uri,
    headers: HeaderMap,
    body: Body,
) -> Result<AxumResponse, WebError>
where
    H: Handler + 'static,
{
    let mut ctx = request_to_context(method, uri, headers, body).await?;

    // Add path parameters
    for (key, value) in params {
        ctx.set_param(key, value);
    }

    let response = handler.handle(ctx).await?;
    Ok(response.into_response())
}

/// Extension trait to convert Rustboot Router to Axum Router.
pub trait IntoAxumRouter {
    /// Convert to an Axum router.
    fn into_axum_router(self) -> AxumRouter;
}

impl IntoAxumRouter for Router {
    fn into_axum_router(self) -> AxumRouter {
        let mut axum_router = AxumRouter::new();

        for (method, path) in self.routes() {
            // Find the handler for this route
            // Note: This is a simplified approach. In production, you'd want to store handlers differently.
            let route_key = format!("{} {}", method.as_str(), path);
            tracing::debug!("Registering route: {}", route_key);

            // For now, we'll create a simple handler that returns a message
            // In a real implementation, you'd need to store and retrieve the actual handlers
            let has_params = path.contains(':');

            if has_params {
                // Route has path parameters - use a wildcard route
                let axum_path = convert_path_to_axum(&path);
                let handler = Arc::new(|ctx: HandlerContext| async move {
                    Ok(Response::ok().with_text(format!("Handler for {}", ctx.path)))
                });

                axum_router = match method {
                    crate::RouteMethod::Get => {
                        axum_router.route(&axum_path, get(axum_handler_with_path::<_>).with_state(handler))
                    }
                    crate::RouteMethod::Post => {
                        axum_router.route(&axum_path, post(axum_handler_with_path::<_>).with_state(handler))
                    }
                    crate::RouteMethod::Put => {
                        axum_router.route(&axum_path, put(axum_handler_with_path::<_>).with_state(handler))
                    }
                    crate::RouteMethod::Delete => {
                        axum_router.route(&axum_path, delete(axum_handler_with_path::<_>).with_state(handler))
                    }
                    crate::RouteMethod::Patch => {
                        axum_router.route(&axum_path, patch(axum_handler_with_path::<_>).with_state(handler))
                    }
                    _ => axum_router,
                };
            }
        }

        axum_router
    }
}

/// Convert Rustboot path format to Axum path format.
/// Converts "/users/:id" to "/users/:id" (they use the same format).
fn convert_path_to_axum(path: &str) -> String {
    path.to_string()
}

/// Builder for creating Axum routers with Rustboot handlers.
pub struct AxumRouterBuilder {
    router: AxumRouter,
}

impl AxumRouterBuilder {
    /// Create a new Axum router builder.
    pub fn new() -> Self {
        Self {
            router: AxumRouter::new(),
        }
    }

    /// Add a GET route.
    pub fn get<H>(mut self, path: &str, handler: H) -> Self
    where
        H: Handler + 'static,
    {
        let handler = Arc::new(handler);
        if path.contains(':') {
            self.router = self
                .router
                .route(path, get(axum_handler_with_path::<H>).with_state(handler));
        } else {
            self.router = self.router.route(path, get(axum_handler::<H>).with_state(handler));
        }
        self
    }

    /// Add a POST route.
    pub fn post<H>(mut self, path: &str, handler: H) -> Self
    where
        H: Handler + 'static,
    {
        let handler = Arc::new(handler);
        if path.contains(':') {
            self.router = self
                .router
                .route(path, post(axum_handler_with_path::<H>).with_state(handler));
        } else {
            self.router = self.router.route(path, post(axum_handler::<H>).with_state(handler));
        }
        self
    }

    /// Add a PUT route.
    pub fn put<H>(mut self, path: &str, handler: H) -> Self
    where
        H: Handler + 'static,
    {
        let handler = Arc::new(handler);
        if path.contains(':') {
            self.router = self
                .router
                .route(path, put(axum_handler_with_path::<H>).with_state(handler));
        } else {
            self.router = self.router.route(path, put(axum_handler::<H>).with_state(handler));
        }
        self
    }

    /// Add a DELETE route.
    pub fn delete<H>(mut self, path: &str, handler: H) -> Self
    where
        H: Handler + 'static,
    {
        let handler = Arc::new(handler);
        if path.contains(':') {
            self.router = self
                .router
                .route(path, delete(axum_handler_with_path::<H>).with_state(handler));
        } else {
            self.router = self
                .router
                .route(path, delete(axum_handler::<H>).with_state(handler));
        }
        self
    }

    /// Add a PATCH route.
    pub fn patch<H>(mut self, path: &str, handler: H) -> Self
    where
        H: Handler + 'static,
    {
        let handler = Arc::new(handler);
        if path.contains(':') {
            self.router = self
                .router
                .route(path, patch(axum_handler_with_path::<H>).with_state(handler));
        } else {
            self.router = self
                .router
                .route(path, patch(axum_handler::<H>).with_state(handler));
        }
        self
    }

    /// Merge another Axum router.
    pub fn merge(mut self, router: AxumRouter) -> Self {
        self.router = self.router.merge(router);
        self
    }

    /// Nest routes under a prefix.
    pub fn nest(mut self, prefix: &str, router: AxumRouter) -> Self {
        self.router = self.router.nest(prefix, router);
        self
    }

    /// Build the Axum router.
    pub fn build(self) -> AxumRouter {
        self.router
    }

    /// Build and serve the router on the given address.
    pub async fn serve(self, addr: &str) -> WebResult<()> {
        let listener = tokio::net::TcpListener::bind(addr)
            .await
            .map_err(|e| WebError::ServerError(format!("Failed to bind to {}: {}", addr, e)))?;

        tracing::info!("Server listening on {}", addr);

        axum::serve(listener, self.router)
            .await
            .map_err(|e| WebError::ServerError(format!("Server error: {}", e)))?;

        Ok(())
    }
}

impl Default for AxumRouterBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{HandlerContext, Response};
    use tower::ServiceExt;
    use axum::extract::Request;

    #[tokio::test]
    async fn test_axum_router_builder() {
        let router = AxumRouterBuilder::new()
            .get("/", |_ctx: HandlerContext| async { Ok(Response::ok().with_text("Home")) })
            .get("/users/:id", |ctx: HandlerContext| async move {
                let id = ctx.param("id").unwrap_or("unknown");
                Ok(Response::ok().with_text(format!("User {}", id)))
            })
            .build();

        // Test root route
        let request = Request::builder()
            .uri("/")
            .body(Body::empty())
            .unwrap();

        let response = router.clone().oneshot(request).await.unwrap();
        assert_eq!(response.status(), AxumStatusCode::OK);

        // Test parameterized route
        let request = Request::builder()
            .uri("/users/123")
            .body(Body::empty())
            .unwrap();

        let response = router.oneshot(request).await.unwrap();
        assert_eq!(response.status(), AxumStatusCode::OK);
    }

    #[tokio::test]
    async fn test_response_conversion() {
        let response = Response::ok().with_text("Hello, World!");
        let axum_response: AxumResponse = response.into_response();

        assert_eq!(axum_response.status(), AxumStatusCode::OK);
    }
}
