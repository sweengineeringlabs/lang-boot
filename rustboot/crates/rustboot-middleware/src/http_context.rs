//! HTTP request/response context for middleware (L4: Core - Middleware).
//!
//! Shared context type used across HTTP middleware implementations.

use std::collections::HashMap;

/// HTTP request/response context for middleware processing.
#[derive(Debug, Clone)]
pub struct HttpContext {
    /// HTTP method (GET, POST, etc.)
    pub method: String,
    /// Request URL/path
    pub url: String,
    /// Request headers
    pub headers: HashMap<String, String>,
    /// Request body
    pub body: Option<Vec<u8>>,
    /// Response status code (set by handler or middleware)
    pub status: Option<u16>,
    /// Response headers (set by handler or middleware)
    pub response_headers: HashMap<String, String>,
    /// Response body (set by handler or middleware)
    pub response_body: Option<Vec<u8>>,
    /// Client IP address (if available)
    pub client_ip: Option<String>,
    /// Whether this is a preflight request (OPTIONS)
    pub is_preflight: bool,
    /// Custom metadata for extensions
    pub metadata: HashMap<String, String>,
}

impl HttpContext {
    /// Create a new HTTP context.
    pub fn new(method: String, url: String) -> Self {
        let is_preflight = method.to_uppercase() == "OPTIONS";
        Self {
            method,
            url,
            headers: HashMap::new(),
            body: None,
            status: None,
            response_headers: HashMap::new(),
            response_body: None,
            client_ip: None,
            is_preflight,
            metadata: HashMap::new(),
        }
    }

    /// Create from method and headers (for backward compatibility with CORS).
    pub fn from_headers(method: String, headers: HashMap<String, String>) -> Self {
        let is_preflight = method.to_uppercase() == "OPTIONS";
        Self {
            method,
            url: String::new(),
            headers,
            body: None,
            status: None,
            response_headers: HashMap::new(),
            response_body: None,
            client_ip: None,
            is_preflight,
            metadata: HashMap::new(),
        }
    }

    /// Add a request header.
    pub fn with_header(mut self, key: String, value: String) -> Self {
        self.headers.insert(key, value);
        self
    }

    /// Add client IP.
    pub fn with_client_ip(mut self, ip: String) -> Self {
        self.client_ip = Some(ip);
        self
    }

    /// Set request body.
    pub fn with_body(mut self, body: Vec<u8>) -> Self {
        self.body = Some(body);
        self
    }

    /// Set URL.
    pub fn with_url(mut self, url: String) -> Self {
        self.url = url;
        self
    }

    /// Get a request header value (case-insensitive).
    pub fn get_header(&self, name: &str) -> Option<&String> {
        let name_lower = name.to_lowercase();
        self.headers
            .iter()
            .find(|(k, _)| k.to_lowercase() == name_lower)
            .map(|(_, v)| v)
    }

    /// Set a response header.
    pub fn set_response_header(&mut self, name: String, value: String) {
        self.response_headers.insert(name, value);
    }

    /// Add a response header (builder pattern).
    pub fn add_response_header(&mut self, key: String, value: String) {
        self.response_headers.insert(key, value);
    }

    /// Set response status and body (helper for middleware that generates responses).
    pub fn set_response(&mut self, status: u16, body: Vec<u8>) {
        self.status = Some(status);
        self.response_body = Some(body);
    }

    /// Check if response has been set.
    pub fn has_response(&self) -> bool {
        self.status.is_some()
    }
}
