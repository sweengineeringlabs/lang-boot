//! # rustboot-error
//!
//! Error handling utilities for Rust applications.
//!
//! This crate provides infrastructure for error handling:
//! - Error conversion utilities
//! - Context extension traits
//! - Error chain helpers
//! - Common error mapping patterns
//! - Retryable error trait for resilience patterns
//! - HTTP status code error mapping
//!
//! ## The Infrastructure Rule
//!
//! Error **types** (enums) belong in domain crates (e.g., Rustratify).
//! Error **utilities** (conversion, context, mapping) belong here.
//!
//! ## Quick Start
//!
//! ```rust,ignore
//! use rustboot_error::{ErrorExt, ResultExt, RetryableError};
//!
//! fn example() -> Result<(), MyError> {
//!     std::fs::read_to_string("config.toml")
//!         .map_err_to(|e| MyError::Io(e.to_string()))?;
//!     Ok(())
//! }
//! ```

use std::fmt::Display;

// ============================================================================
// Retryable Error Trait
// ============================================================================

/// Trait for errors that may be retryable.
///
/// Implement this trait for error types that can indicate whether
/// an operation should be retried, and optionally after how long.
///
/// # Example
///
/// ```rust
/// use rustboot_error::RetryableError;
///
/// #[derive(Debug)]
/// enum ApiError {
///     RateLimited { retry_after_ms: Option<u64> },
///     ServerError(String),
///     InvalidRequest(String),
/// }
///
/// impl RetryableError for ApiError {
///     fn is_retryable(&self) -> bool {
///         matches!(self, ApiError::RateLimited { .. } | ApiError::ServerError(_))
///     }
///
///     fn retry_after_ms(&self) -> Option<u64> {
///         match self {
///             ApiError::RateLimited { retry_after_ms } => *retry_after_ms,
///             ApiError::ServerError(_) => Some(1000), // Default 1s backoff
///             _ => None,
///         }
///     }
/// }
/// ```
pub trait RetryableError {
    /// Returns true if the error indicates the operation can be retried.
    fn is_retryable(&self) -> bool;

    /// Returns the suggested delay before retrying, in milliseconds.
    ///
    /// Returns `None` if no specific delay is suggested (use default backoff).
    fn retry_after_ms(&self) -> Option<u64> {
        None
    }

    /// Returns the suggested delay before retrying, in seconds.
    fn retry_after_secs(&self) -> Option<u64> {
        self.retry_after_ms().map(|ms| ms / 1000)
    }
}

// ============================================================================
// HTTP Status Error Trait
// ============================================================================

/// Common HTTP status categories for error mapping.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HttpStatusCategory {
    /// 400 Bad Request
    BadRequest,
    /// 401 Unauthorized
    Unauthorized,
    /// 403 Forbidden
    Forbidden,
    /// 404 Not Found
    NotFound,
    /// 429 Too Many Requests (rate limited)
    RateLimited,
    /// 408, 504 Timeout
    Timeout,
    /// 500-599 Server Error
    ServerError,
    /// Other status codes
    Other(u16),
}

impl HttpStatusCategory {
    /// Categorize an HTTP status code.
    pub fn from_status(status: u16) -> Self {
        match status {
            400 => Self::BadRequest,
            401 => Self::Unauthorized,
            403 => Self::Forbidden,
            404 => Self::NotFound,
            429 => Self::RateLimited,
            408 | 504 => Self::Timeout,
            500..=599 => Self::ServerError,
            _ => Self::Other(status),
        }
    }

    /// Returns true if this status category typically indicates a retryable error.
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            Self::RateLimited | Self::Timeout | Self::ServerError
        )
    }
}

/// Trait for creating errors from HTTP status codes.
///
/// Implement this trait for error types that need to be created
/// from HTTP responses.
///
/// # Example
///
/// ```rust
/// use rustboot_error::{HttpStatusError, HttpStatusCategory};
///
/// #[derive(Debug)]
/// enum ApiError {
///     RateLimited { retry_after_ms: Option<u64> },
///     Unauthorized(String),
///     ServerError(String),
///     Other { status: u16, message: String },
/// }
///
/// impl HttpStatusError for ApiError {
///     fn from_http_status(status: u16, body: &str, retry_after_ms: Option<u64>) -> Self {
///         match HttpStatusCategory::from_status(status) {
///             HttpStatusCategory::RateLimited => ApiError::RateLimited { retry_after_ms },
///             HttpStatusCategory::Unauthorized => ApiError::Unauthorized(body.to_string()),
///             HttpStatusCategory::ServerError => ApiError::ServerError(body.to_string()),
///             other => ApiError::Other {
///                 status: match other { HttpStatusCategory::Other(s) => s, _ => status },
///                 message: body.to_string(),
///             },
///         }
///     }
/// }
/// ```
pub trait HttpStatusError: Sized {
    /// Create an error from an HTTP status code and response body.
    ///
    /// # Arguments
    ///
    /// * `status` - The HTTP status code
    /// * `body` - The response body (may be empty)
    /// * `retry_after_ms` - Optional retry-after header value in milliseconds
    fn from_http_status(status: u16, body: &str, retry_after_ms: Option<u64>) -> Self;

    /// Create an error from status code only (empty body, no retry header).
    fn from_status(status: u16) -> Self {
        Self::from_http_status(status, "", None)
    }
}

/// Extension trait for adding context to errors.
pub trait ErrorExt: Sized {
    /// Convert error to string representation.
    fn to_error_string(&self) -> String;
}

impl<E: Display> ErrorExt for E {
    fn to_error_string(&self) -> String {
        self.to_string()
    }
}

/// Extension trait for Result types to simplify error conversion.
pub trait ResultExt<T, E> {
    /// Map error using a conversion function.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustboot_error::ResultExt;
    ///
    /// let result: Result<(), std::io::Error> = Err(std::io::Error::other("fail"));
    /// let mapped: Result<(), String> = result.map_err_to(|e| e.to_string());
    /// ```
    fn map_err_to<F, E2>(self, f: F) -> Result<T, E2>
    where
        F: FnOnce(E) -> E2;

    /// Map error to a static string.
    fn map_err_msg<E2>(self, msg: &'static str) -> Result<T, E2>
    where
        E2: From<&'static str>;

    /// Map error to an owned string.
    fn map_err_string<E2>(self, msg: String) -> Result<T, E2>
    where
        E2: From<String>;
}

impl<T, E> ResultExt<T, E> for Result<T, E> {
    fn map_err_to<F, E2>(self, f: F) -> Result<T, E2>
    where
        F: FnOnce(E) -> E2,
    {
        self.map_err(f)
    }

    fn map_err_msg<E2>(self, msg: &'static str) -> Result<T, E2>
    where
        E2: From<&'static str>,
    {
        self.map_err(|_| E2::from(msg))
    }

    fn map_err_string<E2>(self, msg: String) -> Result<T, E2>
    where
        E2: From<String>,
    {
        self.map_err(|_| E2::from(msg))
    }
}

/// Extension trait for Option types to convert to Results with errors.
pub trait OptionExt<T> {
    /// Convert None to an error.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustboot_error::OptionExt;
    ///
    /// let opt: Option<i32> = None;
    /// let result: Result<i32, &str> = opt.ok_or_err("value not found");
    /// assert!(result.is_err());
    /// ```
    fn ok_or_err<E>(self, err: E) -> Result<T, E>;

    /// Convert None to an error using a closure.
    fn ok_or_else_err<E, F>(self, f: F) -> Result<T, E>
    where
        F: FnOnce() -> E;
}

impl<T> OptionExt<T> for Option<T> {
    fn ok_or_err<E>(self, err: E) -> Result<T, E> {
        self.ok_or(err)
    }

    fn ok_or_else_err<E, F>(self, f: F) -> Result<T, E>
    where
        F: FnOnce() -> E,
    {
        self.ok_or_else(f)
    }
}

/// Create an error conversion function for common patterns.
///
/// # Example
///
/// ```rust
/// use rustboot_error::err_to_string;
///
/// let io_err = std::io::Error::other("disk full");
/// let as_string: String = err_to_string(io_err);
/// assert!(as_string.contains("disk full"));
/// ```
pub fn err_to_string<E: Display>(err: E) -> String {
    err.to_string()
}

/// Create an IO error to string converter.
pub fn io_err_to_string(err: std::io::Error) -> String {
    format!("IO error: {}", err)
}

/// Macro for creating From implementations between error types.
///
/// # Example
///
/// ```rust
/// use rustboot_error::impl_error_from;
/// use thiserror::Error;
///
/// #[derive(Debug, Error)]
/// pub enum MyError {
///     #[error("IO error: {0}")]
///     Io(String),
///     #[error("Parse error: {0}")]
///     Parse(String),
/// }
///
/// impl_error_from!(std::io::Error => MyError::Io);
/// ```
#[macro_export]
macro_rules! impl_error_from {
    ($from:ty => $to:ident::$variant:ident) => {
        impl From<$from> for $to {
            fn from(err: $from) -> Self {
                $to::$variant(err.to_string())
            }
        }
    };
}

/// Macro for creating error context wrappers.
///
/// # Example
///
/// ```rust
/// use rustboot_error::with_context;
///
/// fn read_config() -> Result<String, String> {
///     with_context!(
///         std::fs::read_to_string("config.toml"),
///         "failed to read config file"
///     )
/// }
/// ```
#[macro_export]
macro_rules! with_context {
    ($result:expr, $context:expr) => {
        $result.map_err(|e| format!("{}: {}", $context, e))
    };
}

// ============================================================================
// Convenience Macros for Common From Implementations
// ============================================================================

/// Macro for creating From<std::io::Error> implementation.
///
/// # Example
///
/// ```rust
/// use rustboot_error::impl_from_io_error;
/// use thiserror::Error;
///
/// #[derive(Debug, Error)]
/// pub enum MyError {
///     #[error("IO error: {0}")]
///     Io(String),
/// }
///
/// impl_from_io_error!(MyError::Io);
/// ```
#[macro_export]
macro_rules! impl_from_io_error {
    ($to:ident::$variant:ident) => {
        impl From<std::io::Error> for $to {
            fn from(err: std::io::Error) -> Self {
                $to::$variant(err.to_string())
            }
        }
    };
}

/// Macro for creating From<serde_json::Error> implementation.
///
/// Requires serde_json as a dependency.
///
/// # Example
///
/// ```rust,ignore
/// use rustboot_error::impl_from_serde_json_error;
///
/// #[derive(Debug, thiserror::Error)]
/// pub enum MyError {
///     #[error("JSON error: {0}")]
///     Json(String),
/// }
///
/// impl_from_serde_json_error!(MyError::Json);
/// ```
#[macro_export]
macro_rules! impl_from_serde_json_error {
    ($to:ident::$variant:ident) => {
        impl From<serde_json::Error> for $to {
            fn from(err: serde_json::Error) -> Self {
                $to::$variant(err.to_string())
            }
        }
    };
}

/// Macro for creating From<serde_yaml::Error> implementation.
///
/// Requires serde_yaml as a dependency.
///
/// # Example
///
/// ```rust,ignore
/// use rustboot_error::impl_from_serde_yaml_error;
///
/// #[derive(Debug, thiserror::Error)]
/// pub enum MyError {
///     #[error("YAML error: {0}")]
///     Yaml(String),
/// }
///
/// impl_from_serde_yaml_error!(MyError::Yaml);
/// ```
#[macro_export]
macro_rules! impl_from_serde_yaml_error {
    ($to:ident::$variant:ident) => {
        impl From<serde_yaml::Error> for $to {
            fn from(err: serde_yaml::Error) -> Self {
                $to::$variant(err.to_string())
            }
        }
    };
}

/// Macro for implementing multiple From conversions at once.
///
/// # Example
///
/// ```rust
/// use rustboot_error::impl_error_from_many;
/// use thiserror::Error;
///
/// #[derive(Debug, Error)]
/// pub enum MyError {
///     #[error("IO error: {0}")]
///     Io(String),
///     #[error("Parse error: {0}")]
///     Parse(String),
/// }
///
/// impl_error_from_many!(
///     std::io::Error => MyError::Io,
///     std::num::ParseIntError => MyError::Parse
/// );
/// ```
#[macro_export]
macro_rules! impl_error_from_many {
    ($($from:ty => $to:ident::$variant:ident),+ $(,)?) => {
        $(
            $crate::impl_error_from!($from => $to::$variant);
        )+
    };
}

/// Macro for defining a Result type alias for a module.
///
/// # Example
///
/// ```rust
/// use rustboot_error::define_result;
/// use thiserror::Error;
///
/// #[derive(Debug, Error)]
/// pub enum MyError {
///     #[error("Something failed")]
///     Failed,
/// }
///
/// define_result!(MyError);
///
/// fn example() -> Result<()> {
///     Ok(())
/// }
/// ```
#[macro_export]
macro_rules! define_result {
    ($error:ty) => {
        /// Result type alias for this module.
        pub type Result<T> = std::result::Result<T, $error>;
    };
    ($name:ident, $error:ty) => {
        /// Result type alias for this module.
        pub type $name<T> = std::result::Result<T, $error>;
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_result_ext_map_err_to() {
        let result: Result<(), std::io::Error> =
            Err(std::io::Error::other("test error"));
        let mapped: Result<(), String> = result.map_err_to(|e| e.to_string());

        assert!(mapped.is_err());
        assert!(mapped.unwrap_err().contains("test error"));
    }

    #[test]
    fn test_option_ext_ok_or_err() {
        let opt: Option<i32> = None;
        let result: Result<i32, &str> = opt.ok_or_err("missing value");

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "missing value");
    }

    #[test]
    fn test_option_ext_some() {
        let opt: Option<i32> = Some(42);
        let result: Result<i32, &str> = opt.ok_or_err("missing value");

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn test_err_to_string() {
        let io_err = std::io::Error::other("disk full");
        let s = err_to_string(io_err);
        assert!(s.contains("disk full"));
    }

    #[test]
    fn test_with_context_macro() {
        let result: Result<(), std::io::Error> =
            Err(std::io::Error::other("underlying error"));
        let contextualized = with_context!(result, "operation failed");

        assert!(contextualized.is_err());
        let err_msg = contextualized.unwrap_err();
        assert!(err_msg.contains("operation failed"));
        assert!(err_msg.contains("underlying error"));
    }

    // Tests for HttpStatusCategory
    #[test]
    fn test_http_status_category_from_status() {
        assert_eq!(
            HttpStatusCategory::from_status(400),
            HttpStatusCategory::BadRequest
        );
        assert_eq!(
            HttpStatusCategory::from_status(401),
            HttpStatusCategory::Unauthorized
        );
        assert_eq!(
            HttpStatusCategory::from_status(403),
            HttpStatusCategory::Forbidden
        );
        assert_eq!(
            HttpStatusCategory::from_status(404),
            HttpStatusCategory::NotFound
        );
        assert_eq!(
            HttpStatusCategory::from_status(429),
            HttpStatusCategory::RateLimited
        );
        assert_eq!(
            HttpStatusCategory::from_status(408),
            HttpStatusCategory::Timeout
        );
        assert_eq!(
            HttpStatusCategory::from_status(504),
            HttpStatusCategory::Timeout
        );
        assert_eq!(
            HttpStatusCategory::from_status(500),
            HttpStatusCategory::ServerError
        );
        assert_eq!(
            HttpStatusCategory::from_status(503),
            HttpStatusCategory::ServerError
        );
        assert_eq!(
            HttpStatusCategory::from_status(200),
            HttpStatusCategory::Other(200)
        );
    }

    #[test]
    fn test_http_status_category_is_retryable() {
        assert!(HttpStatusCategory::RateLimited.is_retryable());
        assert!(HttpStatusCategory::Timeout.is_retryable());
        assert!(HttpStatusCategory::ServerError.is_retryable());
        assert!(!HttpStatusCategory::BadRequest.is_retryable());
        assert!(!HttpStatusCategory::Unauthorized.is_retryable());
        assert!(!HttpStatusCategory::NotFound.is_retryable());
    }

    // Tests for RetryableError trait
    #[derive(Debug)]
    enum TestError {
        Retryable { retry_after: Option<u64> },
        NotRetryable,
    }

    impl RetryableError for TestError {
        fn is_retryable(&self) -> bool {
            matches!(self, TestError::Retryable { .. })
        }

        fn retry_after_ms(&self) -> Option<u64> {
            match self {
                TestError::Retryable { retry_after } => *retry_after,
                TestError::NotRetryable => None,
            }
        }
    }

    #[test]
    fn test_retryable_error_trait() {
        let retryable = TestError::Retryable {
            retry_after: Some(5000),
        };
        let not_retryable = TestError::NotRetryable;

        assert!(retryable.is_retryable());
        assert!(!not_retryable.is_retryable());
        assert_eq!(retryable.retry_after_ms(), Some(5000));
        assert_eq!(retryable.retry_after_secs(), Some(5));
        assert_eq!(not_retryable.retry_after_ms(), None);
    }

    // Tests for impl_from_io_error macro
    #[derive(Debug, thiserror::Error)]
    enum MacroTestError {
        #[error("IO error: {0}")]
        Io(String),
    }

    impl_from_io_error!(MacroTestError::Io);

    #[test]
    fn test_impl_from_io_error_macro() {
        let io_err = std::io::Error::other("test io error");
        let err: MacroTestError = io_err.into();
        match err {
            MacroTestError::Io(msg) => assert!(msg.contains("test io error")),
        }
    }

    // Tests for define_result macro
    mod result_macro_test {
        use super::*;
        define_result!(MacroTestError);

        #[test]
        fn test_define_result_macro() {
            fn example() -> Result<i32> {
                Ok(42)
            }
            assert_eq!(example().unwrap(), 42);
        }
    }
}
