//! Middleware chain executor (L4: Core - Middleware).
//!
//! Builds and executes middleware pipelines.

use super::traits::{BoxedMiddleware, Middleware, MiddlewareResult, Next};
use std::sync::Arc;

/// A middleware pipeline that executes middleware in sequence.
pub struct Pipeline<Ctx>
where
    Ctx: Send + 'static,
{
    middleware: Vec<BoxedMiddleware<Ctx>>,
}

impl<Ctx> Pipeline<Ctx>
where
    Ctx: Send + 'static,
{
    /// Create a new empty pipeline.
    pub fn new() -> Self {
        Self {
            middleware: Vec::new(),
        }
    }

    /// Add middleware to the pipeline.
    pub fn with_middleware(mut self, middleware: impl Middleware<Ctx> + 'static) -> Self {
        self.middleware.push(Box::new(middleware));
        self
    }

    /// Execute the pipeline with the given context.
    pub async fn execute(self, ctx: Ctx) -> MiddlewareResult<Ctx> {
        let pipeline = Arc::new(self);
        pipeline.execute_from(ctx, 0).await
    }

    /// Execute from a specific middleware index.
    fn execute_from(self: Arc<Self>, ctx: Ctx, index: usize) -> std::pin::Pin<Box<dyn std::future::Future<Output = MiddlewareResult<Ctx>> + Send>> {
        Box::pin(async move {
            if index >= self.middleware.len() {
                // End of pipeline, return context
                return Ok(ctx);
            }

            let self_clone = Arc::clone(&self);
            let next: Next<Ctx> = Arc::new(move |ctx| {
                let self_inner = Arc::clone(&self_clone);
                Box::pin(async move {
                    self_inner.execute_from(ctx, index + 1).await
                })
            });

            self.middleware[index].handle(ctx, next).await
        })
    }
}

impl<Ctx> Default for Pipeline<Ctx>
where
    Ctx: Send + 'static,
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::FnMiddleware;

    #[derive(Debug, Clone)]
    struct TestContext {
        value: i32,
        log: Vec<String>,
    }

    #[tokio::test]
    async fn test_pipeline_execution() {
        let middleware1 = FnMiddleware::new(|mut ctx: TestContext, next: Next<TestContext>| async move {
            ctx.log.push("middleware1_before".to_string());
            ctx.value += 1;
            let result = next(ctx).await;
            if let Ok(mut ctx) = result {
                ctx.log.push("middleware1_after".to_string());
                Ok(ctx)
            } else {
                result
            }
        });

        let middleware2 = FnMiddleware::new(|mut ctx: TestContext, next: Next<TestContext>| async move {
            ctx.log.push("middleware2_before".to_string());
            ctx.value *= 2;
            let result = next(ctx).await;
            if let Ok(mut ctx) = result {
                ctx.log.push("middleware2_after".to_string());
                Ok(ctx)
            } else {
                result
            }
        });

        let pipeline = Pipeline::new()
            .with_middleware(middleware1)
            .with_middleware(middleware2);

        let ctx = TestContext {
            value: 10,
            log: Vec::new(),
        };

        let result = pipeline.execute(ctx).await;
        assert!(result.is_ok());
        
        let final_ctx = result.unwrap();
        assert_eq!(final_ctx.value, 22); // (10 + 1) * 2
        assert_eq!(final_ctx.log, vec![
            "middleware1_before",
            "middleware2_before",
            "middleware2_after",
            "middleware1_after"
        ]);
    }

    #[tokio::test]
    async fn test_empty_pipeline() {
        let pipeline = Pipeline::<TestContext>::new();
        let ctx = TestContext {
            value: 42,
            log: Vec::new(),
        };

        let result = pipeline.execute(ctx).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().value, 42);
    }
}
