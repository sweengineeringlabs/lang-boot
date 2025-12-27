pub mod cached;
pub mod traced;
pub mod retry;
pub mod timed;
pub mod validate_params;
pub mod circuit_breaker;
pub mod rate_limit;
pub mod audit;
pub mod transactional;
pub mod authorized;
pub mod timeout;
pub mod memoize;
pub mod feature_flag;
pub mod metrics_histogram;
pub mod http_request;
// Note: openapi_path module removed - non-functional placeholder
// Will be re-added when proper implementation is complete
