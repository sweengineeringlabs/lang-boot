//! Comprehensive Resilience Example
//!
//! This example demonstrates all resilience patterns available in rustboot-resilience:
//! - Circuit breaker for fault tolerance
//! - Retry with exponential backoff
//! - Timeout wrapping
//! - Fallback mechanisms
//! - Combining multiple patterns
//!
//! Simulates real-world scenarios like external API calls and database operations.

use dev_engineeringlabs_rustboot_resilience::{
    CircuitBreaker, CircuitBreakerConfig, CircuitState, ExponentialBackoff, ResilienceError,
    ResilienceResult, RetryPolicy, with_timeout,
};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

// Simulated external service errors
#[derive(Debug)]
enum ServiceError {
    Timeout,
    ServerError,
    RateLimited,
    NetworkError,
}

impl std::fmt::Display for ServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServiceError::Timeout => write!(f, "Service timeout"),
            ServiceError::ServerError => write!(f, "Server error (500)"),
            ServiceError::RateLimited => write!(f, "Rate limited (429)"),
            ServiceError::NetworkError => write!(f, "Network connection failed"),
        }
    }
}

// Simulated external API client
struct ExternalApiClient {
    fail_count: Arc<AtomicUsize>,
    should_fail: Arc<AtomicBool>,
}

impl ExternalApiClient {
    fn new() -> Self {
        Self {
            fail_count: Arc::new(AtomicUsize::new(0)),
            should_fail: Arc::new(AtomicBool::new(true)),
        }
    }

    async fn fetch_user(&self, user_id: u64) -> Result<String, ServiceError> {
        // Simulate network delay
        sleep(Duration::from_millis(50)).await;

        let count = self.fail_count.fetch_add(1, Ordering::SeqCst);

        // Fail first 2 attempts, then succeed
        if self.should_fail.load(Ordering::SeqCst) && count < 2 {
            Err(ServiceError::ServerError)
        } else {
            Ok(format!("User(id={}, name=Alice)", user_id))
        }
    }

    async fn slow_operation(&self) -> Result<String, ServiceError> {
        // Simulate a slow operation
        sleep(Duration::from_millis(200)).await;
        Ok("Slow operation completed".to_string())
    }

    async fn unstable_operation(&self) -> Result<i32, ServiceError> {
        let count = self.fail_count.load(Ordering::SeqCst);

        // Alternate between success and failure
        if count % 2 == 0 {
            Ok(42)
        } else {
            Err(ServiceError::NetworkError)
        }
    }
}

// Simulated database client
#[derive(Clone)]
struct DatabaseClient {
    connection_available: Arc<AtomicBool>,
    query_count: Arc<AtomicUsize>,
}

impl DatabaseClient {
    fn new() -> Self {
        Self {
            connection_available: Arc::new(AtomicBool::new(false)),
            query_count: Arc::new(AtomicUsize::new(0)),
        }
    }

    async fn query_user(&self, user_id: u64) -> Result<String, ServiceError> {
        sleep(Duration::from_millis(30)).await;

        let count = self.query_count.fetch_add(1, Ordering::SeqCst);

        // First 3 queries fail, then succeed (simulating connection recovery)
        if !self.connection_available.load(Ordering::SeqCst) && count < 3 {
            Err(ServiceError::NetworkError)
        } else {
            self.connection_available.store(true, Ordering::SeqCst);
            Ok(format!("DB_User(id={}, email=alice@example.com)", user_id))
        }
    }
}

/// Example 1: Basic Retry with Exponential Backoff
async fn example_retry_with_backoff() {
    println!("\n=== Example 1: Retry with Exponential Backoff ===");

    let client = ExternalApiClient::new();

    // Configure retry policy with custom backoff
    let backoff = ExponentialBackoff::new(
        Duration::from_millis(50),  // Initial delay
        Duration::from_secs(2),      // Max delay
        2.0,                         // Multiplier
    );

    let retry_policy = RetryPolicy::new(5).with_backoff(backoff);

    println!("Attempting to fetch user with retry...");
    let start = std::time::Instant::now();

    let result = retry_policy
        .execute(|| async {
            println!("  - Attempt #{}", client.fail_count.load(Ordering::SeqCst) + 1);
            client.fetch_user(123).await
        })
        .await;

    match result {
        Ok(user) => {
            println!("Success after {:?}: {}", start.elapsed(), user);
        }
        Err(e) => {
            println!("Failed after {:?}: {}", start.elapsed(), e);
        }
    }
}

/// Example 2: Circuit Breaker Pattern
async fn example_circuit_breaker() {
    println!("\n=== Example 2: Circuit Breaker Pattern ===");

    let config = CircuitBreakerConfig {
        failure_threshold: 3,
        timeout: Duration::from_millis(500),
        success_threshold: 2,
    };

    let circuit_breaker = CircuitBreaker::new(config);
    let client = ExternalApiClient::new();

    println!("Initial state: {:?}", circuit_breaker.state().await);

    // Trigger failures to open the circuit
    println!("\nTriggering failures to open circuit...");
    for i in 1..=4 {
        let result = circuit_breaker
            .execute(|| async { client.unstable_operation().await })
            .await;

        println!("  Attempt {}: {:?}, State: {:?}",
                 i, result.is_ok(), circuit_breaker.state().await);
    }

    // Circuit should be open now
    println!("\nCircuit is now: {:?}", circuit_breaker.state().await);

    // Immediate request should fail fast
    println!("\nAttempting request while circuit is open...");
    let result = circuit_breaker
        .execute(|| async { client.unstable_operation().await })
        .await;

    match result {
        Err(ResilienceError::CircuitOpen) => {
            println!("Request failed fast (circuit is open)");
        }
        _ => println!("Unexpected result: {:?}", result),
    }

    // Wait for timeout to allow half-open state
    println!("\nWaiting for circuit timeout...");
    sleep(Duration::from_millis(600)).await;

    // Reset client to allow successes
    client.should_fail.store(false, Ordering::SeqCst);
    client.fail_count.store(0, Ordering::SeqCst);

    // Attempt recovery
    println!("\nAttempting recovery (half-open state)...");
    for i in 1..=3 {
        let result = circuit_breaker
            .execute(|| async {
                Ok::<_, ServiceError>(format!("Success {}", i))
            })
            .await;

        println!("  Recovery attempt {}: {:?}, State: {:?}",
                 i, result.is_ok(), circuit_breaker.state().await);
    }

    println!("\nFinal state: {:?}", circuit_breaker.state().await);
}

/// Example 3: Timeout Pattern
async fn example_timeout() {
    println!("\n=== Example 3: Timeout Pattern ===");

    let client = ExternalApiClient::new();

    // Operation that completes within timeout
    println!("\nFast operation (should succeed):");
    let result = with_timeout(Duration::from_millis(100), async {
        sleep(Duration::from_millis(30)).await;
        "Quick response".to_string()
    })
    .await;

    match result {
        Ok(response) => println!("  Success: {}", response),
        Err(e) => println!("  Error: {}", e),
    }

    // Operation that exceeds timeout
    println!("\nSlow operation (should timeout):");
    let result = with_timeout(Duration::from_millis(100), async {
        client.slow_operation().await.map_err(|e| e.to_string())
    })
    .await;

    match result {
        Ok(Ok(response)) => println!("  Success: {}", response),
        Ok(Err(e)) => println!("  Service error: {}", e),
        Err(ResilienceError::Timeout(duration)) => {
            println!("  Timed out after {:?}", duration);
        }
        Err(e) => println!("  Error: {}", e),
    }
}

/// Example 4: Fallback Mechanism
async fn example_fallback() {
    println!("\n=== Example 4: Fallback Mechanism ===");

    let db_client = DatabaseClient::new();
    let api_client = ExternalApiClient::new();

    let user_id = 456;

    // Try primary source (database)
    println!("\nTrying primary source (database)...");
    let result = db_client.query_user(user_id).await;

    let user_data = match result {
        Ok(data) => {
            println!("  Primary source succeeded: {}", data);
            data
        }
        Err(e) => {
            println!("  Primary source failed: {}", e);
            println!("  Falling back to API...");

            // Fallback to API
            match api_client.fetch_user(user_id).await {
                Ok(data) => {
                    println!("  Fallback succeeded: {}", data);
                    data
                }
                Err(e) => {
                    println!("  Fallback failed: {}", e);
                    println!("  Using cached/default data");
                    format!("CachedUser(id={})", user_id)
                }
            }
        }
    };

    println!("\nFinal result: {}", user_data);
}

/// Example 5: Combining Multiple Patterns
async fn example_combined_patterns() {
    println!("\n=== Example 5: Combining Multiple Patterns ===");

    let db_client = DatabaseClient::new();

    // Configure circuit breaker
    let circuit_config = CircuitBreakerConfig {
        failure_threshold: 2,
        timeout: Duration::from_secs(1),
        success_threshold: 1,
    };
    let circuit_breaker = Arc::new(CircuitBreaker::new(circuit_config));

    // Configure retry policy
    let backoff = ExponentialBackoff::new(
        Duration::from_millis(100),
        Duration::from_secs(1),
        2.0,
    );
    let retry_policy = RetryPolicy::new(3).with_backoff(backoff);

    println!("Fetching user with combined resilience patterns:");
    println!("  - Circuit breaker protection");
    println!("  - Retry with exponential backoff");
    println!("  - Timeout per attempt");

    let cb = circuit_breaker.clone();
    let db_client_clone = db_client.clone();
    let result = retry_policy
        .execute(|| async {
            // Each retry attempt goes through circuit breaker and timeout
            let cb_inner = cb.clone();
            let db_clone = db_client_clone.clone();

            // Wrap in timeout
            with_timeout(Duration::from_millis(200), async move {
                // Wrap in circuit breaker
                cb_inner
                    .execute(|| async { db_clone.query_user(789).await })
                    .await
            })
            .await
            .and_then(|r| r) // Flatten nested Result
        })
        .await;

    match result {
        Ok(user) => {
            println!("\nSuccess: {}", user);
            println!("Circuit state: {:?}", circuit_breaker.state().await);
        }
        Err(e) => {
            println!("\nFailed: {}", e);
            println!("Circuit state: {:?}", circuit_breaker.state().await);
        }
    }
}

/// Example 6: Realistic API Client with Resilience
async fn example_realistic_api_client() {
    println!("\n=== Example 6: Realistic API Client ===");

    struct ResilientApiClient {
        circuit_breaker: Arc<CircuitBreaker>,
        retry_policy: RetryPolicy,
    }

    impl ResilientApiClient {
        fn new() -> Self {
            let circuit_config = CircuitBreakerConfig {
                failure_threshold: 5,
                timeout: Duration::from_secs(30),
                success_threshold: 2,
            };

            let backoff = ExponentialBackoff::new(
                Duration::from_millis(100),
                Duration::from_secs(5),
                2.0,
            );

            Self {
                circuit_breaker: Arc::new(CircuitBreaker::new(circuit_config)),
                retry_policy: RetryPolicy::new(3).with_backoff(backoff),
            }
        }

        async fn get_user(&self, user_id: u64) -> ResilienceResult<String> {
            let cb = self.circuit_breaker.clone();

            self.retry_policy
                .execute(|| async {
                    let cb_inner = cb.clone();

                    with_timeout(Duration::from_secs(2), async move {
                        cb_inner
                            .execute(|| async {
                                // Simulate API call
                                sleep(Duration::from_millis(100)).await;
                                Ok::<_, ServiceError>(format!("User#{}", user_id))
                            })
                            .await
                    })
                    .await
                    .and_then(|r| r)
                })
                .await
        }

        async fn get_user_with_fallback(&self, user_id: u64) -> String {
            match self.get_user(user_id).await {
                Ok(user) => user,
                Err(e) => {
                    eprintln!("API failed: {}, using default", e);
                    format!("DefaultUser#{}", user_id)
                }
            }
        }
    }

    let client = ResilientApiClient::new();

    // Successful API calls
    println!("\nMaking resilient API calls...");
    for i in 1..=3 {
        let user = client.get_user_with_fallback(i).await;
        println!("  Request {}: {}", i, user);
        sleep(Duration::from_millis(50)).await;
    }

    println!("\nCircuit breaker state: {:?}", client.circuit_breaker.state().await);
}

/// Example 7: Bulkhead Pattern (Resource Isolation)
async fn example_bulkhead_pattern() {
    println!("\n=== Example 7: Bulkhead Pattern (Resource Isolation) ===");

    use tokio::sync::Semaphore;

    // Limit concurrent operations to 3
    let semaphore = Arc::new(Semaphore::new(3));

    println!("Simulating concurrent operations with resource limits...");
    println!("  Max concurrent operations: 3");

    let mut handles = vec![];

    for i in 1..=10 {
        let sem = semaphore.clone();

        let handle = tokio::spawn(async move {
            // Acquire permit (blocks if limit reached)
            let _permit = sem.acquire().await.unwrap();

            println!("  Operation {} started", i);
            sleep(Duration::from_millis(200)).await;
            println!("  Operation {} completed", i);

            // Permit automatically released when dropped
        });

        handles.push(handle);
    }

    // Wait for all operations to complete
    for handle in handles {
        handle.await.unwrap();
    }

    println!("\nAll operations completed with resource isolation");
}

/// Example 8: Health Check with Circuit Breaker
async fn example_health_check() {
    println!("\n=== Example 8: Health Check with Circuit Breaker ===");

    let config = CircuitBreakerConfig {
        failure_threshold: 2,
        timeout: Duration::from_millis(300),
        success_threshold: 1,
    };

    let circuit_breaker = Arc::new(CircuitBreaker::new(config));

    // Health check function
    async fn health_check(
        cb: Arc<CircuitBreaker>,
        service_name: &str,
    ) -> ResilienceResult<String> {
        cb.execute(|| async {
            // Simulate health check
            sleep(Duration::from_millis(50)).await;
            Ok::<_, ServiceError>(format!("{} is healthy", service_name))
        })
        .await
    }

    // Monitor service health
    println!("Monitoring service health...\n");

    for i in 1..=8 {
        sleep(Duration::from_millis(100)).await;

        let state = circuit_breaker.state().await;
        let status = match state {
            CircuitState::Closed => "HEALTHY",
            CircuitState::HalfOpen => "DEGRADED",
            CircuitState::Open => "UNHEALTHY",
        };

        println!("Check #{}: Circuit {:?} [{}]", i, state, status);

        // Simulate some failures
        if i >= 3 && i <= 4 {
            let _ = circuit_breaker
                .execute(|| async { Err::<(), _>(ServiceError::ServerError) })
                .await;
        } else if i >= 6 {
            let _ = health_check(circuit_breaker.clone(), "API Service").await;
        }
    }
}

#[tokio::main]
async fn main() {
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║  Rustboot Resilience - Comprehensive Examples             ║");
    println!("╚════════════════════════════════════════════════════════════╝");

    example_retry_with_backoff().await;
    example_circuit_breaker().await;
    example_timeout().await;
    example_fallback().await;
    example_combined_patterns().await;
    example_realistic_api_client().await;
    example_bulkhead_pattern().await;
    example_health_check().await;

    println!("\n╔════════════════════════════════════════════════════════════╗");
    println!("║  All Examples Completed Successfully!                     ║");
    println!("╚════════════════════════════════════════════════════════════╝");
}
