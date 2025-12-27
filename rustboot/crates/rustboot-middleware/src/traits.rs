//! Middleware traits and types (L4: Core - Middleware).
//!
//! Composable middleware pattern for request/response processing.

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

/// Result type for middleware operations.
pub type MiddlewareResult<T> = Result<T, MiddlewareError>;

/// Errors that can occur in middleware.
#[derive(Debug, thiserror::Error)]
pub enum MiddlewareError {
    /// Middleware rejected the request.
    #[error("Request rejected: {0}")]
    Rejected(String),

    /// Middleware encountered an error.
    #[error("Middleware error: {0}")]
    ExecutionError(String),

    /// Custom error.
    #[error("{0}")]
    Custom(String),
}

/// The next middleware in the chain.
pub type Next<Ctx> = Arc<dyn Fn(Ctx) -> Pin<Box<dyn Future<Output = MiddlewareResult<Ctx>> + Send>> + Send + Sync>;

/// Trait for middleware components.
///
/// Middleware can transform, validate, or short-circuit request processing.
pub trait Middleware<Ctx>: Send + Sync
where
    Ctx: Send + 'static,
{
    /// Handle the request with access to the next middleware.
    fn handle(
        &self,
        ctx: Ctx,
        next: Next<Ctx>,
    ) -> Pin<Box<dyn Future<Output = MiddlewareResult<Ctx>> + Send>>;
}

/// A boxed middleware.
pub type BoxedMiddleware<Ctx> = Box<dyn Middleware<Ctx>>;

/// Middleware that always passes through.
pub struct PassthroughMiddleware;

impl<Ctx> Middleware<Ctx> for PassthroughMiddleware
where
    Ctx: Send + 'static,
{
    fn handle(
        &self,
        ctx: Ctx,
        next: Next<Ctx>,
    ) -> Pin<Box<dyn Future<Output = MiddlewareResult<Ctx>> + Send>> {
        Box::pin(async move { next(ctx).await })
    }
}

// Implement Middleware for Arc<T> where T: Middleware
// This allows middleware to be shared across multiple pipeline executions
impl<Ctx, T> Middleware<Ctx> for std::sync::Arc<T>
where
    Ctx: Send + 'static,
    T: Middleware<Ctx> + ?Sized,
{
    fn handle(
        &self,
        ctx: Ctx,
        next: Next<Ctx>,
    ) -> Pin<Box<dyn Future<Output = MiddlewareResult<Ctx>> + Send>> {
        (**self).handle(ctx, next)
    }
}

/// Middleware from a function.
pub struct FnMiddleware<F> {
    func: F,
}

impl<F> FnMiddleware<F> {
    /// Create middleware from a function.
    pub fn new(func: F) -> Self {
        Self { func }
    }
}

impl<Ctx, F, Fut> Middleware<Ctx> for FnMiddleware<F>
where
    Ctx: Send + 'static,
    F: Fn(Ctx, Next<Ctx>) -> Fut + Send + Sync,
    Fut: Future<Output = MiddlewareResult<Ctx>> + Send + 'static,
{
    fn handle(
        &self,
        ctx: Ctx,
        next: Next<Ctx>,
    ) -> Pin<Box<dyn Future<Output = MiddlewareResult<Ctx>> + Send>> {
        Box::pin((self.func)(ctx, next))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone)]
    struct TestContext {
        value: i32,
    }

    #[tokio::test]
    async fn test_passthrough_middleware() {
        let middleware = PassthroughMiddleware;
        let ctx = TestContext { value: 42 };
        
        let next: Next<TestContext> = Arc::new(|ctx| Box::pin(async move { Ok(ctx) }));
        
        let result = middleware.handle(ctx, next).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().value, 42);
    }

    #[tokio::test]
    async fn test_fn_middleware() {
        let middleware = FnMiddleware::new(|mut ctx: TestContext, next: Next<TestContext>| async move {
            ctx.value += 10;
            next(ctx).await
        });
        
        let ctx = TestContext { value: 42 };
        let next: Next<TestContext> = Arc::new(|ctx| Box::pin(async move { Ok(ctx) }));
        
        let result = middleware.handle(ctx, next).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().value, 52);
    }
}
