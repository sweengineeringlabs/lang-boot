//! Handler traits and context for web requests.

use crate::{Response, WebError, WebResult};
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;

/// Context passed to handlers containing request information.
#[derive(Debug, Clone)]
pub struct HandlerContext {
    /// HTTP method.
    pub method: String,
    /// Request path.
    pub path: String,
    /// Path parameters extracted from the route.
    pub params: HashMap<String, String>,
    /// Query parameters.
    pub query: HashMap<String, String>,
    /// Request headers.
    pub headers: HashMap<String, String>,
    /// Request body as bytes.
    pub body: Vec<u8>,
}

impl HandlerContext {
    /// Create a new handler context.
    pub fn new(method: String, path: String) -> Self {
        Self {
            method,
            path,
            params: HashMap::new(),
            query: HashMap::new(),
            headers: HashMap::new(),
            body: Vec::new(),
        }
    }

    /// Get a path parameter by name.
    pub fn param(&self, name: &str) -> Option<&str> {
        self.params.get(name).map(|s| s.as_str())
    }

    /// Get a query parameter by name.
    pub fn query_param(&self, name: &str) -> Option<&str> {
        self.query.get(name).map(|s| s.as_str())
    }

    /// Get a header by name.
    pub fn header(&self, name: &str) -> Option<&str> {
        self.headers.get(name).map(|s| s.as_str())
    }

    /// Parse the body as JSON.
    pub fn json<T: serde::de::DeserializeOwned>(&self) -> WebResult<T> {
        serde_json::from_slice(&self.body).map_err(WebError::from)
    }

    /// Get the body as a string.
    pub fn text(&self) -> WebResult<String> {
        String::from_utf8(self.body.clone())
            .map_err(|e| WebError::invalid_request(format!("Invalid UTF-8: {}", e)))
    }

    /// Set a path parameter.
    pub fn set_param<K: Into<String>, V: Into<String>>(&mut self, key: K, value: V) {
        self.params.insert(key.into(), value.into());
    }

    /// Set a query parameter.
    pub fn set_query<K: Into<String>, V: Into<String>>(&mut self, key: K, value: V) {
        self.query.insert(key.into(), value.into());
    }

    /// Set a header.
    pub fn set_header<K: Into<String>, V: Into<String>>(&mut self, key: K, value: V) {
        self.headers.insert(key.into(), value.into());
    }

    /// Set the body.
    pub fn set_body(&mut self, body: Vec<u8>) {
        self.body = body;
    }
}

/// Trait for request handlers.
#[async_trait]
pub trait Handler: Send + Sync {
    /// Handle a request.
    async fn handle(&self, ctx: HandlerContext) -> WebResult<Response>;
}

/// A boxed handler.
pub type BoxedHandler = Box<dyn Handler>;

/// Implementation of Handler for async functions.
#[async_trait]
impl<F, Fut> Handler for F
where
    F: Fn(HandlerContext) -> Fut + Send + Sync,
    Fut: std::future::Future<Output = WebResult<Response>> + Send,
{
    async fn handle(&self, ctx: HandlerContext) -> WebResult<Response> {
        self(ctx).await
    }
}

/// A handler that can be shared across threads.
pub type SharedHandler = Arc<dyn Handler>;

/// Helper to create a shared handler from a function.
pub fn handler<F, Fut>(f: F) -> SharedHandler
where
    F: Fn(HandlerContext) -> Fut + Send + Sync + 'static,
    Fut: std::future::Future<Output = WebResult<Response>> + Send + 'static,
{
    Arc::new(f)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::StatusCode;

    #[tokio::test]
    async fn test_handler_context() {
        let mut ctx = HandlerContext::new("GET".to_string(), "/test".to_string());
        ctx.set_param("id", "123");
        ctx.set_query("sort", "asc");
        ctx.set_header("content-type", "application/json");

        assert_eq!(ctx.param("id"), Some("123"));
        assert_eq!(ctx.query_param("sort"), Some("asc"));
        assert_eq!(ctx.header("content-type"), Some("application/json"));
    }

    #[tokio::test]
    async fn test_handler_json() {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        struct TestData {
            message: String,
        }

        let data = TestData {
            message: "Hello".to_string(),
        };

        let mut ctx = HandlerContext::new("POST".to_string(), "/test".to_string());
        ctx.set_body(serde_json::to_vec(&data).unwrap());

        let parsed: TestData = ctx.json().unwrap();
        assert_eq!(parsed, data);
    }

    #[tokio::test]
    async fn test_function_handler() {
        let handler = handler(|_ctx: HandlerContext| async {
            Ok(Response::ok().with_text("Hello"))
        });

        let ctx = HandlerContext::new("GET".to_string(), "/".to_string());
        let response = handler.handle(ctx).await.unwrap();

        assert_eq!(response.status, StatusCode::Ok);
        assert_eq!(response.body, b"Hello");
    }
}
