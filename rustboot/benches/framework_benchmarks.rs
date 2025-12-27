//! Rustboot Framework Performance Benchmarks
//!
//! Run with: cargo bench --bench framework_benchmarks
//!
//! These benchmarks measure the performance of core Rustboot components:
//! - Dependency Injection container operations
//! - Cache operations (read/write)
//! - Validation processing
//! - Serialization/deserialization
//! - Rate limiter throughput
//! - State machine transitions
//! - Resilience patterns (circuit breaker, retry)

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};
use std::sync::Arc;
use std::time::Duration;

// ============================================================================
// Dependency Injection Benchmarks
// ============================================================================

mod di_benchmarks {
    use super::*;
    use dev_engineeringlabs_rustboot_di::Container;

    #[derive(Clone)]
    struct SimpleService {
        value: i32,
    }

    #[derive(Clone)]
    struct ComplexService {
        name: String,
        config: Vec<String>,
        nested: SimpleService,
    }

    pub fn bench_di_operations(c: &mut Criterion) {
        let mut group = c.benchmark_group("DI Container");

        // Benchmark: Register service
        group.bench_function("register_simple", |b| {
            b.iter(|| {
                let container = Container::new();
                container.register(SimpleService { value: 42 });
                black_box(&container);
            })
        });

        // Benchmark: Resolve service
        group.bench_function("resolve_simple", |b| {
            let container = Container::new();
            container.register(SimpleService { value: 42 });

            b.iter(|| {
                let service = container.resolve::<SimpleService>();
                black_box(service);
            })
        });

        // Benchmark: Complex service registration
        group.bench_function("register_complex", |b| {
            b.iter(|| {
                let container = Container::new();
                container.register(ComplexService {
                    name: "test".to_string(),
                    config: vec!["a".to_string(), "b".to_string(), "c".to_string()],
                    nested: SimpleService { value: 100 },
                });
                black_box(&container);
            })
        });

        // Benchmark: Multiple resolutions
        group.bench_function("resolve_100x", |b| {
            let container = Container::new();
            container.register(SimpleService { value: 42 });

            b.iter(|| {
                for _ in 0..100 {
                    let service = container.resolve::<SimpleService>();
                    black_box(service);
                }
            })
        });

        group.finish();
    }
}

// ============================================================================
// Cache Benchmarks
// ============================================================================

mod cache_benchmarks {
    use super::*;
    use dev_engineeringlabs_rustboot_cache::InMemoryCache;
    use tokio::runtime::Runtime;

    pub fn bench_cache_operations(c: &mut Criterion) {
        let rt = Runtime::new().unwrap();
        let mut group = c.benchmark_group("Cache");

        // Benchmark: Cache set
        group.bench_function("set", |b| {
            let cache = Arc::new(InMemoryCache::<String, String>::new());

            b.iter(|| {
                rt.block_on(async {
                    cache
                        .set(
                            "key".to_string(),
                            "value".to_string(),
                            Some(Duration::from_secs(60)),
                        )
                        .await;
                });
            })
        });

        // Benchmark: Cache get (hit)
        group.bench_function("get_hit", |b| {
            let cache = Arc::new(InMemoryCache::<String, String>::new());
            rt.block_on(async {
                cache
                    .set("key".to_string(), "value".to_string(), None)
                    .await;
            });

            b.iter(|| {
                rt.block_on(async {
                    let value = cache.get(&"key".to_string()).await;
                    black_box(value);
                });
            })
        });

        // Benchmark: Cache get (miss)
        group.bench_function("get_miss", |b| {
            let cache = Arc::new(InMemoryCache::<String, String>::new());

            b.iter(|| {
                rt.block_on(async {
                    let value = cache.get(&"nonexistent".to_string()).await;
                    black_box(value);
                });
            })
        });

        // Benchmark: Mixed operations
        group.throughput(Throughput::Elements(1000));
        group.bench_function("mixed_1000_ops", |b| {
            let cache = Arc::new(InMemoryCache::<String, String>::new());

            b.iter(|| {
                rt.block_on(async {
                    for i in 0..1000 {
                        let key = format!("key_{}", i % 100);
                        if i % 3 == 0 {
                            cache.set(key, format!("value_{}", i), None).await;
                        } else {
                            cache.get(&key).await;
                        }
                    }
                });
            })
        });

        group.finish();
    }
}

// ============================================================================
// Validation Benchmarks
// ============================================================================

mod validation_benchmarks {
    use super::*;
    use dev_engineeringlabs_rustboot_validation::{ValidationError, Validator};

    #[derive(Clone)]
    struct SimpleValidation {
        name: String,
        age: u32,
    }

    impl Validator for SimpleValidation {
        fn validate(&self) -> Result<(), ValidationError> {
            if self.name.is_empty() {
                return Err(ValidationError::Field {
                    field: "name".to_string(),
                    message: "Name is required".to_string(),
                });
            }
            if self.age < 18 || self.age > 120 {
                return Err(ValidationError::Field {
                    field: "age".to_string(),
                    message: "Age must be between 18 and 120".to_string(),
                });
            }
            Ok(())
        }
    }

    #[derive(Clone)]
    struct ComplexValidation {
        username: String,
        email: String,
        password: String,
        age: u32,
        phone: String,
    }

    impl Validator for ComplexValidation {
        fn validate(&self) -> Result<(), ValidationError> {
            let mut errors = Vec::new();

            if self.username.len() < 3 || self.username.len() > 50 {
                errors.push("Username must be 3-50 characters".to_string());
            }
            if !self.email.contains('@') {
                errors.push("Invalid email".to_string());
            }
            if self.password.len() < 8 {
                errors.push("Password too short".to_string());
            }
            if self.age < 18 {
                errors.push("Must be 18+".to_string());
            }
            if self.phone.len() != 10 {
                errors.push("Invalid phone".to_string());
            }

            if errors.is_empty() {
                Ok(())
            } else {
                Err(ValidationError::Multiple(errors))
            }
        }
    }

    pub fn bench_validation(c: &mut Criterion) {
        let mut group = c.benchmark_group("Validation");

        // Benchmark: Simple validation (pass)
        group.bench_function("simple_pass", |b| {
            let data = SimpleValidation {
                name: "John".to_string(),
                age: 25,
            };

            b.iter(|| {
                let result = data.validate();
                black_box(result);
            })
        });

        // Benchmark: Simple validation (fail)
        group.bench_function("simple_fail", |b| {
            let data = SimpleValidation {
                name: "".to_string(),
                age: 10,
            };

            b.iter(|| {
                let result = data.validate();
                black_box(result);
            })
        });

        // Benchmark: Complex validation
        group.bench_function("complex_pass", |b| {
            let data = ComplexValidation {
                username: "johndoe".to_string(),
                email: "john@example.com".to_string(),
                password: "securepassword123".to_string(),
                age: 25,
                phone: "1234567890".to_string(),
            };

            b.iter(|| {
                let result = data.validate();
                black_box(result);
            })
        });

        // Benchmark: Batch validation
        group.throughput(Throughput::Elements(1000));
        group.bench_function("batch_1000", |b| {
            let items: Vec<SimpleValidation> = (0..1000)
                .map(|i| SimpleValidation {
                    name: format!("User{}", i),
                    age: 20 + (i % 80) as u32,
                })
                .collect();

            b.iter(|| {
                for item in &items {
                    let result = item.validate();
                    black_box(result);
                }
            })
        });

        group.finish();
    }
}

// ============================================================================
// Serialization Benchmarks
// ============================================================================

mod serialization_benchmarks {
    use super::*;
    use dev_engineeringlabs_rustboot_serialization::{JsonSerializer, Serializer};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Serialize, Deserialize)]
    struct SmallPayload {
        id: u64,
        name: String,
        active: bool,
    }

    #[derive(Clone, Serialize, Deserialize)]
    struct LargePayload {
        id: u64,
        name: String,
        description: String,
        tags: Vec<String>,
        metadata: std::collections::HashMap<String, String>,
        items: Vec<SmallPayload>,
    }

    pub fn bench_serialization(c: &mut Criterion) {
        let mut group = c.benchmark_group("Serialization");

        let serializer = JsonSerializer;

        // Small payload
        let small = SmallPayload {
            id: 1,
            name: "test".to_string(),
            active: true,
        };

        // Large payload
        let mut metadata = std::collections::HashMap::new();
        for i in 0..10 {
            metadata.insert(format!("key{}", i), format!("value{}", i));
        }

        let large = LargePayload {
            id: 1,
            name: "Large Object".to_string(),
            description: "A large payload with nested data".repeat(10),
            tags: (0..20).map(|i| format!("tag{}", i)).collect(),
            metadata,
            items: (0..50)
                .map(|i| SmallPayload {
                    id: i,
                    name: format!("Item {}", i),
                    active: i % 2 == 0,
                })
                .collect(),
        };

        // Benchmark: Serialize small
        group.bench_function("serialize_small", |b| {
            b.iter(|| {
                let result = serializer.serialize(&small);
                black_box(result);
            })
        });

        // Benchmark: Deserialize small
        let small_bytes = serializer.serialize(&small).unwrap();
        group.bench_function("deserialize_small", |b| {
            b.iter(|| {
                let result: SmallPayload = serializer.deserialize(&small_bytes).unwrap();
                black_box(result);
            })
        });

        // Benchmark: Serialize large
        group.bench_function("serialize_large", |b| {
            b.iter(|| {
                let result = serializer.serialize(&large);
                black_box(result);
            })
        });

        // Benchmark: Deserialize large
        let large_bytes = serializer.serialize(&large).unwrap();
        group.bench_function("deserialize_large", |b| {
            b.iter(|| {
                let result: LargePayload = serializer.deserialize(&large_bytes).unwrap();
                black_box(result);
            })
        });

        group.finish();
    }
}

// ============================================================================
// Rate Limiter Benchmarks
// ============================================================================

mod ratelimit_benchmarks {
    use super::*;
    use dev_engineeringlabs_rustboot_ratelimit::{RateLimiter, SlidingWindowLimiter};

    pub fn bench_rate_limiter(c: &mut Criterion) {
        let mut group = c.benchmark_group("Rate Limiter");

        // Benchmark: Try acquire (under limit)
        group.bench_function("try_acquire_pass", |b| {
            let limiter = SlidingWindowLimiter::new(10000, Duration::from_secs(1));

            b.iter(|| {
                let result = limiter.try_acquire();
                black_box(result);
            })
        });

        // Benchmark: Check limit status
        group.bench_function("remaining", |b| {
            let limiter = SlidingWindowLimiter::new(10000, Duration::from_secs(1));

            b.iter(|| {
                let remaining = limiter.remaining();
                black_box(remaining);
            })
        });

        // Benchmark: High throughput scenario
        group.throughput(Throughput::Elements(10000));
        group.bench_function("throughput_10k", |b| {
            let limiter = SlidingWindowLimiter::new(100000, Duration::from_secs(1));

            b.iter(|| {
                for _ in 0..10000 {
                    let result = limiter.try_acquire();
                    black_box(result);
                }
            })
        });

        group.finish();
    }
}

// ============================================================================
// State Machine Benchmarks
// ============================================================================

mod state_machine_benchmarks {
    use super::*;
    use dev_engineeringlabs_rustboot_state_machine::StateMachine;

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    enum OrderState {
        Pending,
        Confirmed,
        Processing,
        Shipped,
        Delivered,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    enum OrderEvent {
        Confirm,
        Process,
        Ship,
        Deliver,
    }

    fn create_order_machine() -> StateMachine<OrderState, OrderEvent> {
        let mut sm = StateMachine::new(OrderState::Pending);
        sm.add_transition(OrderState::Pending, OrderEvent::Confirm, OrderState::Confirmed);
        sm.add_transition(OrderState::Confirmed, OrderEvent::Process, OrderState::Processing);
        sm.add_transition(OrderState::Processing, OrderEvent::Ship, OrderState::Shipped);
        sm.add_transition(OrderState::Shipped, OrderEvent::Deliver, OrderState::Delivered);
        sm
    }

    pub fn bench_state_machine(c: &mut Criterion) {
        let mut group = c.benchmark_group("State Machine");

        // Benchmark: Create state machine
        group.bench_function("create", |b| {
            b.iter(|| {
                let sm = create_order_machine();
                black_box(sm);
            })
        });

        // Benchmark: Single transition
        group.bench_function("single_transition", |b| {
            b.iter(|| {
                let mut sm = create_order_machine();
                let result = sm.trigger(OrderEvent::Confirm);
                black_box(result);
            })
        });

        // Benchmark: Full workflow
        group.bench_function("full_workflow", |b| {
            b.iter(|| {
                let mut sm = create_order_machine();
                sm.trigger(OrderEvent::Confirm).unwrap();
                sm.trigger(OrderEvent::Process).unwrap();
                sm.trigger(OrderEvent::Ship).unwrap();
                sm.trigger(OrderEvent::Deliver).unwrap();
                black_box(sm.current_state());
            })
        });

        // Benchmark: Can trigger check
        group.bench_function("can_trigger", |b| {
            let sm = create_order_machine();

            b.iter(|| {
                let result = sm.can_trigger(&OrderEvent::Confirm);
                black_box(result);
            })
        });

        group.finish();
    }
}

// ============================================================================
// Resilience Benchmarks
// ============================================================================

mod resilience_benchmarks {
    use super::*;
    use dev_engineeringlabs_rustboot_resilience::{CircuitBreaker, CircuitBreakerConfig, RetryPolicy};
    use tokio::runtime::Runtime;

    pub fn bench_resilience(c: &mut Criterion) {
        let rt = Runtime::new().unwrap();
        let mut group = c.benchmark_group("Resilience");

        // Circuit breaker config
        let cb_config = CircuitBreakerConfig {
            failure_threshold: 5,
            timeout: Duration::from_secs(30),
            success_threshold: 2,
        };

        // Benchmark: Circuit breaker execute (closed)
        group.bench_function("circuit_breaker_closed", |b| {
            let cb = CircuitBreaker::new(cb_config.clone());

            b.iter(|| {
                rt.block_on(async {
                    let result = cb.execute(|| async { Ok::<_, String>("success") }).await;
                    black_box(result);
                });
            })
        });

        // Benchmark: Retry policy (immediate success)
        group.bench_function("retry_immediate_success", |b| {
            let retry = RetryPolicy::new(3);

            b.iter(|| {
                rt.block_on(async {
                    let result = retry
                        .execute(|| async { Ok::<_, String>("success") })
                        .await;
                    black_box(result);
                });
            })
        });

        // Benchmark: Circuit breaker state check
        group.bench_function("circuit_breaker_state", |b| {
            let cb = CircuitBreaker::new(cb_config.clone());

            b.iter(|| {
                rt.block_on(async {
                    let state = cb.state().await;
                    black_box(state);
                });
            })
        });

        group.finish();
    }
}

// ============================================================================
// Criterion Configuration
// ============================================================================

criterion_group!(
    benches,
    di_benchmarks::bench_di_operations,
    cache_benchmarks::bench_cache_operations,
    validation_benchmarks::bench_validation,
    serialization_benchmarks::bench_serialization,
    ratelimit_benchmarks::bench_rate_limiter,
    state_machine_benchmarks::bench_state_machine,
    resilience_benchmarks::bench_resilience,
);

criterion_main!(benches);
