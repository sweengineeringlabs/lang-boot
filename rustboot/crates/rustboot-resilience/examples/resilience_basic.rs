//! Rustboot Resilience - Basic Examples
//!
//! This example demonstrates practical usage of resilience patterns:
//! 1. Retry with exponential backoff
//! 2. Circuit breaker pattern
//! 3. Timeout pattern
//! 4. Combining patterns (retry with circuit breaker)
//! 5. Real-world scenario: unreliable external API calls
//!
//! Run with: cargo run --example resilience_basic

use dev_engineeringlabs_rustboot_resilience::{
    CircuitBreaker, CircuitBreakerConfig, CircuitState, ExponentialBackoff, ResilienceError,
    RetryPolicy, with_timeout,
};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

// ============================================================================
// Simulated External API
// ============================================================================

/// Simulates an unreliable external API (e.g., weather service, payment gateway)
struct UnreliableApi {
    call_count: Arc<AtomicUsize>,
    failure_rate: f64,
}

impl UnreliableApi {
    fn new(failure_rate: f64) -> Self {
        Self {
            call_count: Arc::new(AtomicUsize::new(0)),
            failure_rate,
        }
    }

    /// Simulate API call that might fail
    async fn fetch_data(&self, endpoint: &str) -> Result<String, String> {
        let count = self.call_count.fetch_add(1, Ordering::SeqCst);

        // Simulate network delay
        sleep(Duration::from_millis(50)).await;

        // Fail based on failure rate (first few attempts)
        if count < 2 && (count as f64) < (self.failure_rate * 10.0) {
            Err(format!("API Error: {} temporarily unavailable", endpoint))
        } else {
            Ok(format!("Data from {}: {{\"status\": \"success\", \"data\": \"...\" }}", endpoint))
        }
    }

    /// Simulate a slow API call
    async fn slow_fetch(&self) -> Result<String, String> {
        sleep(Duration::from_millis(300)).await;
        Ok("Slow response data".to_string())
    }
}

// ============================================================================
// Example 1: Simple Retry Pattern
// ============================================================================

async fn example_1_simple_retry() {
    println!("\n=== Example 1: Simple Retry Pattern ===");
    println!("Scenario: Calling an API that fails intermittently\n");

    let api = UnreliableApi::new(0.5);
    let retry = RetryPolicy::new(5); // Max 5 retries

    println!("Attempting API call with automatic retry...");
    let start = std::time::Instant::now();

    match retry.execute(|| async {
        let attempt = api.call_count.load(Ordering::SeqCst) + 1;
        println!("  - Attempt #{}", attempt);
        api.fetch_data("/api/weather").await
    }).await {
        Ok(response) => {
            println!("\n✓ Success after {:?}", start.elapsed());
            println!("  Response: {}", response);
        }
        Err(e) => {
            println!("\n✗ Failed after {:?}: {}", start.elapsed(), e);
        }
    }
}

// ============================================================================
// Example 2: Retry with Exponential Backoff
// ============================================================================

async fn example_2_exponential_backoff() {
    println!("\n=== Example 2: Retry with Exponential Backoff ===");
    println!("Scenario: Rate-limited API that needs progressive delays\n");

    let api = UnreliableApi::new(0.6);

    // Configure exponential backoff: 100ms, 200ms, 400ms, 800ms...
    let backoff = ExponentialBackoff::new(
        Duration::from_millis(100), // Initial delay
        Duration::from_secs(5),      // Max delay
        2.0,                         // Multiplier
    );

    let retry = RetryPolicy::new(4).with_backoff(backoff);

    println!("Attempting API call with exponential backoff...");
    let start = std::time::Instant::now();

    match retry.execute(|| async {
        let attempt = api.call_count.load(Ordering::SeqCst) + 1;
        let elapsed = start.elapsed();
        println!("  - Attempt #{} at {:?}", attempt, elapsed);
        api.fetch_data("/api/users").await
    }).await {
        Ok(response) => {
            println!("\n✓ Success after {:?}", start.elapsed());
            println!("  Response: {}", response);
        }
        Err(e) => {
            println!("\n✗ Failed after {:?}: {}", start.elapsed(), e);
        }
    }
}

// ============================================================================
// Example 3: Timeout Pattern
// ============================================================================

async fn example_3_timeout() {
    println!("\n=== Example 3: Timeout Pattern ===");
    println!("Scenario: Preventing indefinite waits on slow services\n");

    let api = UnreliableApi::new(0.0);

    // Fast operation - completes within timeout
    println!("Testing fast operation (timeout: 100ms)...");
    let result = with_timeout(Duration::from_millis(100), async {
        sleep(Duration::from_millis(30)).await;
        "Quick response"
    }).await;

    match result {
        Ok(data) => println!("  ✓ Completed: {}\n", data),
        Err(e) => println!("  ✗ {}\n", e),
    }

    // Slow operation - exceeds timeout
    println!("Testing slow operation (timeout: 100ms)...");
    let result = with_timeout(Duration::from_millis(100), api.slow_fetch()).await;

    match result {
        Ok(Ok(data)) => println!("  ✓ Completed: {}", data),
        Err(ResilienceError::Timeout(duration)) => {
            println!("  ✗ Operation timed out after {:?}", duration);
            println!("  → This prevents hanging on unresponsive services");
        }
        _ => println!("  ✗ Unexpected error"),
    }
}

// ============================================================================
// Example 4: Circuit Breaker Pattern
// ============================================================================

async fn example_4_circuit_breaker() {
    println!("\n\n=== Example 4: Circuit Breaker Pattern ===");
    println!("Scenario: Protecting against cascading failures\n");

    let config = CircuitBreakerConfig {
        failure_threshold: 3,        // Open after 3 failures
        timeout: Duration::from_secs(2), // Wait 2 seconds before retry
        success_threshold: 2,        // Need 2 successes to close
    };

    let circuit = CircuitBreaker::new(config);

    println!("Initial state: {:?}\n", circuit.state().await);

    // Simulate multiple failures
    println!("Triggering failures...");
    for i in 1..=5 {
        let result = circuit.execute(|| async {
            Err::<(), _>("Service unavailable")
        }).await;

        let state = circuit.state().await;
        println!("  Attempt {}: {:?} | Circuit: {:?}", i,
                 if result.is_ok() { "Success" } else { "Failed" },
                 state);

        if state == CircuitState::Open && i == 4 {
            println!("\n  → Circuit OPENED! Subsequent requests fail fast");
            println!("  → This prevents overwhelming a failing service\n");
        }
    }

    println!("\nCurrent state: {:?}", circuit.state().await);
    println!("Waiting for timeout to attempt recovery...");
    sleep(Duration::from_millis(2100)).await;

    // Simulate recovery
    println!("\nAttempting recovery...");
    for i in 1..=3 {
        let result = circuit.execute(|| async {
            Ok::<_, String>(format!("Recovery attempt {}", i))
        }).await;

        let state = circuit.state().await;
        println!("  Recovery {}: {:?} | Circuit: {:?}", i,
                 if result.is_ok() { "Success" } else { "Failed" },
                 state);
    }

    println!("\nFinal state: {:?}", circuit.state().await);
}

// ============================================================================
// Example 5: Combining Patterns (Real-World Scenario)
// ============================================================================

async fn example_5_combined_patterns() {
    println!("\n\n=== Example 5: Combining Multiple Patterns ===");
    println!("Scenario: Production-grade API client with full resilience\n");

    // Create a resilient API client
    struct ResilientClient {
        api: UnreliableApi,
        circuit_breaker: Arc<CircuitBreaker>,
        retry_policy: RetryPolicy,
    }

    impl ResilientClient {
        fn new() -> Self {
            let circuit_config = CircuitBreakerConfig {
                failure_threshold: 3,
                timeout: Duration::from_secs(5),
                success_threshold: 2,
            };

            let backoff = ExponentialBackoff::new(
                Duration::from_millis(100),
                Duration::from_secs(2),
                2.0,
            );

            Self {
                api: UnreliableApi::new(0.5),
                circuit_breaker: Arc::new(CircuitBreaker::new(circuit_config)),
                retry_policy: RetryPolicy::new(3).with_backoff(backoff),
            }
        }

        async fn fetch_with_resilience(&self, endpoint: &str) -> Result<String, ResilienceError> {
            let cb = self.circuit_breaker.clone();
            let endpoint = endpoint.to_string();

            // Retry with exponential backoff
            self.retry_policy.execute(|| {
                let cb_inner = cb.clone();
                let endpoint_inner = endpoint.clone();
                let api_ref = &self.api;

                async move {
                    // Apply timeout to each attempt
                    with_timeout(Duration::from_secs(1), async {
                        // Circuit breaker protects the service
                        cb_inner.execute(|| async {
                            api_ref.fetch_data(&endpoint_inner).await
                        }).await
                    }).await
                    .and_then(|r| r) // Flatten nested Result
                }
            }).await
        }
    }

    let client = ResilientClient::new();

    println!("Calling API with combined resilience patterns:");
    println!("  ✓ Circuit breaker (prevents cascading failures)");
    println!("  ✓ Retry with exponential backoff (handles transient errors)");
    println!("  ✓ Timeout per attempt (prevents hanging)\n");

    let start = std::time::Instant::now();

    match client.fetch_with_resilience("/api/payment/process").await {
        Ok(response) => {
            println!("✓ Success after {:?}", start.elapsed());
            println!("  Response: {}", response);
            println!("  Circuit state: {:?}", client.circuit_breaker.state().await);
        }
        Err(e) => {
            println!("✗ Failed after {:?}", start.elapsed());
            println!("  Error: {}", e);
            println!("  Circuit state: {:?}", client.circuit_breaker.state().await);
        }
    }
}

// ============================================================================
// Example 6: Real-World External API Call
// ============================================================================

async fn example_6_real_world_scenario() {
    println!("\n\n=== Example 6: Real-World Scenario ===");
    println!("Scenario: E-commerce checkout calling payment gateway\n");

    struct PaymentGateway {
        attempts: Arc<AtomicUsize>,
    }

    impl PaymentGateway {
        fn new() -> Self {
            Self {
                attempts: Arc::new(AtomicUsize::new(0)),
            }
        }

        async fn process_payment(&self, amount: f64) -> Result<String, String> {
            let attempt = self.attempts.fetch_add(1, Ordering::SeqCst);

            // Simulate network delay
            sleep(Duration::from_millis(100)).await;

            // Simulate transient failures (first 2 attempts fail)
            if attempt < 2 {
                Err(format!("Payment gateway error: Network timeout (attempt #{})", attempt + 1))
            } else {
                Ok(format!("Payment confirmed: ${:.2} | Transaction ID: TXN-{}", amount, attempt))
            }
        }
    }

    let gateway = PaymentGateway::new();

    // Configure aggressive retry for critical payment operations
    let backoff = ExponentialBackoff::new(
        Duration::from_millis(200),  // Start with 200ms
        Duration::from_secs(3),       // Max 3 seconds
        2.0,
    );
    let retry = RetryPolicy::new(5).with_backoff(backoff);

    // Circuit breaker to protect against gateway outages
    let circuit_config = CircuitBreakerConfig {
        failure_threshold: 5,
        timeout: Duration::from_secs(30),
        success_threshold: 2,
    };
    let circuit = Arc::new(CircuitBreaker::new(circuit_config));

    println!("Processing payment of $99.99...");
    let start = std::time::Instant::now();

    let cb = circuit.clone();
    let result = retry.execute(|| {
        let cb_inner = cb.clone();
        let gateway_ref = &gateway;

        async move {
            let attempt = gateway_ref.attempts.load(Ordering::SeqCst) + 1;
            println!("  → Attempt #{}", attempt);

            // Timeout + Circuit Breaker + Actual call
            with_timeout(Duration::from_secs(5), async {
                cb_inner.execute(|| async {
                    gateway_ref.process_payment(99.99).await
                }).await
            }).await
            .and_then(|r| r)
        }
    }).await;

    match result {
        Ok(confirmation) => {
            println!("\n✓ Payment processed successfully after {:?}", start.elapsed());
            println!("  {}", confirmation);
        }
        Err(e) => {
            println!("\n✗ Payment failed after {:?}", start.elapsed());
            println!("  Error: {}", e);
            println!("  → Would trigger fallback: notify customer, log for retry");
        }
    }

    println!("  Circuit state: {:?}", circuit.state().await);
}

// ============================================================================
// Main Function
// ============================================================================

#[tokio::main]
async fn main() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║                                                              ║");
    println!("║        Rustboot Resilience - Practical Examples             ║");
    println!("║                                                              ║");
    println!("║  Building fault-tolerant systems with:                      ║");
    println!("║  • Retry policies with exponential backoff                  ║");
    println!("║  • Circuit breakers for cascading failure protection        ║");
    println!("║  • Timeouts to prevent resource exhaustion                  ║");
    println!("║  • Combined patterns for production resilience              ║");
    println!("║                                                              ║");
    println!("╚══════════════════════════════════════════════════════════════╝");

    // Run all examples
    example_1_simple_retry().await;
    example_2_exponential_backoff().await;
    example_3_timeout().await;
    example_4_circuit_breaker().await;
    example_5_combined_patterns().await;
    example_6_real_world_scenario().await;

    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║                                                              ║");
    println!("║           All Examples Completed Successfully!              ║");
    println!("║                                                              ║");
    println!("║  Key Takeaways:                                              ║");
    println!("║  1. Use retries for transient failures                      ║");
    println!("║  2. Add exponential backoff to avoid overwhelming services  ║");
    println!("║  3. Circuit breakers prevent cascading failures             ║");
    println!("║  4. Timeouts prevent resource exhaustion                    ║");
    println!("║  5. Combine patterns for production-grade resilience        ║");
    println!("║                                                              ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");
}
