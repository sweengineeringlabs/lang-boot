//! HTTP client abstraction (L4: Core - HTTP).
//!
//! Generic HTTP client trait for making requests.

use async_trait::async_trait;
use std::collections::HashMap;

/// HTTP method.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Method {
    /// GET method.
    Get,
    /// POST method.
    Post,
    /// PUT method.
    Put,
    /// DELETE method.
    Delete,
    /// PATCH method.
    Patch,
    /// HEAD method.
    Head,
    /// OPTIONS method.
    Options,
}

/// HTTP request.
#[derive(Debug, Clone, rustboot_macros::Builder)]
pub struct Request {
    /// HTTP method.
    pub method: Method,
    /// Request URL.
    pub url: String,
    /// Request headers.
    pub headers: HashMap<String, String>,
    /// Request body.
    pub body: Option<Vec<u8>>,
}

impl Request {
    /// Create a new HTTP request.
    pub fn new(method: Method, url: String) -> Self {
        Self {
            method,
            url,
            headers: HashMap::new(),
            body: None,
        }
    }

    /// Set JSON body (convenience method).
    pub fn json<T: serde::Serialize>(mut self, data: &T) -> Result<Self, serde_json::Error> {
        let json = serde_json::to_vec(data)?;
        self.body = Some(json);
        self.headers.insert("Content-Type".to_string(), "application/json".to_string());
        Ok(self)
    }

    /// Set request body.
    pub fn body(mut self, body: Vec<u8>) -> Self {
        self.body = Some(body);
        self
    }

    /// Add a  header.
    pub fn header(mut self, key: String, value: String) -> Self {
        self.headers.insert(key, value);
        self
    }
}

/// HTTP response.
#[derive(Debug, Clone)]
pub struct Response {
    /// Status code.
    pub status: u16,
    /// Response headers.
    pub headers: HashMap<String, String>,
    /// Response body.
    pub body: Vec<u8>,
}

impl Response {
    /// Check if response is successful (2xx).
    pub fn is_success(&self) -> bool {
        self.status >= 200 && self.status < 300
    }
    
    /// Get body as string.
    pub fn text(&self) -> Result<String, std::string::FromUtf8Error> {
        String::from_utf8(self.body.clone())
    }
    
    /// Parse body as JSON.
    pub fn json<T: serde::de::DeserializeOwned>(&self) -> Result<T, serde_json::Error> {
        serde_json::from_slice(&self.body)
    }
}

/// HTTP client error.
#[derive(Debug, thiserror::Error)]
pub enum HttpError {
    /// Request error.
    #[error("Request error: {0}")]
    Request(String),
    
    /// Timeout error.
    #[error("Request timeout")]
    Timeout,
    
    /// Connection error.
    #[error("Connection error: {0}")]
    Connection(String),
}

/// Result type for HTTP operations.
pub type HttpResult<T> = Result<T, HttpError>;

/// HTTP client trait.
#[async_trait]
pub trait HttpClient: Send + Sync {
    /// Send a request.
    async fn send(&self, request: Request) -> HttpResult<Response>;
    
    /// Send a GET request.
    async fn get(&self, url: &str) -> HttpResult<Response> {
        self.send(Request::new(Method::Get, url.to_string())).await
    }
    
    /// Send a POST request.
    async fn post(&self, url: &str, body: Vec<u8>) -> HttpResult<Response> {
        self.send(Request::new(Method::Post, url.to_string()).body(body)).await
    }
    
    /// Send a PUT request.
    async fn put(&self, url: &str, body: Vec<u8>) -> HttpResult<Response> {
        self.send(Request::new(Method::Put, url.to_string()).body(body)).await
    }
    
    /// Send a DELETE request.
    async fn delete(&self, url: &str) -> HttpResult<Response> {
        self.send(Request::new(Method::Delete, url.to_string())).await
    }
}
