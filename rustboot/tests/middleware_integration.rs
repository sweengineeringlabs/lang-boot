//! Integration tests for Rustboot middleware

use rustboot::middleware::*;
use async_trait::async_trait;

#[derive(Clone)]
struct TestContext {
    value: String,
}

struct UppercaseMiddleware;

#[async_trait]
impl Middleware<TestContext> for UppercaseMiddleware {
    async fn handle(&self, mut ctx: TestContext) -> MiddlewareResult<TestContext> {
        ctx.value = ctx.value.to_uppercase();
        Ok(ctx)
    }
}

struct ReverseMiddleware;

#[async_trait]
impl Middleware<TestContext> for ReverseMiddleware {
    async fn handle(&self, mut ctx: TestContext) -> MiddlewareResult<TestContext> {
        ctx.value = ctx.value.chars().rev().collect();
        Ok(ctx)
    }
}

#[tokio::test]
async fn test_middleware_pipeline() {
    let pipeline = Pipeline::new()
        .add(Box::new(UppercaseMiddleware))
        .add(Box::new(ReverseMiddleware));
    
    let ctx = TestContext { value: "hello".to_string() };
    let result = pipeline.execute(ctx).await.unwrap();
    
    // "hello" -> "HELLO" -> "OLLEH"
    assert_eq!(result.value, "OLLEH");
}

#[tokio::test]
async fn test_middleware_single() {
    let ctx = TestContext { value: "hello".to_string() };
    let pipeline = Pipeline::new().add(Box::new(UppercaseMiddleware));
    
    let result = pipeline.execute(ctx).await.unwrap();
    assert_eq!(result.value, "HELLO");
}

#[tokio::test]
async fn test_middleware_order_matters() {
    // First reverse, then uppercase
    let pipeline1 = Pipeline::new()
        .add(Box::new(ReverseMiddleware))
        .add(Box::new(UppercaseMiddleware));
    
    let ctx1 = TestContext { value: "hello".to_string() };
    let result1 = pipeline1.execute(ctx1).await.unwrap();
    assert_eq!(result1.value, "OLLEH");
    
    // First uppercase, then reverse
    let pipeline2 = Pipeline::new()
        .add(Box::new(UppercaseMiddleware))
        .add(Box::new(ReverseMiddleware));
    
    let ctx2 = TestContext { value: "hello".to_string() };
    let result2 = pipeline2.execute(ctx2).await.unwrap();
    assert_eq!(result2.value, "OLLEH");
}
