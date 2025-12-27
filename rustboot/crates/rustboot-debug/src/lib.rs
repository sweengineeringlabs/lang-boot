//! Rustboot Debug - Development debugging utilities
//!
//! This crate provides debugging and diagnostic utilities for development.
//! These tools should be disabled in production builds.
//!
//! # Features
//!
//! - `http`: HTTP request/response dumping middleware
//! - `database`: Database query logging utilities
//! - `state-machine`: State machine visualization
//! - `di`: DI container introspection
//! - `config`: Configuration dumping utilities
//! - `all`: Enable all features
//!
//! # Warning
//!
//! These utilities are intended for development and testing only.
//! They may expose sensitive information and have performance overhead.
//! Always disable in production builds.
//!
//! # Examples
//!
//! ## HTTP Debugging
//!
//! ```ignore
//! use dev_engineeringlabs_rustboot_debug::http_debug::HttpDumpMiddleware;
//!
//! let middleware = HttpDumpMiddleware::new()
//!     .with_headers(true)
//!     .with_body(true);
//! ```
//!
//! ## Timing Utilities
//!
//! ```ignore
//! use dev_engineeringlabs_rustboot_debug::timing::TimingScope;
//!
//! async {
//!     let _scope = TimingScope::new("operation");
//!     // Your code here
//! }; // Timing logged on drop
//! ```

pub mod timing;

#[cfg(feature = "http")]
pub mod http_debug;

#[cfg(feature = "database")]
pub mod db_debug;

#[cfg(feature = "state-machine")]
pub mod state_machine_viz;

#[cfg(feature = "di")]
pub mod di_introspect;

#[cfg(feature = "config")]
pub mod config_dump;

// Re-exports
pub use timing::{TimingGuard, TimingScope};

#[cfg(feature = "http")]
pub use http_debug::{HttpDumpConfig, HttpDumpMiddleware};

#[cfg(feature = "database")]
pub use db_debug::{QueryLogger, QueryLoggerConfig};

#[cfg(feature = "state-machine")]
pub use state_machine_viz::StateMachineVisualizer;

#[cfg(feature = "di")]
pub use di_introspect::ContainerIntrospector;

#[cfg(feature = "config")]
pub use config_dump::ConfigDumper;

/// Check if debug mode is enabled (via compile-time flag).
#[inline]
pub fn is_debug_mode() -> bool {
    cfg!(debug_assertions)
}

/// Conditional debug execution - only runs in debug mode.
#[macro_export]
macro_rules! debug_only {
    ($($code:tt)*) => {
        #[cfg(debug_assertions)]
        {
            $($code)*
        }
    };
}

/// Log a debug message with location information.
#[macro_export]
macro_rules! debug_log {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        {
            tracing::debug!(
                target: "rustboot::debug",
                file = file!(),
                line = line!(),
                $($arg)*
            );
        }
    };
}
