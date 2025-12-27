//! HTTP-specific middleware components.
//!
//! This module provides middleware implementations specifically designed for HTTP request/response processing,
//! including rate limiting with various strategies.

#[cfg(feature = "ratelimit")]
pub mod ratelimit {
    //! Rate limiting middleware for HTTP requests.

    use super::super::http_context::HttpContext;
    use super::super::traits::{Middleware, MiddlewareError, MiddlewareResult, Next};
    use rustboot_ratelimit::{
        FixedWindow, LeakyBucket, RateLimitError, SlidingWindow, TokenBucket,
    };
    use std::collections::HashMap;
    use std::fmt;
    use std::future::Future;
    use std::pin::Pin;
    use std::sync::Arc;
    use std::time::Duration;
    use tokio::sync::RwLock;

    /// Strategy for extracting the rate limit key from a request.
    pub trait KeyExtractor: Send + Sync {
        /// Extract a rate limit key from the HTTP context.
        fn extract_key(&self, ctx: &HttpContext) -> Option<String>;
    }

    /// Extract rate limit key from client IP address.
    #[derive(Clone)]
    pub struct IpKeyExtractor;

    impl KeyExtractor for IpKeyExtractor {
        fn extract_key(&self, ctx: &HttpContext) -> Option<String> {
            ctx.client_ip.clone()
        }
    }

    /// Extract rate limit key from a specific header (e.g., API key).
    #[derive(Clone)]
    pub struct HeaderKeyExtractor {
        header_name: String,
    }

    impl HeaderKeyExtractor {
        /// Create a new header-based key extractor.
        pub fn new(header_name: impl Into<String>) -> Self {
            Self {
                header_name: header_name.into(),
            }
        }
    }

    impl KeyExtractor for HeaderKeyExtractor {
        fn extract_key(&self, ctx: &HttpContext) -> Option<String> {
            ctx.get_header(&self.header_name).cloned()
        }
    }

    /// Extract rate limit key using a custom function.
    pub struct CustomKeyExtractor<F>
    where
        F: Fn(&HttpContext) -> Option<String> + Send + Sync,
    {
        extractor: F,
    }

    impl<F> CustomKeyExtractor<F>
    where
        F: Fn(&HttpContext) -> Option<String> + Send + Sync,
    {
        /// Create a new custom key extractor.
        pub fn new(extractor: F) -> Self {
            Self { extractor }
        }
    }

    impl<F> KeyExtractor for CustomKeyExtractor<F>
    where
        F: Fn(&HttpContext) -> Option<String> + Send + Sync,
    {
        fn extract_key(&self, ctx: &HttpContext) -> Option<String> {
            (self.extractor)(ctx)
        }
    }

    /// Rate limiting algorithm type.
    enum RateLimiter {
        TokenBucket(TokenBucket),
        LeakyBucket(LeakyBucket),
        FixedWindow(FixedWindow),
        SlidingWindow(SlidingWindow),
    }

    impl RateLimiter {
        async fn try_acquire(&self) -> Result<(), RateLimitError> {
            match self {
                RateLimiter::TokenBucket(limiter) => limiter.try_acquire().await,
                RateLimiter::LeakyBucket(limiter) => limiter.try_acquire().await,
                RateLimiter::FixedWindow(limiter) => limiter.try_acquire().await,
                RateLimiter::SlidingWindow(limiter) => limiter.try_acquire().await,
            }
        }

        async fn remaining_capacity(&self) -> usize {
            match self {
                RateLimiter::TokenBucket(limiter) => limiter.available_tokens().await,
                RateLimiter::LeakyBucket(_limiter) => {
                    // Leaky bucket doesn't expose capacity, return 0 as approximation
                    0
                }
                RateLimiter::FixedWindow(_limiter) => {
                    // Fixed window doesn't expose remaining, return 0
                    0
                }
                RateLimiter::SlidingWindow(_limiter) => {
                    // Sliding window doesn't expose remaining, return 0
                    0
                }
            }
        }
    }

    /// Configuration for rate limit algorithm.
    pub enum RateLimitConfig {
        /// Token bucket: (capacity, refill_rate, refill_interval)
        TokenBucket {
            capacity: usize,
            refill_rate: usize,
            refill_interval: Duration,
        },
        /// Leaky bucket: (capacity, leak_rate, leak_interval)
        LeakyBucket {
            capacity: usize,
            leak_rate: usize,
            leak_interval: Duration,
        },
        /// Fixed window: (max_requests, window_size)
        FixedWindow {
            max_requests: usize,
            window_size: Duration,
        },
        /// Sliding window: (max_requests, window_size)
        SlidingWindow {
            max_requests: usize,
            window_size: Duration,
        },
    }

    impl RateLimitConfig {
        fn create_limiter(&self) -> RateLimiter {
            match self {
                RateLimitConfig::TokenBucket {
                    capacity,
                    refill_rate,
                    refill_interval,
                } => RateLimiter::TokenBucket(TokenBucket::new(
                    *capacity,
                    *refill_rate,
                    *refill_interval,
                )),
                RateLimitConfig::LeakyBucket {
                    capacity,
                    leak_rate,
                    leak_interval,
                } => RateLimiter::LeakyBucket(LeakyBucket::new(
                    *capacity,
                    *leak_rate,
                    *leak_interval,
                )),
                RateLimitConfig::FixedWindow {
                    max_requests,
                    window_size,
                } => RateLimiter::FixedWindow(FixedWindow::new(*max_requests, *window_size)),
                RateLimitConfig::SlidingWindow {
                    max_requests,
                    window_size,
                } => RateLimiter::SlidingWindow(SlidingWindow::new(*max_requests, *window_size)),
            }
        }

        fn max_requests(&self) -> usize {
            match self {
                RateLimitConfig::TokenBucket { capacity, .. } => *capacity,
                RateLimitConfig::LeakyBucket { capacity, .. } => *capacity,
                RateLimitConfig::FixedWindow { max_requests, .. } => *max_requests,
                RateLimitConfig::SlidingWindow { max_requests, .. } => *max_requests,
            }
        }
    }

    impl fmt::Debug for RateLimitConfig {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                RateLimitConfig::TokenBucket {
                    capacity,
                    refill_rate,
                    refill_interval,
                } => write!(
                    f,
                    "TokenBucket(capacity={}, refill_rate={}, refill_interval={:?})",
                    capacity, refill_rate, refill_interval
                ),
                RateLimitConfig::LeakyBucket {
                    capacity,
                    leak_rate,
                    leak_interval,
                } => write!(
                    f,
                    "LeakyBucket(capacity={}, leak_rate={}, leak_interval={:?})",
                    capacity, leak_rate, leak_interval
                ),
                RateLimitConfig::FixedWindow {
                    max_requests,
                    window_size,
                } => write!(
                    f,
                    "FixedWindow(max_requests={}, window_size={:?})",
                    max_requests, window_size
                ),
                RateLimitConfig::SlidingWindow {
                    max_requests,
                    window_size,
                } => write!(
                    f,
                    "SlidingWindow(max_requests={}, window_size={:?})",
                    max_requests, window_size
                ),
            }
        }
    }

    /// HTTP rate limiting middleware.
    ///
    /// Enforces rate limits on HTTP requests using configurable algorithms and key extraction strategies.
    /// Adds standard rate limit headers to responses and returns 429 Too Many Requests when limits are exceeded.
    pub struct RateLimitMiddleware {
        config: RateLimitConfig,
        key_extractor: Arc<dyn KeyExtractor>,
        limiters: Arc<RwLock<HashMap<String, RateLimiter>>>,
        include_headers: bool,
    }

    impl RateLimitMiddleware {
        /// Create a new rate limit middleware with IP-based limiting.
        pub fn new(config: RateLimitConfig) -> Self {
            Self {
                config,
                key_extractor: Arc::new(IpKeyExtractor),
                limiters: Arc::new(RwLock::new(HashMap::new())),
                include_headers: true,
            }
        }

        /// Create a new rate limit middleware with custom key extraction.
        pub fn with_key_extractor(
            config: RateLimitConfig,
            key_extractor: Arc<dyn KeyExtractor>,
        ) -> Self {
            Self {
                config,
                key_extractor,
                limiters: Arc::new(RwLock::new(HashMap::new())),
                include_headers: true,
            }
        }

        /// Set whether to include rate limit headers in responses.
        pub fn with_headers(mut self, include_headers: bool) -> Self {
            self.include_headers = include_headers;
            self
        }

        // Note: get_limiter method removed - we now manage limiters inline in the handle method
        // to avoid lifetime and cloning issues
    }

    impl Middleware<HttpContext> for RateLimitMiddleware {
        fn handle(
            &self,
            mut ctx: HttpContext,
            next: Next<HttpContext>,
        ) -> Pin<Box<dyn Future<Output = MiddlewareResult<HttpContext>> + Send>> {
            let config = self.config.clone_config();
            let key_extractor = Arc::clone(&self.key_extractor);
            let limiters = Arc::clone(&self.limiters);
            let include_headers = self.include_headers;
            let max_requests = self.config.max_requests();

            Box::pin(async move {
                // Extract rate limit key
                let key = match key_extractor.extract_key(&ctx) {
                    Some(key) => key,
                    None => {
                        // No key available, reject request
                        tracing::warn!("Rate limit key extraction failed - no key available");
                        ctx.set_response(
                            400,
                            b"Bad Request: Unable to identify client for rate limiting".to_vec(),
                        );
                        return Err(MiddlewareError::Rejected(
                            "Unable to extract rate limit key".to_string(),
                        ));
                    }
                };

                // Get or create limiter for this key
                let mut limiters_map = limiters.write().await;
                let limiter = limiters_map
                    .entry(key.clone())
                    .or_insert_with(|| config.create_limiter());

                // Try to acquire
                match limiter.try_acquire().await {
                    Ok(()) => {
                        // Request allowed
                        tracing::debug!("Rate limit check passed for key: {}", key);

                        // Add rate limit headers if enabled
                        if include_headers {
                            ctx.add_response_header(
                                "X-RateLimit-Limit".to_string(),
                                max_requests.to_string(),
                            );
                            let remaining = limiter.remaining_capacity().await;
                            ctx.add_response_header(
                                "X-RateLimit-Remaining".to_string(),
                                remaining.to_string(),
                            );
                        }

                        drop(limiters_map); // Release lock before calling next
                        next(ctx).await
                    }
                    Err(_) => {
                        // Rate limit exceeded
                        tracing::warn!("Rate limit exceeded for key: {}", key);

                        // Add rate limit headers
                        if include_headers {
                            ctx.add_response_header(
                                "X-RateLimit-Limit".to_string(),
                                max_requests.to_string(),
                            );
                            ctx.add_response_header("X-RateLimit-Remaining".to_string(), "0".to_string());
                            // Add Retry-After header (in seconds) - could be improved with actual calculation
                            ctx.add_response_header("Retry-After".to_string(), "60".to_string());
                        }

                        // Set 429 response
                        ctx.set_response(429, b"Too Many Requests".to_vec());

                        Err(MiddlewareError::Rejected("Rate limit exceeded".to_string()))
                    }
                }
            })
        }
    }

    // Helper trait to clone config
    impl RateLimitConfig {
        fn clone_config(&self) -> Self {
            match self {
                RateLimitConfig::TokenBucket {
                    capacity,
                    refill_rate,
                    refill_interval,
                } => RateLimitConfig::TokenBucket {
                    capacity: *capacity,
                    refill_rate: *refill_rate,
                    refill_interval: *refill_interval,
                },
                RateLimitConfig::LeakyBucket {
                    capacity,
                    leak_rate,
                    leak_interval,
                } => RateLimitConfig::LeakyBucket {
                    capacity: *capacity,
                    leak_rate: *leak_rate,
                    leak_interval: *leak_interval,
                },
                RateLimitConfig::FixedWindow {
                    max_requests,
                    window_size,
                } => RateLimitConfig::FixedWindow {
                    max_requests: *max_requests,
                    window_size: *window_size,
                },
                RateLimitConfig::SlidingWindow {
                    max_requests,
                    window_size,
                } => RateLimitConfig::SlidingWindow {
                    max_requests: *max_requests,
                    window_size: *window_size,
                },
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::chain::Pipeline;

        #[tokio::test]
        async fn test_ip_key_extractor() {
            let extractor = IpKeyExtractor;
            let ctx = HttpContext::new("GET".to_string(), "/test".to_string())
                .with_client_ip("192.168.1.1".to_string());

            let key = extractor.extract_key(&ctx);
            assert_eq!(key, Some("192.168.1.1".to_string()));
        }

        #[tokio::test]
        async fn test_header_key_extractor() {
            let extractor = HeaderKeyExtractor::new("X-API-Key");
            let ctx = HttpContext::new("GET".to_string(), "/test".to_string())
                .with_header("X-API-Key".to_string(), "secret123".to_string());

            let key = extractor.extract_key(&ctx);
            assert_eq!(key, Some("secret123".to_string()));
        }

        #[tokio::test]
        async fn test_custom_key_extractor() {
            let extractor = CustomKeyExtractor::new(|ctx| {
                // Extract user ID from URL path
                ctx.url.split('/').nth(2).map(|s| s.to_string())
            });

            let ctx = HttpContext::new("GET".to_string(), "/users/42/profile".to_string());
            let key = extractor.extract_key(&ctx);
            assert_eq!(key, Some("42".to_string()));
        }

        #[tokio::test]
        async fn test_rate_limit_middleware_allows_requests() {
            let config = RateLimitConfig::FixedWindow {
                max_requests: 5,
                window_size: Duration::from_secs(60),
            };
            let middleware = RateLimitMiddleware::new(config);
            let pipeline = Pipeline::new().with_middleware(middleware);

            // First request should succeed
            let ctx = HttpContext::new("GET".to_string(), "/test".to_string())
                .with_client_ip("192.168.1.1".to_string());

            let result = pipeline.execute(ctx).await;
            assert!(result.is_ok());

            let ctx = result.unwrap();
            // Check rate limit headers were added
            assert!(ctx.response_headers.contains_key("X-RateLimit-Limit"));
            assert_eq!(ctx.response_headers.get("X-RateLimit-Limit"), Some(&"5".to_string()));
        }

        #[tokio::test]
        async fn test_rate_limit_middleware_rejects_excess() {
            let config = RateLimitConfig::FixedWindow {
                max_requests: 2,
                window_size: Duration::from_secs(60),
            };
            let middleware = Arc::new(RateLimitMiddleware::new(config));

            let ip = "192.168.1.2";

            // First 2 requests should succeed
            for _ in 0..2 {
                let ctx = HttpContext::new("GET".to_string(), "/test".to_string())
                    .with_client_ip(ip.to_string());
                let pipeline = Pipeline::new().with_middleware(Arc::clone(&middleware));
                assert!(pipeline.execute(ctx).await.is_ok());
            }

            // Third request should be rejected
            let ctx = HttpContext::new("GET".to_string(), "/test".to_string())
                .with_client_ip(ip.to_string());
            let pipeline = Pipeline::new().with_middleware(Arc::clone(&middleware));
            let result = pipeline.execute(ctx).await;
            assert!(result.is_err());
        }

        #[tokio::test]
        async fn test_rate_limit_middleware_per_key() {
            let config = RateLimitConfig::FixedWindow {
                max_requests: 2,
                window_size: Duration::from_secs(60),
            };
            let middleware = Arc::new(RateLimitMiddleware::new(config));

            // Use up limit for first IP
            for _ in 0..2 {
                let ctx = HttpContext::new("GET".to_string(), "/test".to_string())
                    .with_client_ip("192.168.1.1".to_string());
                let pipeline = Pipeline::new().with_middleware(Arc::clone(&middleware));
                assert!(pipeline.execute(ctx).await.is_ok());
            }

            // Third request from first IP should fail
            let ctx = HttpContext::new("GET".to_string(), "/test".to_string())
                .with_client_ip("192.168.1.1".to_string());
            let pipeline = Pipeline::new().with_middleware(Arc::clone(&middleware));
            assert!(pipeline.execute(ctx).await.is_err());

            // Request from different IP should still work
            let ctx = HttpContext::new("GET".to_string(), "/test".to_string())
                .with_client_ip("192.168.1.2".to_string());
            let pipeline = Pipeline::new().with_middleware(Arc::clone(&middleware));
            assert!(pipeline.execute(ctx).await.is_ok());
        }

        #[tokio::test]
        async fn test_rate_limit_with_api_key() {
            let config = RateLimitConfig::TokenBucket {
                capacity: 3,
                refill_rate: 1,
                refill_interval: Duration::from_secs(10),
            };

            let extractor = Arc::new(HeaderKeyExtractor::new("X-API-Key"));
            let middleware = Arc::new(RateLimitMiddleware::with_key_extractor(config, extractor));

            let api_key = "test-key-123";

            // Use up the token bucket
            for i in 0..3 {
                let ctx = HttpContext::new("GET".to_string(), "/test".to_string())
                    .with_header("X-API-Key".to_string(), api_key.to_string());
                let pipeline = Pipeline::new().with_middleware(Arc::clone(&middleware));
                let result = pipeline.execute(ctx).await;
                assert!(result.is_ok(), "Request {} should succeed", i);
            }

            // Next request should fail
            let ctx = HttpContext::new("GET".to_string(), "/test".to_string())
                .with_header("X-API-Key".to_string(), api_key.to_string());
            let pipeline = Pipeline::new().with_middleware(Arc::clone(&middleware));
            let result = pipeline.execute(ctx).await;
            assert!(result.is_err());

            if let Err(e) = result {
                assert!(matches!(e, MiddlewareError::Rejected(_)));
            }
        }

        #[tokio::test]
        async fn test_rate_limit_no_key_rejects() {
            let config = RateLimitConfig::FixedWindow {
                max_requests: 5,
                window_size: Duration::from_secs(60),
            };
            let middleware = RateLimitMiddleware::new(config);
            let pipeline = Pipeline::new().with_middleware(middleware);

            // Request without client IP should be rejected
            let ctx = HttpContext::new("GET".to_string(), "/test".to_string());
            let result = pipeline.execute(ctx).await;
            assert!(result.is_err());
        }
    }
}
