//! # Rustboot - Application Framework for Rust
//!
//! Rustboot is a comprehensive application framework providing validation, caching,
//! dependency injection, state machines, and more.
//!
//! ## Quick Start
//!
//! ```rust,ignore
//! use rustboot::prelude::*;
//!
//! // Validation
//! let validator = StringValidationBuilder::new("email")
//!     .not_empty()
//!     .email()
//!     .build();
//!
//! // Caching
//! let cache = InMemoryCache::new();
//! cache.set("key", "value")?;
//!
//! // Dependency Injection
//! let container = Container::new();
//! ```
//!
//! ## Feature Flags
//!
//! Enable additional functionality via Cargo features:
//!
//! ```toml
//! [dependencies]
//! rustboot = { version = "0.1", features = ["full"] }
//! ```
//!
//! Available features:
//! - `full` - Enable all optional crates
//! - `config` - Configuration management
//! - `resilience` - Retry, circuit breaker, timeout patterns
//! - `ratelimit` - Rate limiting (token bucket, sliding window)
//! - `serialization` - Serialization formats
//! - `security` - Authentication, authorization, secrets, auditing
//! - `async` - Async utilities
//! - `fileio` - File I/O utilities
//! - `compress` - Compression utilities
//! - `crypto` - Cryptography primitives
//! - `datetime` - Date/time utilities
//! - `uuid` - UUID generation
//! - `streams` - Streaming data utilities

// ============================================================================
// CORE MODULES (always available)
// ============================================================================

/// Validation framework with fluent builders
pub use dev_engineeringlabs_rustboot_validation as validation;

/// Caching abstractions with TTL support
pub use dev_engineeringlabs_rustboot_cache as cache;

/// Dependency injection container
pub use dev_engineeringlabs_rustboot_di as di;

/// State machine with transition guards
pub use dev_engineeringlabs_rustboot_state_machine as state_machine;

/// HTTP client abstractions
pub use dev_engineeringlabs_rustboot_http as http;

/// Pub/sub messaging patterns
pub use dev_engineeringlabs_rustboot_messaging as messaging;

/// Database abstractions and repository pattern
pub use dev_engineeringlabs_rustboot_database as database;

/// Middleware pipeline pattern
pub use dev_engineeringlabs_rustboot_middleware as middleware;

/// Observability (metrics, logging, tracing)
pub use dev_engineeringlabs_rustboot_observability as observability;

// ============================================================================
// OPTIONAL MODULES (feature-gated)
// ============================================================================

/// Configuration management from multiple sources
#[cfg(feature = "config")]
pub use dev_engineeringlabs_rustboot_config as config;

/// Resilience patterns (retry, circuit breaker, timeout)
#[cfg(feature = "resilience")]
pub use dev_engineeringlabs_rustboot_resilience as resilience;

/// Rate limiting (token bucket, leaky bucket, sliding window)
#[cfg(feature = "ratelimit")]
pub use dev_engineeringlabs_rustboot_ratelimit as ratelimit;

/// Serialization formats
#[cfg(feature = "serialization")]
pub use dev_engineeringlabs_rustboot_serialization as serialization;

/// Security (authentication, authorization, secrets, auditing)
#[cfg(feature = "security")]
pub use dev_engineeringlabs_rustboot_security as security;

/// Async utilities
#[cfg(feature = "async")]
pub use dev_engineeringlabs_rustboot_async as async_utils;

/// File I/O utilities
#[cfg(feature = "fileio")]
pub use dev_engineeringlabs_rustboot_fileio as fileio;

/// Compression utilities
#[cfg(feature = "compress")]
pub use dev_engineeringlabs_rustboot_compress as compress;

/// Cryptography primitives
#[cfg(feature = "crypto")]
pub use dev_engineeringlabs_rustboot_crypto as crypto;

/// Date/time utilities
#[cfg(feature = "datetime")]
pub use dev_engineeringlabs_rustboot_datetime as datetime;

/// UUID generation
#[cfg(feature = "uuid")]
pub use dev_engineeringlabs_rustboot_uuid as uuid;

/// Streaming data utilities
#[cfg(feature = "streams")]
pub use dev_engineeringlabs_rustboot_streams as streams;

// ============================================================================
// PRELUDE - commonly used types
// ============================================================================

/// Convenience prelude with commonly used types
pub mod prelude {
    // ========================================================================
    // Core modules (always available)
    // ========================================================================
    
    // Validation - export key types
    pub use crate::validation::traits::{
        Validate, Validator, ValidationResult, ValidationErrors, ValidationError,
        CompositeValidator, PredicateValidator,
    };
    
    // Cache - export key types  
    pub use crate::cache::{Cache, CacheResult, CacheError};
    
    // DI - export key types
    pub use crate::di::{Container, Injectable};
    
    // State Machine - export key types
    pub use crate::state_machine::{StateMachine, StateMachineResult, StateMachineError};
    
    // HTTP - export key types
    pub use crate::http::{HttpClient, HttpResult, HttpError, Request, Response, Method};
    
    // Messaging - export key types
    pub use crate::messaging::{Publisher, Subscriber, Message, MessagingResult};
    
    // Database - export key types
    pub use crate::database::{Repository, Transaction};
    
    // Middleware - export key types with explicit naming to avoid conflicts
    pub use crate::middleware::{Middleware, Pipeline, MiddlewareResult};
    
    // Observability - export key types
    pub use crate::observability::{metrics, tracing};
    
    // ========================================================================
    // Optional modules (feature-gated)
    // ========================================================================
    
    // Config types
    #[cfg(feature = "config")]
    pub use crate::config::{ConfigLoader, ConfigError, ConfigResult, Mergeable};
    
    // Resilience types
    #[cfg(feature = "resilience")]
    pub use crate::resilience::{
        RetryPolicy, ExponentialBackoff, CircuitBreaker, CircuitBreakerConfig,
        ResilienceError, ResilienceResult,
    };
    
    // Rate limiting types
    #[cfg(feature = "ratelimit")]
    pub use crate::ratelimit::{
        TokenBucket, LeakyBucket, FixedWindow, SlidingWindow,
        RateLimitError, RateLimitResult,
    };
    
    // Security types
    #[cfg(feature = "security")]
    pub use crate::security::{SecurityError, SecurityResult};
    
    // Async utilities
    #[cfg(feature = "async")]
    pub use crate::async_utils::{spawn_task, spawn_blocking_task, TimeoutPool, AsyncError, AsyncResult};
    
    // File I/O types
    #[cfg(feature = "fileio")]
    pub use crate::fileio::{write_atomic, ensure_dir, safe_join, FileIoError, FileIoResult};
    
    // Compression types
    #[cfg(feature = "compress")]
    pub use crate::compress::{gzip_compress, gzip_decompress, CompressionError, CompressionResult};
    
    // Crypto types
    #[cfg(feature = "crypto")]
    pub use crate::crypto::{sha256, hmac_sha256, hash_password, verify_password, CryptoError, CryptoResult};
    
    // DateTime types
    #[cfg(feature = "datetime")]
    pub use crate::datetime::{now, now_millis, now_secs, format_timestamp, parse_timestamp, DateTimeError, DateTimeResult};
    
    // UUID types
    #[cfg(feature = "uuid")]
    pub use crate::uuid::{Uuid, new_v4, new_v7, parse_uuid, UuidError, UuidResult};
    
    // Streams types
    #[cfg(feature = "streams")]
    pub use crate::streams::{EventStream, EventSender, StreamBuilder, create_stream};
}
