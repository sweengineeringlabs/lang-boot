//! Integration tests for rustboot-resilience
//!
//! Tests the public API as an external user would use it

use dev_engineeringlabs_rustboot_resilience::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;

// ============================================================================
// Circuit Breaker Tests
// ============================================================================

#[tokio::test]
async fn test_circuit_breaker_closed_state() {
    let config = CircuitBreakerConfig {
        failure_threshold: 3,
        timeout: Duration::from_secs(1),
        success_threshold: 2,
    };
    let breaker = CircuitBreaker::new(config);

    // Initially closed
    assert_eq!(breaker.state().await, CircuitState::Closed);

    // Successful operations keep it closed
    for _ in 0..5 {
        let result = breaker.execute(|| async { Ok::<_, String>(42) }).await;
        assert!(result.is_ok());
        assert_eq!(breaker.state().await, CircuitState::Closed);
    }
}

#[tokio::test]
async fn test_circuit_breaker_opens_after_failures() {
    let config = CircuitBreakerConfig {
        failure_threshold: 3,
        timeout: Duration::from_millis(100),
        success_threshold: 2,
    };
    let breaker = CircuitBreaker::new(config);

    assert_eq!(breaker.state().await, CircuitState::Closed);

    // Trigger failures
    for i in 0..3 {
        let result = breaker.execute(|| async { Err::<(), _>("service unavailable") }).await;
        assert!(result.is_err());

        if i < 2 {
            assert_eq!(breaker.state().await, CircuitState::Closed);
        } else {
            assert_eq!(breaker.state().await, CircuitState::Open);
        }
    }

    // Circuit is open - requests fail fast
    let result = breaker.execute(|| async { Ok::<_, String>(42) }).await;
    assert!(matches!(result, Err(ResilienceError::CircuitOpen)));
}

#[tokio::test]
async fn test_circuit_breaker_half_open_transition() {
    let config = CircuitBreakerConfig {
        failure_threshold: 2,
        timeout: Duration::from_millis(50),
        success_threshold: 2,
    };
    let breaker = CircuitBreaker::new(config);

    // Open the circuit
    for _ in 0..2 {
        let _ = breaker.execute(|| async { Err::<(), _>("error") }).await;
    }
    assert_eq!(breaker.state().await, CircuitState::Open);

    // Wait for timeout to allow half-open
    tokio::time::sleep(Duration::from_millis(60)).await;

    // Next request transitions to half-open
    let result = breaker.execute(|| async { Ok::<_, String>(1) }).await;
    assert!(result.is_ok());
    assert_eq!(breaker.state().await, CircuitState::HalfOpen);
}

#[tokio::test]
async fn test_circuit_breaker_half_open_to_closed() {
    let config = CircuitBreakerConfig {
        failure_threshold: 2,
        timeout: Duration::from_millis(50),
        success_threshold: 2,
    };
    let breaker = CircuitBreaker::new(config);

    // Open the circuit
    for _ in 0..2 {
        let _ = breaker.execute(|| async { Err::<(), _>("error") }).await;
    }

    // Wait and transition to half-open
    tokio::time::sleep(Duration::from_millis(60)).await;
    let _ = breaker.execute(|| async { Ok::<_, String>(1) }).await;
    assert_eq!(breaker.state().await, CircuitState::HalfOpen);

    // Success threshold reached - circuit closes
    let _ = breaker.execute(|| async { Ok::<_, String>(2) }).await;
    assert_eq!(breaker.state().await, CircuitState::Closed);
}

#[tokio::test]
async fn test_circuit_breaker_half_open_to_open() {
    let config = CircuitBreakerConfig {
        failure_threshold: 2,
        timeout: Duration::from_millis(50),
        success_threshold: 2,
    };
    let breaker = CircuitBreaker::new(config);

    // Open the circuit
    for _ in 0..2 {
        let _ = breaker.execute(|| async { Err::<(), _>("error") }).await;
    }

    // Wait and transition to half-open
    tokio::time::sleep(Duration::from_millis(60)).await;
    let _ = breaker.execute(|| async { Ok::<_, String>(1) }).await;
    assert_eq!(breaker.state().await, CircuitState::HalfOpen);

    // Failure in half-open state reopens circuit
    let _ = breaker.execute(|| async { Err::<(), _>("error again") }).await;
    assert_eq!(breaker.state().await, CircuitState::Open);
}

#[tokio::test]
async fn test_circuit_breaker_concurrent_access() {
    let config = CircuitBreakerConfig {
        failure_threshold: 5,
        timeout: Duration::from_millis(100),
        success_threshold: 2,
    };
    let breaker = Arc::new(CircuitBreaker::new(config));

    // Spawn multiple concurrent operations
    let mut handles = vec![];
    for i in 0..10 {
        let breaker_clone = Arc::clone(&breaker);
        let handle = tokio::spawn(async move {
            breaker_clone.execute(|| async move {
                tokio::time::sleep(Duration::from_millis(5)).await;
                Ok::<_, String>(i)
            }).await
        });
        handles.push(handle);
    }

    // All should succeed
    for handle in handles {
        let result = handle.await.unwrap();
        assert!(result.is_ok());
    }

    assert_eq!(breaker.state().await, CircuitState::Closed);
}

// ============================================================================
// Retry Policy Tests
// ============================================================================

#[tokio::test]
async fn test_retry_immediate_success() {
    let policy = RetryPolicy::new(3);
    let counter = Arc::new(AtomicUsize::new(0));
    let counter_clone = Arc::clone(&counter);

    let result = policy
        .execute(|| {
            let c = Arc::clone(&counter_clone);
            async move {
                c.fetch_add(1, Ordering::SeqCst);
                Ok::<_, String>(42)
            }
        })
        .await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 42);
    assert_eq!(counter.load(Ordering::SeqCst), 1);
}

#[tokio::test]
async fn test_retry_success_after_failures() {
    let policy = RetryPolicy::new(5);
    let counter = Arc::new(AtomicUsize::new(0));
    let counter_clone = Arc::clone(&counter);

    let result = policy
        .execute(|| {
            let c = Arc::clone(&counter_clone);
            async move {
                let count = c.fetch_add(1, Ordering::SeqCst);
                if count < 3 {
                    Err("temporary failure")
                } else {
                    Ok(100)
                }
            }
        })
        .await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 100);
    assert_eq!(counter.load(Ordering::SeqCst), 4); // Failed 3 times, succeeded on 4th
}

#[tokio::test]
async fn test_retry_max_retries_exceeded() {
    let policy = RetryPolicy::new(3);
    let counter = Arc::new(AtomicUsize::new(0));
    let counter_clone = Arc::clone(&counter);

    let result = policy
        .execute(|| {
            let c = Arc::clone(&counter_clone);
            async move {
                c.fetch_add(1, Ordering::SeqCst);
                Err::<i32, _>("persistent failure")
            }
        })
        .await;

    assert!(result.is_err());
    assert!(matches!(result, Err(ResilienceError::MaxRetriesExceeded(3))));
    assert_eq!(counter.load(Ordering::SeqCst), 3);
}

#[tokio::test]
async fn test_retry_with_custom_backoff() {
    let backoff = ExponentialBackoff::new(
        Duration::from_millis(10),
        Duration::from_millis(100),
        2.0,
    );
    let policy = RetryPolicy::new(4).with_backoff(backoff);
    let counter = Arc::new(AtomicUsize::new(0));
    let counter_clone = Arc::clone(&counter);

    let start = std::time::Instant::now();
    let result = policy
        .execute(|| {
            let c = Arc::clone(&counter_clone);
            async move {
                let count = c.fetch_add(1, Ordering::SeqCst);
                if count < 2 {
                    Err("retry me")
                } else {
                    Ok("success")
                }
            }
        })
        .await;
    let elapsed = start.elapsed();

    assert!(result.is_ok());
    // Should have delays: ~10ms + ~20ms = ~30ms minimum
    assert!(elapsed.as_millis() >= 25);
}

#[tokio::test]
async fn test_exponential_backoff_calculation() {
    let backoff = ExponentialBackoff::new(
        Duration::from_millis(100),
        Duration::from_secs(5),
        2.0,
    );

    assert_eq!(backoff.next_delay(1), Duration::from_millis(100));
    assert_eq!(backoff.next_delay(2), Duration::from_millis(200));
    assert_eq!(backoff.next_delay(3), Duration::from_millis(400));
    assert_eq!(backoff.next_delay(4), Duration::from_millis(800));
    assert_eq!(backoff.next_delay(5), Duration::from_millis(1600));

    // Max delay capping
    assert_eq!(backoff.next_delay(10), Duration::from_secs(5));
}

#[tokio::test]
async fn test_retry_different_backoff_multipliers() {
    let backoff = ExponentialBackoff::new(
        Duration::from_millis(50),
        Duration::from_secs(10),
        1.5,
    );

    assert_eq!(backoff.next_delay(1), Duration::from_millis(50));
    assert_eq!(backoff.next_delay(2), Duration::from_millis(75));
    assert_eq!(backoff.next_delay(3), Duration::from_millis(112));
}

// ============================================================================
// Timeout Tests
// ============================================================================

#[tokio::test]
async fn test_timeout_success() {
    let result = with_timeout(Duration::from_millis(100), async {
        tokio::time::sleep(Duration::from_millis(10)).await;
        42
    })
    .await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 42);
}

#[tokio::test]
async fn test_timeout_exceeded() {
    let result = with_timeout(Duration::from_millis(10), async {
        tokio::time::sleep(Duration::from_millis(100)).await;
        42
    })
    .await;

    assert!(result.is_err());
    assert!(matches!(result, Err(ResilienceError::Timeout(_))));
}

#[tokio::test]
async fn test_timeout_with_computation() {
    let result = with_timeout(Duration::from_millis(200), async {
        let mut sum = 0;
        for i in 0..1000 {
            sum += i;
            if i % 100 == 0 {
                tokio::time::sleep(Duration::from_micros(100)).await;
            }
        }
        sum
    })
    .await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 499500);
}

#[tokio::test]
async fn test_timeout_zero_duration() {
    // Zero timeout should fail immediately unless operation is instant
    let result = with_timeout(Duration::from_millis(0), async {
        tokio::time::sleep(Duration::from_millis(1)).await;
        42
    })
    .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_timeout_concurrent_operations() {
    let handles: Vec<_> = (0..5)
        .map(|i| {
            tokio::spawn(async move {
                with_timeout(Duration::from_millis(50), async move {
                    tokio::time::sleep(Duration::from_millis(10)).await;
                    i * 2
                })
                .await
            })
        })
        .collect();

    for (i, handle) in handles.into_iter().enumerate() {
        let result = handle.await.unwrap();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), i * 2);
    }
}

// ============================================================================
// Combined Resilience Patterns
// ============================================================================

#[tokio::test]
async fn test_retry_with_timeout() {
    let policy = RetryPolicy::new(3);
    let counter = Arc::new(AtomicUsize::new(0));
    let counter_clone = Arc::clone(&counter);

    let result = policy
        .execute(|| {
            let c = Arc::clone(&counter_clone);
            async move {
                let count = c.fetch_add(1, Ordering::SeqCst);
                with_timeout(Duration::from_millis(50), async move {
                    if count < 2 {
                        tokio::time::sleep(Duration::from_millis(100)).await;
                        Ok::<_, String>("too slow")
                    } else {
                        Ok("success")
                    }
                })
                .await
            }
        })
        .await;

    assert!(result.is_ok());
    // Result is nested: outer from retry, inner from the operation
    let inner_result = result.unwrap();
    assert!(inner_result.is_ok());
    assert_eq!(inner_result.unwrap(), "success");
    // First two attempts timeout, third succeeds
    assert_eq!(counter.load(Ordering::SeqCst), 3);
}

#[tokio::test]
async fn test_circuit_breaker_with_retry() {
    let breaker_config = CircuitBreakerConfig {
        failure_threshold: 3,
        timeout: Duration::from_millis(100),
        success_threshold: 2,
    };
    let breaker = Arc::new(CircuitBreaker::new(breaker_config));
    let retry_policy = RetryPolicy::new(2);

    let counter = Arc::new(AtomicUsize::new(0));
    let counter_clone = Arc::clone(&counter);
    let breaker_clone = Arc::clone(&breaker);

    // This will retry the operation, but circuit breaker may open
    let result = retry_policy
        .execute(|| {
            let b = Arc::clone(&breaker_clone);
            let c = Arc::clone(&counter_clone);
            async move {
                c.fetch_add(1, Ordering::SeqCst);
                b.execute(|| async { Ok::<_, String>(42) }).await
            }
        })
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_circuit_breaker_with_timeout() {
    let config = CircuitBreakerConfig {
        failure_threshold: 2,
        timeout: Duration::from_millis(100),
        success_threshold: 1,
    };
    let breaker = CircuitBreaker::new(config);

    // Operations that timeout are treated as failures
    for _ in 0..2 {
        let result = breaker
            .execute(|| async {
                with_timeout(Duration::from_millis(10), async {
                    tokio::time::sleep(Duration::from_millis(50)).await;
                    Ok::<_, String>(42)
                })
                .await
            })
            .await;
        assert!(result.is_err());
    }

    assert_eq!(breaker.state().await, CircuitState::Open);
}

#[tokio::test]
async fn test_retry_timeout_circuit_breaker_combo() {
    // Test combining all three patterns: retry, circuit breaker, and timeout
    let breaker = Arc::new(CircuitBreaker::new(CircuitBreakerConfig {
        failure_threshold: 5,
        timeout: Duration::from_secs(1),
        success_threshold: 2,
    }));
    let retry_policy = RetryPolicy::new(3);

    let counter = Arc::new(AtomicUsize::new(0));
    let counter_clone = Arc::clone(&counter);
    let breaker_clone = Arc::clone(&breaker);

    let result = retry_policy
        .execute(|| {
            let b = Arc::clone(&breaker_clone);
            let c = Arc::clone(&counter_clone);
            async move {
                let count = c.fetch_add(1, Ordering::SeqCst);
                // Use circuit breaker for the operation
                b.execute(|| async move {
                    // Use timeout for individual operation
                    match with_timeout(Duration::from_millis(100), async move {
                        if count < 2 {
                            tokio::time::sleep(Duration::from_millis(10)).await;
                            Err::<&str, _>("temporary failure")
                        } else {
                            Ok("success")
                        }
                    })
                    .await
                    {
                        Ok(inner_result) => inner_result.map_err(|e| e.to_string()),
                        Err(e) => Err(e.to_string()),
                    }
                })
                .await
            }
        })
        .await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "success");
}

// ============================================================================
// Real-world Scenarios
// ============================================================================

#[tokio::test]
async fn test_database_retry_scenario() {
    // Simulates a flaky database connection
    let policy = RetryPolicy::new(5).with_backoff(ExponentialBackoff::new(
        Duration::from_millis(10),
        Duration::from_millis(200),
        2.0,
    ));

    let attempt = Arc::new(AtomicUsize::new(0));
    let attempt_clone = Arc::clone(&attempt);

    let result = policy
        .execute(|| {
            let a = Arc::clone(&attempt_clone);
            async move {
                let current = a.fetch_add(1, Ordering::SeqCst);
                // Simulate connection failures
                if current < 2 {
                    Err("connection refused")
                } else {
                    Ok(vec!["data1", "data2", "data3"])
                }
            }
        })
        .await;

    assert!(result.is_ok());
    let data = result.unwrap();
    assert_eq!(data.len(), 3);
}

#[tokio::test]
async fn test_api_call_with_circuit_breaker() {
    let config = CircuitBreakerConfig {
        failure_threshold: 3,
        timeout: Duration::from_millis(50),
        success_threshold: 2,
    };
    let breaker = Arc::new(CircuitBreaker::new(config));

    // Simulate API failures
    let failure_count = Arc::new(AtomicUsize::new(0));

    for _ in 0..3 {
        let fc = Arc::clone(&failure_count);
        let result = breaker
            .execute(|| async move {
                fc.fetch_add(1, Ordering::SeqCst);
                Err::<String, _>("503 Service Unavailable")
            })
            .await;
        assert!(result.is_err());
    }

    assert_eq!(breaker.state().await, CircuitState::Open);
    assert_eq!(failure_count.load(Ordering::SeqCst), 3);

    // Circuit is open - fast fail
    let result = breaker.execute(|| async { Ok::<_, String>("data") }).await;
    assert!(matches!(result, Err(ResilienceError::CircuitOpen)));
    // Counter doesn't increment because circuit is open
    assert_eq!(failure_count.load(Ordering::SeqCst), 3);
}

#[tokio::test]
async fn test_microservice_call_chain() {
    // Simulates calling multiple microservices with resilience patterns
    let service_a_breaker = Arc::new(CircuitBreaker::new(CircuitBreakerConfig {
        failure_threshold: 3,
        timeout: Duration::from_millis(100),
        success_threshold: 2,
    }));

    let service_b_retry = RetryPolicy::new(3);

    // Service A call with circuit breaker
    let service_a_result = service_a_breaker
        .execute(|| async {
            with_timeout(Duration::from_millis(50), async {
                tokio::time::sleep(Duration::from_millis(5)).await;
                Ok::<_, String>("user_data")
            })
            .await
        })
        .await;

    assert!(service_a_result.is_ok());

    // Service B call with retry
    let counter = Arc::new(AtomicUsize::new(0));
    let counter_clone = Arc::clone(&counter);

    let service_b_result = service_b_retry
        .execute(|| {
            let c = Arc::clone(&counter_clone);
            async move {
                let count = c.fetch_add(1, Ordering::SeqCst);
                if count < 1 {
                    Err("temporary glitch")
                } else {
                    Ok("order_data")
                }
            }
        })
        .await;

    assert!(service_b_result.is_ok());

    // Combine results - unwrap nested Results
    let user_data_inner = service_a_result.unwrap();
    assert!(user_data_inner.is_ok());
    assert_eq!(user_data_inner.unwrap(), "user_data");

    let order_data = service_b_result.unwrap();
    assert_eq!(order_data, "order_data");
}

#[tokio::test]
async fn test_batch_processing_with_resilience() {
    let policy = RetryPolicy::new(2);
    let breaker = Arc::new(CircuitBreaker::new(CircuitBreakerConfig::default()));

    let items = vec![1, 2, 3, 4, 5];
    let mut results = Vec::new();

    for item in items {
        let b = Arc::clone(&breaker);
        let result = policy
            .execute(|| {
                let b_clone = Arc::clone(&b);
                async move {
                    b_clone
                        .execute(|| async move {
                            // Simulate processing
                            tokio::time::sleep(Duration::from_millis(5)).await;
                            Ok::<_, String>(item * 2)
                        })
                        .await
                }
            })
            .await;

        if let Ok(value) = result {
            results.push(value);
        }
    }

    assert_eq!(results, vec![2, 4, 6, 8, 10]);
}

// ============================================================================
// Async Scenarios
// ============================================================================

#[tokio::test]
async fn test_async_operations_with_shared_state() {
    let breaker = Arc::new(CircuitBreaker::new(CircuitBreakerConfig {
        failure_threshold: 5,
        timeout: Duration::from_millis(200),
        success_threshold: 2,
    }));

    let shared_counter = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let b = Arc::clone(&breaker);
        let c = Arc::clone(&shared_counter);

        let handle = tokio::spawn(async move {
            b.execute(|| async {
                let value = c.fetch_add(1, Ordering::SeqCst);
                tokio::time::sleep(Duration::from_millis(5)).await;
                Ok::<_, String>(value)
            })
            .await
        });

        handles.push(handle);
    }

    let mut results = vec![];
    for handle in handles {
        if let Ok(Ok(value)) = handle.await {
            results.push(value);
        }
    }

    assert_eq!(results.len(), 10);
    assert_eq!(shared_counter.load(Ordering::SeqCst), 10);
}

#[tokio::test]
async fn test_concurrent_retry_policies() {
    let policy = Arc::new(RetryPolicy::new(3));
    let mut handles = vec![];

    for i in 0..5 {
        let p = Arc::clone(&policy);
        let handle = tokio::spawn(async move {
            let counter = Arc::new(AtomicUsize::new(0));
            let counter_clone = Arc::clone(&counter);

            p.execute(|| {
                let c = Arc::clone(&counter_clone);
                async move {
                    let count = c.fetch_add(1, Ordering::SeqCst);
                    if count < 1 {
                        Err("retry once")
                    } else {
                        Ok(i * 10)
                    }
                }
            })
            .await
        });
        handles.push(handle);
    }

    let mut results = vec![];
    for handle in handles {
        if let Ok(Ok(value)) = handle.await {
            results.push(value);
        }
    }

    results.sort();
    assert_eq!(results, vec![0, 10, 20, 30, 40]);
}

#[tokio::test]
async fn test_timeout_with_cancel_safety() {
    // Test that timeout properly cancels operations
    let counter = Arc::new(AtomicUsize::new(0));
    let counter_clone = Arc::clone(&counter);

    let result = with_timeout(Duration::from_millis(20), async move {
        for _ in 0..100 {
            counter_clone.fetch_add(1, Ordering::SeqCst);
            tokio::time::sleep(Duration::from_millis(5)).await;
        }
        "completed"
    })
    .await;

    assert!(result.is_err());
    // Counter should not reach 100 because operation was cancelled
    assert!(counter.load(Ordering::SeqCst) < 100);
}

// ============================================================================
// Edge Cases
// ============================================================================

#[tokio::test]
async fn test_circuit_breaker_exactly_at_threshold() {
    let config = CircuitBreakerConfig {
        failure_threshold: 3,
        timeout: Duration::from_millis(50),
        success_threshold: 2,
    };
    let breaker = CircuitBreaker::new(config);

    // Two failures - still closed
    for _ in 0..2 {
        let _ = breaker.execute(|| async { Err::<(), _>("error") }).await;
        assert_eq!(breaker.state().await, CircuitState::Closed);
    }

    // Third failure - opens
    let _ = breaker.execute(|| async { Err::<(), _>("error") }).await;
    assert_eq!(breaker.state().await, CircuitState::Open);
}

#[tokio::test]
async fn test_retry_with_zero_retries() {
    let policy = RetryPolicy::new(0);
    let counter = Arc::new(AtomicUsize::new(0));
    let counter_clone = Arc::clone(&counter);

    let result = policy
        .execute(|| {
            let c = Arc::clone(&counter_clone);
            async move {
                c.fetch_add(1, Ordering::SeqCst);
                Err::<(), _>("error")
            }
        })
        .await;

    // Should fail after first attempt with no retries
    assert!(result.is_err());
    assert_eq!(counter.load(Ordering::SeqCst), 1); // Tries once, then no retries
}

#[tokio::test]
async fn test_circuit_breaker_success_resets_failure_count() {
    let config = CircuitBreakerConfig {
        failure_threshold: 3,
        timeout: Duration::from_millis(50),
        success_threshold: 2,
    };
    let breaker = CircuitBreaker::new(config);

    // Two failures
    for _ in 0..2 {
        let _ = breaker.execute(|| async { Err::<(), _>("error") }).await;
    }

    // Success should reset failure count
    let _ = breaker.execute(|| async { Ok::<_, String>(42) }).await;
    assert_eq!(breaker.state().await, CircuitState::Closed);

    // Now we can have 2 more failures before opening
    for _ in 0..2 {
        let _ = breaker.execute(|| async { Err::<(), _>("error") }).await;
        assert_eq!(breaker.state().await, CircuitState::Closed);
    }
}

#[tokio::test]
async fn test_multiple_circuit_breaker_cycles() {
    let config = CircuitBreakerConfig {
        failure_threshold: 2,
        timeout: Duration::from_millis(30),
        success_threshold: 1,
    };
    let breaker = CircuitBreaker::new(config);

    // Cycle 1: Close -> Open
    for _ in 0..2 {
        let _ = breaker.execute(|| async { Err::<(), _>("error") }).await;
    }
    assert_eq!(breaker.state().await, CircuitState::Open);

    // Cycle 2: Open -> HalfOpen -> Closed
    tokio::time::sleep(Duration::from_millis(35)).await;
    let _ = breaker.execute(|| async { Ok::<_, String>(1) }).await;
    assert_eq!(breaker.state().await, CircuitState::Closed);

    // Cycle 3: Close -> Open again
    for _ in 0..2 {
        let _ = breaker.execute(|| async { Err::<(), _>("error") }).await;
    }
    assert_eq!(breaker.state().await, CircuitState::Open);
}

// ============================================================================
// Bulkhead Pattern Tests (Concurrency Limiting)
// ============================================================================

#[tokio::test]
async fn test_bulkhead_limits_concurrency() {
    use tokio::sync::Semaphore;

    let max_concurrent = 3;
    let semaphore = Arc::new(Semaphore::new(max_concurrent));
    let active_count = Arc::new(AtomicUsize::new(0));
    let max_observed = Arc::new(AtomicUsize::new(0));

    let mut handles = vec![];

    for _ in 0..10 {
        let sem = Arc::clone(&semaphore);
        let active = Arc::clone(&active_count);
        let max_obs = Arc::clone(&max_observed);

        let handle = tokio::spawn(async move {
            let _permit = sem.acquire().await.unwrap();

            // Track active operations
            let current = active.fetch_add(1, Ordering::SeqCst) + 1;

            // Update max observed
            max_obs.fetch_max(current, Ordering::SeqCst);

            // Simulate work
            tokio::time::sleep(Duration::from_millis(10)).await;

            active.fetch_sub(1, Ordering::SeqCst);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }

    // Should never exceed the limit
    assert!(max_observed.load(Ordering::SeqCst) <= max_concurrent);
}

#[tokio::test]
async fn test_bulkhead_with_timeout() {
    use tokio::sync::Semaphore;

    let semaphore = Arc::new(Semaphore::new(2));

    // Fill up the semaphore
    let permit1 = semaphore.clone().acquire_owned().await.unwrap();
    let permit2 = semaphore.clone().acquire_owned().await.unwrap();

    // Try to acquire with timeout - should fail
    let sem_clone = Arc::clone(&semaphore);
    let handle = tokio::spawn(async move {
        tokio::time::timeout(
            Duration::from_millis(50),
            sem_clone.acquire_owned(),
        )
        .await
    });

    let result = handle.await.unwrap();
    assert!(result.is_err()); // Should timeout

    // Release permits
    drop(permit1);
    drop(permit2);

    // Now should succeed
    let result = tokio::time::timeout(
        Duration::from_millis(50),
        semaphore.acquire_owned(),
    )
    .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_bulkhead_resource_isolation() {
    use tokio::sync::Semaphore;

    // Separate resource pools for different services
    let db_pool = Arc::new(Semaphore::new(2));
    let api_pool = Arc::new(Semaphore::new(3));

    let db_counter = Arc::new(AtomicUsize::new(0));
    let api_counter = Arc::new(AtomicUsize::new(0));

    let mut handles = vec![];

    // Spawn DB operations
    for _ in 0..5 {
        let pool = Arc::clone(&db_pool);
        let counter = Arc::clone(&db_counter);

        let handle = tokio::spawn(async move {
            let _permit = pool.acquire().await.unwrap();
            counter.fetch_add(1, Ordering::SeqCst);
            tokio::time::sleep(Duration::from_millis(10)).await;
        });
        handles.push(handle);
    }

    // Spawn API operations
    for _ in 0..5 {
        let pool = Arc::clone(&api_pool);
        let counter = Arc::clone(&api_counter);

        let handle = tokio::spawn(async move {
            let _permit = pool.acquire().await.unwrap();
            counter.fetch_add(1, Ordering::SeqCst);
            tokio::time::sleep(Duration::from_millis(10)).await;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }

    assert_eq!(db_counter.load(Ordering::SeqCst), 5);
    assert_eq!(api_counter.load(Ordering::SeqCst), 5);
}

#[tokio::test]
async fn test_bulkhead_with_circuit_breaker() {
    use tokio::sync::Semaphore;

    let semaphore = Arc::new(Semaphore::new(2));
    let breaker = Arc::new(CircuitBreaker::new(CircuitBreakerConfig {
        failure_threshold: 3,
        timeout: Duration::from_millis(100),
        success_threshold: 2,
    }));

    let mut handles = vec![];

    for i in 0..5 {
        let sem = Arc::clone(&semaphore);
        let cb = Arc::clone(&breaker);

        let handle = tokio::spawn(async move {
            let _permit = sem.acquire().await.unwrap();

            cb.execute(|| async move {
                tokio::time::sleep(Duration::from_millis(5)).await;
                Ok::<_, String>(i * 2)
            })
            .await
        });

        handles.push(handle);
    }

    let mut results = vec![];
    for handle in handles {
        if let Ok(Ok(value)) = handle.await {
            results.push(value);
        }
    }

    assert_eq!(results.len(), 5);
    assert_eq!(breaker.state().await, CircuitState::Closed);
}

#[tokio::test]
async fn test_bulkhead_prevents_resource_exhaustion() {
    use tokio::sync::Semaphore;

    let semaphore = Arc::new(Semaphore::new(3));
    let started = Arc::new(AtomicUsize::new(0));
    let completed = Arc::new(AtomicUsize::new(0));

    let mut handles = vec![];

    for _ in 0..100 {
        let sem = Arc::clone(&semaphore);
        let s = Arc::clone(&started);
        let c = Arc::clone(&completed);

        let handle = tokio::spawn(async move {
            let _permit = sem.acquire().await.unwrap();
            s.fetch_add(1, Ordering::SeqCst);

            // Simulate work
            tokio::time::sleep(Duration::from_millis(1)).await;

            c.fetch_add(1, Ordering::SeqCst);
        });

        handles.push(handle);
    }

    // Wait a bit and check that not all started immediately
    tokio::time::sleep(Duration::from_millis(5)).await;
    let started_count = started.load(Ordering::SeqCst);
    assert!(started_count < 100); // Not all should have started

    // Wait for all to complete
    for handle in handles {
        handle.await.unwrap();
    }

    assert_eq!(completed.load(Ordering::SeqCst), 100);
}

// ============================================================================
// Advanced Error Propagation Tests
// ============================================================================

#[tokio::test]
async fn test_error_propagation_through_retry() {
    let policy = RetryPolicy::new(3);

    // Custom error type
    #[derive(Debug)]
    struct CustomError {
        code: i32,
        message: String,
    }

    impl std::fmt::Display for CustomError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Error {}: {}", self.code, self.message)
        }
    }

    let counter = Arc::new(AtomicUsize::new(0));
    let counter_clone = Arc::clone(&counter);

    let result = policy
        .execute(|| {
            let c = Arc::clone(&counter_clone);
            async move {
                c.fetch_add(1, Ordering::SeqCst);
                Err::<i32, _>(CustomError {
                    code: 500,
                    message: "Internal Server Error".to_string(),
                })
            }
        })
        .await;

    assert!(result.is_err());
    assert_eq!(counter.load(Ordering::SeqCst), 3);

    match result {
        Err(ResilienceError::MaxRetriesExceeded(n)) => {
            assert_eq!(n, 3);
        }
        _ => panic!("Expected MaxRetriesExceeded error"),
    }
}

#[tokio::test]
async fn test_error_propagation_through_circuit_breaker() {
    let breaker = CircuitBreaker::new(CircuitBreakerConfig {
        failure_threshold: 2,
        timeout: Duration::from_millis(50),
        success_threshold: 1,
    });

    // Trigger circuit to open
    for _ in 0..2 {
        let result = breaker
            .execute(|| async { Err::<(), _>("service error") })
            .await;

        assert!(result.is_err());
        if let Err(ResilienceError::OperationFailed(msg)) = result {
            assert_eq!(msg, "service error");
        }
    }

    // Circuit is open
    let result = breaker.execute(|| async { Ok::<_, String>(42) }).await;

    assert!(result.is_err());
    assert!(matches!(result, Err(ResilienceError::CircuitOpen)));
}

#[tokio::test]
async fn test_nested_error_propagation() {
    let policy = RetryPolicy::new(2);
    let breaker = Arc::new(CircuitBreaker::new(CircuitBreakerConfig::default()));

    let counter = Arc::new(AtomicUsize::new(0));
    let counter_clone = Arc::clone(&counter);
    let breaker_clone = Arc::clone(&breaker);

    let result = policy
        .execute(|| {
            let c = Arc::clone(&counter_clone);
            let b = Arc::clone(&breaker_clone);
            async move {
                c.fetch_add(1, Ordering::SeqCst);

                // Circuit breaker around timeout
                b.execute(|| async {
                    with_timeout(Duration::from_millis(50), async {
                        tokio::time::sleep(Duration::from_millis(10)).await;
                        Ok::<_, String>("success")
                    })
                    .await
                })
                .await
            }
        })
        .await;

    assert!(result.is_ok());
    let inner = result.unwrap();
    assert!(inner.is_ok());
    assert_eq!(inner.unwrap(), "success");
}

// ============================================================================
// Performance and Stress Tests
// ============================================================================

#[tokio::test]
async fn test_high_concurrency_circuit_breaker() {
    let breaker = Arc::new(CircuitBreaker::new(CircuitBreakerConfig {
        failure_threshold: 100,
        timeout: Duration::from_millis(500),
        success_threshold: 10,
    }));

    let mut handles = vec![];

    for i in 0..1000 {
        let b = Arc::clone(&breaker);

        let handle = tokio::spawn(async move {
            b.execute(|| async move {
                tokio::time::sleep(Duration::from_micros(100)).await;
                Ok::<_, String>(i)
            })
            .await
        });

        handles.push(handle);
    }

    let mut success_count = 0;
    for handle in handles {
        if let Ok(Ok(_)) = handle.await {
            success_count += 1;
        }
    }

    assert_eq!(success_count, 1000);
    assert_eq!(breaker.state().await, CircuitState::Closed);
}

#[tokio::test]
async fn test_retry_backoff_timing_accuracy() {
    let backoff = ExponentialBackoff::new(
        Duration::from_millis(100),
        Duration::from_secs(1),
        2.0,
    );
    let policy = RetryPolicy::new(3).with_backoff(backoff);

    let counter = Arc::new(AtomicUsize::new(0));
    let counter_clone = Arc::clone(&counter);

    let start = std::time::Instant::now();

    let result = policy
        .execute(|| {
            let c = Arc::clone(&counter_clone);
            async move {
                let count = c.fetch_add(1, Ordering::SeqCst);
                if count < 2 {
                    Err("retry")
                } else {
                    Ok("success")
                }
            }
        })
        .await;

    let elapsed = start.elapsed();

    assert!(result.is_ok());
    // Should take at least 100ms + 200ms = 300ms for 2 retries
    assert!(elapsed.as_millis() >= 290);
    assert!(elapsed.as_millis() < 500); // But not too long
}

#[tokio::test]
async fn test_timeout_precision() {
    let timeout_duration = Duration::from_millis(100);

    let start = std::time::Instant::now();
    let result = with_timeout(timeout_duration, async {
        tokio::time::sleep(Duration::from_millis(200)).await;
        42
    })
    .await;
    let elapsed = start.elapsed();

    assert!(result.is_err());
    // Should timeout around 100ms
    assert!(elapsed.as_millis() >= 95 && elapsed.as_millis() <= 150);
}

// ============================================================================
// Edge Case Tests
// ============================================================================

#[tokio::test]
async fn test_circuit_breaker_with_instant_operations() {
    let breaker = CircuitBreaker::new(CircuitBreakerConfig {
        failure_threshold: 3,
        timeout: Duration::from_millis(100),
        success_threshold: 2,
    });

    // Very fast operations should still work correctly
    for _ in 0..100 {
        let result = breaker.execute(|| async { Ok::<_, String>(42) }).await;
        assert!(result.is_ok());
    }

    assert_eq!(breaker.state().await, CircuitState::Closed);
}

#[tokio::test]
async fn test_retry_with_large_max_retries() {
    let policy = RetryPolicy::new(20);
    let counter = Arc::new(AtomicUsize::new(0));
    let counter_clone = Arc::clone(&counter);

    let result = policy
        .execute(|| {
            let c = Arc::clone(&counter_clone);
            async move {
                let count = c.fetch_add(1, Ordering::SeqCst);
                if count < 10 {
                    Err("keep retrying")
                } else {
                    Ok("finally succeeded")
                }
            }
        })
        .await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "finally succeeded");
    assert_eq!(counter.load(Ordering::SeqCst), 11);
}

#[tokio::test]
async fn test_combined_patterns_all_succeed() {
    let breaker = Arc::new(CircuitBreaker::new(CircuitBreakerConfig::default()));
    let policy = RetryPolicy::new(3);

    let result = policy
        .execute(|| {
            let b = Arc::clone(&breaker);
            async move {
                b.execute(|| async {
                    with_timeout(Duration::from_millis(100), async {
                        tokio::time::sleep(Duration::from_millis(10)).await;
                        Ok::<_, String>("all patterns passed")
                    })
                    .await
                })
                .await
            }
        })
        .await;

    assert!(result.is_ok());
    let inner = result.unwrap();
    assert!(inner.is_ok());
    assert_eq!(inner.unwrap(), "all patterns passed");
}

#[tokio::test]
async fn test_combined_patterns_timeout_triggers_retry() {
    let policy = RetryPolicy::new(3);
    let counter = Arc::new(AtomicUsize::new(0));
    let counter_clone = Arc::clone(&counter);

    let result = policy
        .execute(|| {
            let c = Arc::clone(&counter_clone);
            async move {
                let count = c.fetch_add(1, Ordering::SeqCst);
                with_timeout(
                    Duration::from_millis(50),
                    async move {
                        if count < 2 {
                            // First two attempts timeout
                            tokio::time::sleep(Duration::from_millis(100)).await;
                            Ok::<_, String>("timeout")
                        } else {
                            // Third attempt succeeds quickly
                            Ok("success")
                        }
                    },
                )
                .await
            }
        })
        .await;

    assert!(result.is_ok());
    assert_eq!(counter.load(Ordering::SeqCst), 3);
}

#[tokio::test]
async fn test_circuit_breaker_half_open_multiple_requests() {
    let config = CircuitBreakerConfig {
        failure_threshold: 2,
        timeout: Duration::from_millis(50),
        success_threshold: 3,
    };
    let breaker = CircuitBreaker::new(config);

    // Open the circuit
    for _ in 0..2 {
        let _ = breaker.execute(|| async { Err::<(), _>("error") }).await;
    }
    assert_eq!(breaker.state().await, CircuitState::Open);

    // Wait for timeout
    tokio::time::sleep(Duration::from_millis(60)).await;

    // First successful request transitions to half-open
    let _ = breaker.execute(|| async { Ok::<_, String>(1) }).await;
    assert_eq!(breaker.state().await, CircuitState::HalfOpen);

    // Need 3 successes total to close (success_threshold)
    let _ = breaker.execute(|| async { Ok::<_, String>(2) }).await;
    assert_eq!(breaker.state().await, CircuitState::HalfOpen);

    let _ = breaker.execute(|| async { Ok::<_, String>(3) }).await;
    assert_eq!(breaker.state().await, CircuitState::Closed);
}

#[tokio::test]
async fn test_zero_timeout_edge_case() {
    // Zero or very small timeout
    let result = with_timeout(Duration::from_nanos(1), async {
        // Even instant operations might timeout
        42
    })
    .await;

    // Result could be Ok or Err depending on scheduler
    // Just verify no panic occurs
    let _ = result;
}

#[tokio::test]
async fn test_retry_with_one_retry() {
    let policy = RetryPolicy::new(1);
    let counter = Arc::new(AtomicUsize::new(0));
    let counter_clone = Arc::clone(&counter);

    let result = policy
        .execute(|| {
            let c = Arc::clone(&counter_clone);
            async move {
                c.fetch_add(1, Ordering::SeqCst);
                Err::<(), _>("always fails")
            }
        })
        .await;

    assert!(result.is_err());
    assert_eq!(counter.load(Ordering::SeqCst), 1); // Only one attempt, no retries
}
