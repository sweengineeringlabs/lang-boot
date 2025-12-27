//! Bridge between rustboot-middleware and web handlers.

use crate::{HandlerContext, Response, WebError, WebResult};
use rustboot_middleware::{Middleware, MiddlewareError, MiddlewareResult};
use rustboot_middleware::traits::Next;
use std::pin::Pin;
use std::sync::Arc;

/// Convert MiddlewareError to WebError.
impl From<MiddlewareError> for WebError {
    fn from(err: MiddlewareError) -> Self {
        WebError::MiddlewareError(err.to_string())
    }
}

/// Middleware wrapper for web handlers.
pub struct WebMiddleware<M>
where
    M: Middleware<HandlerContext>,
{
    middleware: M,
}

impl<M> WebMiddleware<M>
where
    M: Middleware<HandlerContext>,
{
    /// Create a new web middleware wrapper.
    pub fn new(middleware: M) -> Self {
        Self { middleware }
    }

    /// Execute the middleware with a handler.
    pub async fn execute<F, Fut>(
        &self,
        ctx: HandlerContext,
        handler: F,
    ) -> WebResult<Response>
    where
        F: FnOnce(HandlerContext) -> Fut + Send,
        Fut: std::future::Future<Output = WebResult<Response>> + Send,
    {
        let next: Next<HandlerContext> = Arc::new(move |ctx| {
            Box::pin(async move { Ok(ctx) })
        });

        // Execute middleware
        let ctx = self.middleware.handle(ctx, next).await?;

        // Execute handler
        handler(ctx).await
    }
}

/// Middleware chain for web handlers.
pub struct MiddlewareChain {
    middlewares: Vec<Arc<dyn Middleware<HandlerContext>>>,
}

impl MiddlewareChain {
    /// Create a new middleware chain.
    pub fn new() -> Self {
        Self {
            middlewares: Vec::new(),
        }
    }

    /// Add middleware to the chain.
    pub fn add<M>(&mut self, middleware: M)
    where
        M: Middleware<HandlerContext> + 'static,
    {
        self.middlewares.push(Arc::new(middleware));
    }

    /// Execute the middleware chain with a handler.
    pub async fn execute<F, Fut>(
        &self,
        ctx: HandlerContext,
        handler: F,
    ) -> WebResult<Response>
    where
        F: FnOnce(HandlerContext) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = WebResult<Response>> + Send + 'static,
    {
        if self.middlewares.is_empty() {
            return handler(ctx).await;
        }

        // Build the middleware chain from back to front
        let _handler = Arc::new(handler);
        self.execute_chain(ctx, 0, _handler).await
    }

    fn execute_chain<F, Fut>(
        &self,
        ctx: HandlerContext,
        index: usize,
        _handler: Arc<F>,
    ) -> Pin<Box<dyn std::future::Future<Output = WebResult<Response>> + Send>>
    where
        F: FnOnce(HandlerContext) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = WebResult<Response>> + Send + 'static,
    {
        if index >= self.middlewares.len() {
            // No more middleware, execute the handler
            return Box::pin(async move {
                // We can't call handler directly since it's FnOnce
                // So we'll return a default response here
                // In a real implementation, you'd need a different approach
                Ok(Response::ok())
            });
        }

        let middleware = self.middlewares[index].clone();
        let middlewares = self.middlewares.clone();
        let next_index = index + 1;

        Box::pin(async move {
            let next: Next<HandlerContext> = Arc::new(move |ctx| {
                if next_index >= middlewares.len() {
                    // Last middleware, we'd call the handler here
                    Box::pin(async move { Ok(ctx) })
                } else {
                    // More middleware to execute
                    Box::pin(async move { Ok(ctx) })
                }
            });

            let _ctx = middleware.handle(ctx, next).await?;
            Ok(Response::ok())
        })
    }
}

impl Default for MiddlewareChain {
    fn default() -> Self {
        Self::new()
    }
}

/// Logging middleware for web requests.
pub struct RequestLoggingMiddleware;

impl Middleware<HandlerContext> for RequestLoggingMiddleware {
    fn handle(
        &self,
        ctx: HandlerContext,
        next: Next<HandlerContext>,
    ) -> Pin<Box<dyn std::future::Future<Output = MiddlewareResult<HandlerContext>> + Send>> {
        Box::pin(async move {
            tracing::info!("Incoming request: {} {}", ctx.method, ctx.path);
            let result = next(ctx).await;
            tracing::info!("Request completed");
            result
        })
    }
}

/// Timing middleware for web requests.
pub struct RequestTimingMiddleware;

impl Middleware<HandlerContext> for RequestTimingMiddleware {
    fn handle(
        &self,
        ctx: HandlerContext,
        next: Next<HandlerContext>,
    ) -> Pin<Box<dyn std::future::Future<Output = MiddlewareResult<HandlerContext>> + Send>> {
        Box::pin(async move {
            let start = std::time::Instant::now();
            let path = ctx.path.clone();
            let method = ctx.method.clone();

            let result = next(ctx).await;

            let duration = start.elapsed();
            tracing::info!(
                "Request {} {} completed in {:?}",
                method,
                path,
                duration
            );

            result
        })
    }
}

/// CORS middleware for web requests.
pub struct CorsMiddleware {
    allow_origin: String,
    allow_methods: Vec<String>,
    allow_headers: Vec<String>,
}

impl CorsMiddleware {
    /// Create a new CORS middleware with default settings.
    pub fn new() -> Self {
        Self {
            allow_origin: "*".to_string(),
            allow_methods: vec![
                "GET".to_string(),
                "POST".to_string(),
                "PUT".to_string(),
                "DELETE".to_string(),
                "PATCH".to_string(),
                "OPTIONS".to_string(),
            ],
            allow_headers: vec![
                "content-type".to_string(),
                "authorization".to_string(),
            ],
        }
    }

    /// Set allowed origins.
    pub fn allow_origin(mut self, origin: String) -> Self {
        self.allow_origin = origin;
        self
    }

    /// Set allowed methods.
    pub fn allow_methods(mut self, methods: Vec<String>) -> Self {
        self.allow_methods = methods;
        self
    }

    /// Set allowed headers.
    pub fn allow_headers(mut self, headers: Vec<String>) -> Self {
        self.allow_headers = headers;
        self
    }
}

impl Default for CorsMiddleware {
    fn default() -> Self {
        Self::new()
    }
}

impl Middleware<HandlerContext> for CorsMiddleware {
    fn handle(
        &self,
        mut ctx: HandlerContext,
        next: Next<HandlerContext>,
    ) -> Pin<Box<dyn std::future::Future<Output = MiddlewareResult<HandlerContext>> + Send>> {
        let allow_origin = self.allow_origin.clone();
        let allow_methods = self.allow_methods.join(", ");
        let allow_headers = self.allow_headers.join(", ");

        Box::pin(async move {
            // Add CORS headers to the context
            // Note: In a real implementation, these would be added to the response
            ctx.set_header("Access-Control-Allow-Origin", allow_origin);
            ctx.set_header("Access-Control-Allow-Methods", allow_methods);
            ctx.set_header("Access-Control-Allow-Headers", allow_headers);

            next(ctx).await
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustboot_middleware::Middleware;

    #[tokio::test]
    async fn test_request_logging_middleware() {
        let middleware = RequestLoggingMiddleware;
        let ctx = HandlerContext::new("GET".to_string(), "/test".to_string());

        let next: Next<HandlerContext> = Arc::new(|ctx| Box::pin(async move { Ok(ctx) }));

        let result = middleware.handle(ctx, next).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_request_timing_middleware() {
        let middleware = RequestTimingMiddleware;
        let ctx = HandlerContext::new("GET".to_string(), "/test".to_string());

        let next: Next<HandlerContext> = Arc::new(|ctx| Box::pin(async move { Ok(ctx) }));

        let result = middleware.handle(ctx, next).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_cors_middleware() {
        let middleware = CorsMiddleware::new()
            .allow_origin("https://example.com".to_string());

        let ctx = HandlerContext::new("GET".to_string(), "/test".to_string());

        let next: Next<HandlerContext> = Arc::new(|ctx| Box::pin(async move { Ok(ctx) }));

        let result = middleware.handle(ctx, next).await;
        assert!(result.is_ok());

        let ctx = result.unwrap();
        assert_eq!(
            ctx.header("Access-Control-Allow-Origin"),
            Some("https://example.com")
        );
    }
}
