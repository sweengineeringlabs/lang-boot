//! Reqwest-based HTTP client implementation.
//!
//! This module provides a concrete implementation of the `HttpClient` trait
//! using the `reqwest` library for making HTTP/HTTPS requests.

use crate::client::{HttpClient, HttpError, HttpResult, Method, Request, Response};
use async_trait::async_trait;
use std::collections::HashMap;
use std::time::Duration;

/// HTTP client implementation using reqwest.
///
/// Supports HTTP/HTTPS requests with configurable timeouts and connection pooling.
///
/// # Example
///
/// ```ignore
/// use rustboot_http::{ReqwestClient, HttpClient, Method, Request};
///
/// // Simple usage
/// let client = ReqwestClient::new();
/// let response = client.get("https://httpbin.org/get").await?;
/// println!("Status: {}", response.status);
///
/// // Using builder
/// let client = ReqwestClient::builder()
///     .timeout(Duration::from_secs(60))
///     .user_agent("MyApp/1.0")
///     .default_header("Authorization", "Bearer token")
///     .build()?;
/// ```
#[derive(Clone)]
pub struct ReqwestClient {
    client: reqwest::Client,
}

/// Builder for configuring and creating a `ReqwestClient`.
///
/// # Example
///
/// ```ignore
/// use rustboot_http::ReqwestClientBuilder;
/// use std::time::Duration;
///
/// let client = ReqwestClientBuilder::new()
///     .timeout(Duration::from_secs(60))
///     .connect_timeout(Duration::from_secs(10))
///     .user_agent("MyApp/1.0")
///     .default_header("X-Api-Key", "secret")
///     .max_redirects(5)
///     .build()?;
/// ```
#[derive(Default)]
pub struct ReqwestClientBuilder {
    timeout: Option<Duration>,
    connect_timeout: Option<Duration>,
    user_agent: Option<String>,
    default_headers: HashMap<String, String>,
    max_redirects: Option<usize>,
    danger_accept_invalid_certs: bool,
    pool_idle_timeout: Option<Duration>,
    pool_max_idle_per_host: Option<usize>,
}

impl ReqwestClientBuilder {
    /// Create a new builder with default settings.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the request timeout.
    ///
    /// This is the total time allowed for a request, including connection time.
    /// Default is 30 seconds if not specified.
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Set the connection timeout.
    ///
    /// This is the time allowed to establish a connection.
    pub fn connect_timeout(mut self, timeout: Duration) -> Self {
        self.connect_timeout = Some(timeout);
        self
    }

    /// Set the User-Agent header.
    pub fn user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = Some(user_agent.into());
        self
    }

    /// Add a default header that will be sent with every request.
    pub fn default_header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.default_headers.insert(key.into(), value.into());
        self
    }

    /// Set multiple default headers at once.
    pub fn default_headers(mut self, headers: HashMap<String, String>) -> Self {
        self.default_headers.extend(headers);
        self
    }

    /// Set the maximum number of redirects to follow.
    ///
    /// Set to 0 to disable redirects. Default is 10.
    pub fn max_redirects(mut self, max: usize) -> Self {
        self.max_redirects = Some(max);
        self
    }

    /// Disable redirect following entirely.
    pub fn no_redirects(mut self) -> Self {
        self.max_redirects = Some(0);
        self
    }

    /// Accept invalid SSL certificates.
    ///
    /// **WARNING**: This is dangerous and should only be used for testing.
    pub fn danger_accept_invalid_certs(mut self, accept: bool) -> Self {
        self.danger_accept_invalid_certs = accept;
        self
    }

    /// Set the connection pool idle timeout.
    ///
    /// Connections that remain idle for longer than this will be closed.
    pub fn pool_idle_timeout(mut self, timeout: Duration) -> Self {
        self.pool_idle_timeout = Some(timeout);
        self
    }

    /// Set the maximum number of idle connections per host.
    pub fn pool_max_idle_per_host(mut self, max: usize) -> Self {
        self.pool_max_idle_per_host = Some(max);
        self
    }

    /// Build the `ReqwestClient`.
    ///
    /// Returns an error if the client configuration is invalid.
    pub fn build(self) -> Result<ReqwestClient, HttpError> {
        let mut builder = reqwest::Client::builder();

        // Set timeout (default 30s)
        builder = builder.timeout(self.timeout.unwrap_or(Duration::from_secs(30)));

        // Set connect timeout
        if let Some(timeout) = self.connect_timeout {
            builder = builder.connect_timeout(timeout);
        }

        // Set user agent
        if let Some(user_agent) = self.user_agent {
            builder = builder.user_agent(user_agent);
        }

        // Set default headers
        if !self.default_headers.is_empty() {
            let mut headers = reqwest::header::HeaderMap::new();
            for (key, value) in self.default_headers {
                if let (Ok(name), Ok(val)) = (
                    reqwest::header::HeaderName::try_from(key.as_str()),
                    reqwest::header::HeaderValue::try_from(value.as_str()),
                ) {
                    headers.insert(name, val);
                }
            }
            builder = builder.default_headers(headers);
        }

        // Set redirect policy
        if let Some(max) = self.max_redirects {
            builder = builder.redirect(reqwest::redirect::Policy::limited(max));
        }

        // Set dangerous options
        if self.danger_accept_invalid_certs {
            builder = builder.danger_accept_invalid_certs(true);
        }

        // Set pool settings
        if let Some(timeout) = self.pool_idle_timeout {
            builder = builder.pool_idle_timeout(timeout);
        }
        if let Some(max) = self.pool_max_idle_per_host {
            builder = builder.pool_max_idle_per_host(max);
        }

        let client = builder
            .build()
            .map_err(|e| HttpError::Request(format!("Failed to build client: {}", e)))?;

        Ok(ReqwestClient { client })
    }
}

impl ReqwestClient {
    /// Create a new reqwest client with default settings.
    ///
    /// Default timeout is 30 seconds.
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::builder()
                .timeout(Duration::from_secs(30))
                .build()
                .expect("Failed to build reqwest client"),
        }
    }

    /// Create a new builder for configuring the client.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let client = ReqwestClient::builder()
    ///     .timeout(Duration::from_secs(60))
    ///     .user_agent("MyApp/1.0")
    ///     .build()?;
    /// ```
    pub fn builder() -> ReqwestClientBuilder {
        ReqwestClientBuilder::new()
    }

    /// Create a new reqwest client with custom timeout.
    pub fn with_timeout(timeout: Duration) -> Self {
        Self {
            client: reqwest::Client::builder()
                .timeout(timeout)
                .build()
                .expect("Failed to build reqwest client"),
        }
    }

    /// Create a new reqwest client from an existing reqwest::Client.
    ///
    /// Useful when you need full control over client configuration.
    pub fn from_client(client: reqwest::Client) -> Self {
        Self { client }
    }

    /// Convert our Method enum to reqwest::Method.
    fn convert_method(method: Method) -> reqwest::Method {
        match method {
            Method::Get => reqwest::Method::GET,
            Method::Post => reqwest::Method::POST,
            Method::Put => reqwest::Method::PUT,
            Method::Delete => reqwest::Method::DELETE,
            Method::Patch => reqwest::Method::PATCH,
            Method::Head => reqwest::Method::HEAD,
            Method::Options => reqwest::Method::OPTIONS,
        }
    }
}

impl Default for ReqwestClient {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl HttpClient for ReqwestClient {
    async fn send(&self, request: Request) -> HttpResult<Response> {
        let method = Self::convert_method(request.method);

        let mut req_builder = self.client.request(method, &request.url);

        // Add headers
        for (key, value) in &request.headers {
            req_builder = req_builder.header(key, value);
        }

        // Add body if present
        if let Some(body) = request.body {
            req_builder = req_builder.body(body);
        }

        // Send request
        let response = req_builder.send().await.map_err(|e| {
            if e.is_timeout() {
                HttpError::Timeout
            } else if e.is_connect() {
                HttpError::Connection(e.to_string())
            } else {
                HttpError::Request(e.to_string())
            }
        })?;

        // Extract status
        let status = response.status().as_u16();

        // Extract headers
        let mut headers = HashMap::new();
        for (key, value) in response.headers() {
            if let Ok(v) = value.to_str() {
                headers.insert(key.to_string(), v.to_string());
            }
        }

        // Extract body
        let body = response.bytes().await.map_err(|e| {
            HttpError::Request(format!("Failed to read response body: {}", e))
        })?;

        Ok(Response {
            status,
            headers,
            body: body.to_vec(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_client() {
        let client = ReqwestClient::new();
        assert!(std::mem::size_of_val(&client) > 0);
    }

    #[test]
    fn test_default_client() {
        let client = ReqwestClient::default();
        assert!(std::mem::size_of_val(&client) > 0);
    }

    #[test]
    fn test_with_timeout() {
        let client = ReqwestClient::with_timeout(Duration::from_secs(60));
        assert!(std::mem::size_of_val(&client) > 0);
    }

    #[test]
    fn test_convert_method() {
        assert_eq!(ReqwestClient::convert_method(Method::Get), reqwest::Method::GET);
        assert_eq!(ReqwestClient::convert_method(Method::Post), reqwest::Method::POST);
        assert_eq!(ReqwestClient::convert_method(Method::Put), reqwest::Method::PUT);
        assert_eq!(ReqwestClient::convert_method(Method::Delete), reqwest::Method::DELETE);
        assert_eq!(ReqwestClient::convert_method(Method::Patch), reqwest::Method::PATCH);
        assert_eq!(ReqwestClient::convert_method(Method::Head), reqwest::Method::HEAD);
        assert_eq!(ReqwestClient::convert_method(Method::Options), reqwest::Method::OPTIONS);
    }

    #[test]
    fn test_client_clone() {
        let client1 = ReqwestClient::new();
        let client2 = client1.clone();
        assert!(std::mem::size_of_val(&client2) > 0);
    }

    // Builder tests
    #[test]
    fn test_builder_default() {
        let client = ReqwestClientBuilder::new().build().unwrap();
        assert!(std::mem::size_of_val(&client) > 0);
    }

    #[test]
    fn test_builder_with_timeout() {
        let client = ReqwestClient::builder()
            .timeout(Duration::from_secs(120))
            .build()
            .unwrap();
        assert!(std::mem::size_of_val(&client) > 0);
    }

    #[test]
    fn test_builder_with_connect_timeout() {
        let client = ReqwestClient::builder()
            .connect_timeout(Duration::from_secs(10))
            .build()
            .unwrap();
        assert!(std::mem::size_of_val(&client) > 0);
    }

    #[test]
    fn test_builder_with_user_agent() {
        let client = ReqwestClient::builder()
            .user_agent("TestApp/1.0")
            .build()
            .unwrap();
        assert!(std::mem::size_of_val(&client) > 0);
    }

    #[test]
    fn test_builder_with_default_header() {
        let client = ReqwestClient::builder()
            .default_header("Authorization", "Bearer token123")
            .default_header("X-Api-Key", "secret")
            .build()
            .unwrap();
        assert!(std::mem::size_of_val(&client) > 0);
    }

    #[test]
    fn test_builder_with_default_headers() {
        let mut headers = HashMap::new();
        headers.insert("X-Custom-1".to_string(), "value1".to_string());
        headers.insert("X-Custom-2".to_string(), "value2".to_string());

        let client = ReqwestClient::builder()
            .default_headers(headers)
            .build()
            .unwrap();
        assert!(std::mem::size_of_val(&client) > 0);
    }

    #[test]
    fn test_builder_with_max_redirects() {
        let client = ReqwestClient::builder()
            .max_redirects(5)
            .build()
            .unwrap();
        assert!(std::mem::size_of_val(&client) > 0);
    }

    #[test]
    fn test_builder_no_redirects() {
        let client = ReqwestClient::builder()
            .no_redirects()
            .build()
            .unwrap();
        assert!(std::mem::size_of_val(&client) > 0);
    }

    #[test]
    fn test_builder_pool_settings() {
        let client = ReqwestClient::builder()
            .pool_idle_timeout(Duration::from_secs(60))
            .pool_max_idle_per_host(10)
            .build()
            .unwrap();
        assert!(std::mem::size_of_val(&client) > 0);
    }

    #[test]
    fn test_builder_full_config() {
        let client = ReqwestClient::builder()
            .timeout(Duration::from_secs(60))
            .connect_timeout(Duration::from_secs(10))
            .user_agent("FullConfigApp/2.0")
            .default_header("Accept", "application/json")
            .default_header("X-Request-Id", "test-123")
            .max_redirects(3)
            .pool_idle_timeout(Duration::from_secs(90))
            .pool_max_idle_per_host(5)
            .build()
            .unwrap();
        assert!(std::mem::size_of_val(&client) > 0);
    }

    #[test]
    fn test_builder_chaining() {
        // Test that builder methods can be chained in any order
        let client = ReqwestClient::builder()
            .max_redirects(10)
            .user_agent("ChainTest/1.0")
            .timeout(Duration::from_secs(45))
            .default_header("X-Test", "value")
            .connect_timeout(Duration::from_secs(5))
            .build()
            .unwrap();
        assert!(std::mem::size_of_val(&client) > 0);
    }
}
