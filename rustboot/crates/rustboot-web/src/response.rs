//! Response types for web handlers.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// HTTP status codes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusCode {
    Ok = 200,
    Created = 201,
    Accepted = 202,
    NoContent = 204,
    BadRequest = 400,
    Unauthorized = 401,
    Forbidden = 403,
    NotFound = 404,
    MethodNotAllowed = 405,
    Conflict = 409,
    InternalServerError = 500,
    NotImplemented = 501,
    BadGateway = 502,
    ServiceUnavailable = 503,
}

impl StatusCode {
    /// Convert to u16.
    pub fn as_u16(&self) -> u16 {
        *self as u16
    }

    /// Check if status is success (2xx).
    pub fn is_success(&self) -> bool {
        let code = self.as_u16();
        (200..300).contains(&code)
    }

    /// Check if status is client error (4xx).
    pub fn is_client_error(&self) -> bool {
        let code = self.as_u16();
        (400..500).contains(&code)
    }

    /// Check if status is server error (5xx).
    pub fn is_server_error(&self) -> bool {
        let code = self.as_u16();
        (500..600).contains(&code)
    }
}

/// HTTP response.
#[derive(Debug, Clone)]
pub struct Response {
    /// HTTP status code.
    pub status: StatusCode,
    /// Response headers.
    pub headers: HashMap<String, String>,
    /// Response body as bytes.
    pub body: Vec<u8>,
}

impl Response {
    /// Create a new response.
    pub fn new(status: StatusCode) -> Self {
        Self {
            status,
            headers: HashMap::new(),
            body: Vec::new(),
        }
    }

    /// Create an OK response.
    pub fn ok() -> Self {
        Self::new(StatusCode::Ok)
    }

    /// Create a Created response.
    pub fn created() -> Self {
        Self::new(StatusCode::Created)
    }

    /// Create a No Content response.
    pub fn no_content() -> Self {
        Self::new(StatusCode::NoContent)
    }

    /// Create a Bad Request response.
    pub fn bad_request() -> Self {
        Self::new(StatusCode::BadRequest)
    }

    /// Create a Not Found response.
    pub fn not_found() -> Self {
        Self::new(StatusCode::NotFound)
    }

    /// Create an Internal Server Error response.
    pub fn internal_error() -> Self {
        Self::new(StatusCode::InternalServerError)
    }

    /// Set the response body as bytes.
    pub fn with_body(mut self, body: Vec<u8>) -> Self {
        self.body = body;
        self
    }

    /// Set the response body as a string.
    pub fn with_text<S: Into<String>>(mut self, text: S) -> Self {
        self.body = text.into().into_bytes();
        self.headers.insert("content-type".to_string(), "text/plain".to_string());
        self
    }

    /// Set the response body as JSON.
    pub fn with_json<T: Serialize>(mut self, data: &T) -> Result<Self, serde_json::Error> {
        self.body = serde_json::to_vec(data)?;
        self.headers.insert("content-type".to_string(), "application/json".to_string());
        Ok(self)
    }

    /// Add a header to the response.
    pub fn with_header<K: Into<String>, V: Into<String>>(mut self, key: K, value: V) -> Self {
        self.headers.insert(key.into(), value.into());
        self
    }

    /// Set multiple headers.
    pub fn with_headers(mut self, headers: HashMap<String, String>) -> Self {
        self.headers.extend(headers);
        self
    }
}

/// Builder for JSON responses.
pub struct JsonResponse;

impl JsonResponse {
    /// Create an OK JSON response.
    pub fn ok<T: Serialize>(data: &T) -> Result<Response, serde_json::Error> {
        Response::ok().with_json(data)
    }

    /// Create a Created JSON response.
    pub fn created<T: Serialize>(data: &T) -> Result<Response, serde_json::Error> {
        Response::created().with_json(data)
    }

    /// Create a Bad Request JSON response.
    pub fn bad_request<T: Serialize>(data: &T) -> Result<Response, serde_json::Error> {
        Response::bad_request().with_json(data)
    }

    /// Create an error JSON response.
    pub fn error<T: Serialize>(status: StatusCode, data: &T) -> Result<Response, serde_json::Error> {
        Response::new(status).with_json(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_code() {
        assert_eq!(StatusCode::Ok.as_u16(), 200);
        assert!(StatusCode::Ok.is_success());
        assert!(!StatusCode::Ok.is_client_error());
        assert!(!StatusCode::Ok.is_server_error());

        assert_eq!(StatusCode::NotFound.as_u16(), 404);
        assert!(!StatusCode::NotFound.is_success());
        assert!(StatusCode::NotFound.is_client_error());
        assert!(!StatusCode::NotFound.is_server_error());

        assert_eq!(StatusCode::InternalServerError.as_u16(), 500);
        assert!(!StatusCode::InternalServerError.is_success());
        assert!(!StatusCode::InternalServerError.is_client_error());
        assert!(StatusCode::InternalServerError.is_server_error());
    }

    #[test]
    fn test_response_builder() {
        let response = Response::ok().with_text("Hello, world!");
        assert_eq!(response.status, StatusCode::Ok);
        assert_eq!(response.body, b"Hello, world!");
        assert_eq!(response.headers.get("content-type"), Some(&"text/plain".to_string()));
    }

    #[test]
    fn test_json_response() {
        #[derive(Serialize, Deserialize)]
        struct TestData {
            message: String,
        }

        let data = TestData {
            message: "Hello".to_string(),
        };

        let response = JsonResponse::ok(&data).unwrap();
        assert_eq!(response.status, StatusCode::Ok);
        assert_eq!(response.headers.get("content-type"), Some(&"application/json".to_string()));

        let parsed: TestData = serde_json::from_slice(&response.body).unwrap();
        assert_eq!(parsed.message, "Hello");
    }
}
