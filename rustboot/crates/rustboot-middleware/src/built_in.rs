//! Built-in middleware implementations (L4: Core - Middleware).
//!
//! Common middleware for logging, metrics, and timing.

use super::traits::{Middleware, MiddlewareResult, Next};
use std::pin::Pin;
use std::time::Instant;

/// Middleware that logs request processing.
pub struct LoggingMiddleware {
    name: String,
}

impl LoggingMiddleware {
    /// Create a new logging middleware.
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

impl<Ctx> Middleware<Ctx> for LoggingMiddleware
where
    Ctx: Send + std::fmt::Debug + 'static,
{
    fn handle(
        &self,
        ctx: Ctx,
        next: Next<Ctx>,
    ) -> Pin<Box<dyn std::future::Future<Output = MiddlewareResult<Ctx>> + Send>> {
        let name = self.name.clone();
        Box::pin(async move {
            tracing::debug!("[{}] Processing request: {:?}", name, ctx);
            let result = next(ctx).await;
            match &result {
                Ok(_) => tracing::debug!("[{}] Request successful", name),
                Err(e) => tracing::error!("[{}] Request failed: {}", name, e),
            }
            result
        })
    }
}

/// Middleware that measures execution time.
pub struct TimingMiddleware {
    name: String,
}

impl TimingMiddleware {
    /// Create a new timing middleware.
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

impl<Ctx> Middleware<Ctx> for TimingMiddleware
where
    Ctx: Send + 'static,
{
    fn handle(
        &self,
        ctx: Ctx,
        next: Next<Ctx>,
    ) -> Pin<Box<dyn std::future::Future<Output = MiddlewareResult<Ctx>> + Send>> {
        let name = self.name.clone();
        Box::pin(async move {
            let start = Instant::now();
            let result = next(ctx).await;
            let duration = start.elapsed();
            tracing::info!("[{}] Execution time: {:?}", name, duration);
            result
        })
    }
}

/// Middleware that counts requests.
pub struct MetricsMiddleware {
    name: String,
}

impl MetricsMiddleware {
    /// Create a new metrics middleware.
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

impl<Ctx> Middleware<Ctx> for MetricsMiddleware
where
    Ctx: Send + 'static,
{
    fn handle(
        &self,
        ctx: Ctx,
        next: Next<Ctx>,
    ) -> Pin<Box<dyn std::future::Future<Output = MiddlewareResult<Ctx>> + Send>> {
        let name = self.name.clone();
        Box::pin(async move {
            tracing::trace!("[{}] Request started", name);
            let result = next(ctx).await;
            match &result {
                Ok(_) => tracing::trace!("[{}] Request completed successfully", name),
                Err(_) => tracing::trace!("[{}] Request failed", name),
            }
            result
        })
    }
}

/// Middleware that validates context before processing.
///
/// Note: Currently commented out due to lifetime complexities.
/// Can be re-enabled with proper lifetime bounds.
/*
pub struct ValidationMiddleware<Ctx, F>
where
    F: Fn(&Ctx) -> Result<(), String> + Send + Sync,
{
    validator: F,
    _phantom: std::marker::PhantomData<Ctx>,
}

impl<Ctx, F> ValidationMiddleware<Ctx, F>
where
    F: Fn(&Ctx) -> Result<(), String> + Send + Sync,
{
    /// Create a new validation middleware.
    pub fn new(validator: F) -> Self {
        Self {
            validator,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<Ctx, F> Middleware<Ctx> for ValidationMiddleware<Ctx, F>
where
    Ctx: Send + Sync + 'static,
    F: Fn(&Ctx) -> Result<(), String> + Send + Sync,
{
    fn handle(
        &self,
        ctx: Ctx,
        next: Next<Ctx>,
    ) -> Pin<Box<dyn std::future::Future<Output = MiddlewareResult<Ctx>> + Send>> {
        Box::pin(async move {
            if let Err(e) = (self.validator)(&ctx) {
                return Err(super::traits::MiddlewareError::Rejected(e));
            }
            next(ctx).await
        })
    }
}
*/

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chain::Pipeline;

    #[derive(Debug, Clone)]
    struct TestContext {
        value: i32,
    }

    #[tokio::test]
    async fn test_timing_middleware() {
        let middleware = TimingMiddleware::new("test");
        let pipeline = Pipeline::new().with_middleware(middleware);

        let ctx = TestContext { value: 42 };
        let result = pipeline.execute(ctx).await;

        assert!(result.is_ok());
    }

    /* ValidationMiddleware test commented out since the struct is commented out
    #[tokio::test]
    async fn test_validation_middleware() {
        let middleware = ValidationMiddleware::new(|ctx: &TestContext| {
            if ctx.value > 0 {
                Ok(())
            } else {
                Err("Value must be positive".to_string())
            }
        });
        
        let pipeline = Arc::new(Pipeline::new().add(middleware));
        
        // Valid context
        let ctx = TestContext { value: 42 };
        assert!(pipeline.clone().execute(ctx).await.is_ok());
        
        // Invalid context
        let ctx = TestContext { value: -1 };
        assert!(pipeline.execute(ctx).await.is_err());
    }
    */
}
