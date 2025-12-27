//! Request extractors for parsing request data.

use crate::{HandlerContext, WebError, WebResult};
use serde::de::DeserializeOwned;
use std::collections::HashMap;

/// JSON extractor for deserializing request bodies.
#[derive(Debug, Clone)]
pub struct Json<T>(pub T);

impl<T: DeserializeOwned> Json<T> {
    /// Extract JSON from the handler context.
    pub fn from_context(ctx: &HandlerContext) -> WebResult<Self> {
        let data = serde_json::from_slice(&ctx.body)?;
        Ok(Json(data))
    }
}

impl<T> std::ops::Deref for Json<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Path parameter extractor.
#[derive(Debug, Clone)]
pub struct Path<T>(pub T);

impl Path<HashMap<String, String>> {
    /// Extract path parameters from the handler context.
    pub fn from_context(ctx: &HandlerContext) -> WebResult<Self> {
        Ok(Path(ctx.params.clone()))
    }
}

impl Path<String> {
    /// Extract a single path parameter by name.
    pub fn from_context_with_name(ctx: &HandlerContext, name: &str) -> WebResult<Self> {
        ctx.params
            .get(name)
            .map(|s| Path(s.clone()))
            .ok_or_else(|| WebError::invalid_request(format!("Missing path parameter: {}", name)))
    }
}

impl<T> std::ops::Deref for Path<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Query parameter extractor.
#[derive(Debug, Clone)]
pub struct Query<T>(pub T);

impl Query<HashMap<String, String>> {
    /// Extract query parameters from the handler context.
    pub fn from_context(ctx: &HandlerContext) -> WebResult<Self> {
        Ok(Query(ctx.query.clone()))
    }
}

impl<T: DeserializeOwned> Query<T> {
    /// Extract and deserialize query parameters from the handler context.
    pub fn from_context_deserialize(ctx: &HandlerContext) -> WebResult<Self> {
        // Convert HashMap<String, String> to a format serde can deserialize
        let json_value = serde_json::to_value(&ctx.query)
            .map_err(|e| WebError::invalid_request(format!("Failed to serialize query: {}", e)))?;

        let data = serde_json::from_value(json_value)
            .map_err(|e| WebError::invalid_request(format!("Failed to deserialize query: {}", e)))?;

        Ok(Query(data))
    }
}

impl<T> std::ops::Deref for Query<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Headers extractor.
#[derive(Debug, Clone)]
pub struct Headers(pub HashMap<String, String>);

impl Headers {
    /// Extract headers from the handler context.
    pub fn from_context(ctx: &HandlerContext) -> WebResult<Self> {
        Ok(Headers(ctx.headers.clone()))
    }

    /// Get a header value by name.
    pub fn get(&self, name: &str) -> Option<&str> {
        self.0.get(name).map(|s| s.as_str())
    }

    /// Get a header value by name, case-insensitive.
    pub fn get_case_insensitive(&self, name: &str) -> Option<&str> {
        let name_lower = name.to_lowercase();
        self.0
            .iter()
            .find(|(k, _)| k.to_lowercase() == name_lower)
            .map(|(_, v)| v.as_str())
    }
}

impl std::ops::Deref for Headers {
    type Target = HashMap<String, String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_extractor() {
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

        let extracted = Json::<TestData>::from_context(&ctx).unwrap();
        assert_eq!(extracted.message, "Hello");
    }

    #[test]
    fn test_path_extractor() {
        let mut ctx = HandlerContext::new("GET".to_string(), "/users/123".to_string());
        ctx.set_param("id", "123");

        let extracted = Path::<String>::from_context_with_name(&ctx, "id").unwrap();
        assert_eq!(*extracted, "123");
    }

    #[test]
    fn test_query_extractor() {
        let mut ctx = HandlerContext::new("GET".to_string(), "/search".to_string());
        ctx.set_query("q", "rust");
        ctx.set_query("limit", "10");

        let extracted = Query::<HashMap<String, String>>::from_context(&ctx).unwrap();
        assert_eq!(extracted.get("q"), Some(&"rust".to_string()));
        assert_eq!(extracted.get("limit"), Some(&"10".to_string()));
    }

    #[test]
    fn test_headers_extractor() {
        let mut ctx = HandlerContext::new("GET".to_string(), "/".to_string());
        ctx.set_header("content-type", "application/json");
        ctx.set_header("authorization", "Bearer token123");

        let headers = Headers::from_context(&ctx).unwrap();
        assert_eq!(headers.get("content-type"), Some("application/json"));
        assert_eq!(headers.get("authorization"), Some("Bearer token123"));
    }

    #[test]
    fn test_headers_case_insensitive() {
        let mut ctx = HandlerContext::new("GET".to_string(), "/".to_string());
        ctx.set_header("Content-Type", "application/json");

        let headers = Headers::from_context(&ctx).unwrap();
        assert_eq!(headers.get_case_insensitive("content-type"), Some("application/json"));
        assert_eq!(headers.get_case_insensitive("CONTENT-TYPE"), Some("application/json"));
    }
}
