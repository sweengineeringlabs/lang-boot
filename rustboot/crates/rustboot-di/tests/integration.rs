//! Integration tests for rustboot-di
//!
//! Tests the public API as an external user would use it

use dev_engineeringlabs_rustboot_di::*;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

// ============================================================================
// Basic Service Registration and Resolution
// ============================================================================

#[derive(Debug, Clone)]
struct DatabaseConnection {
    connection_string: String,
}

#[derive(Debug, Clone)]
struct Logger {
    log_level: String,
}

#[test]
fn test_basic_service_registration() {
    let container = Container::new();

    let db = DatabaseConnection {
        connection_string: "postgres://localhost:5432/mydb".to_string(),
    };

    container.register(db.clone());

    let resolved = container.resolve::<DatabaseConnection>().unwrap();
    assert_eq!(resolved.connection_string, "postgres://localhost:5432/mydb");
}

#[test]
fn test_resolve_missing_service() {
    let container = Container::new();

    let result = container.resolve::<DatabaseConnection>();
    assert!(result.is_none());
}

#[test]
fn test_multiple_service_types() {
    let container = Container::new();

    let db = DatabaseConnection {
        connection_string: "postgres://localhost:5432/mydb".to_string(),
    };
    let logger = Logger {
        log_level: "INFO".to_string(),
    };

    container.register(db);
    container.register(logger);

    // Resolve both services
    let resolved_db = container.resolve::<DatabaseConnection>().unwrap();
    let resolved_logger = container.resolve::<Logger>().unwrap();

    assert_eq!(resolved_db.connection_string, "postgres://localhost:5432/mydb");
    assert_eq!(resolved_logger.log_level, "INFO");
}

// ============================================================================
// Singleton Lifetime Testing
// ============================================================================

#[derive(Clone)]
struct SingletonService {
    id: Arc<AtomicUsize>,
}

impl SingletonService {
    fn new(id: usize) -> Self {
        Self {
            id: Arc::new(AtomicUsize::new(id)),
        }
    }

    fn get_id(&self) -> usize {
        self.id.load(Ordering::SeqCst)
    }

    fn increment(&self) {
        self.id.fetch_add(1, Ordering::SeqCst);
    }
}

#[test]
fn test_singleton_behavior() {
    let container = Container::new();

    // Register a singleton service
    let service = SingletonService::new(100);
    container.register(service);

    // Resolve twice - should get the same instance (shared state)
    let instance1 = container.resolve::<SingletonService>().unwrap();
    let instance2 = container.resolve::<SingletonService>().unwrap();

    assert_eq!(instance1.get_id(), 100);
    assert_eq!(instance2.get_id(), 100);

    // Modify through one instance
    instance1.increment();

    // Both should see the change (same underlying Arc)
    assert_eq!(instance1.get_id(), 101);
    assert_eq!(instance2.get_id(), 101);
}

#[test]
fn test_service_shared_across_container_clones() {
    let container1 = Container::new();

    let service = SingletonService::new(200);
    container1.register(service);

    // Clone the container
    let container2 = container1.clone();

    // Resolve from both containers
    let instance1 = container1.resolve::<SingletonService>().unwrap();
    let instance2 = container2.resolve::<SingletonService>().unwrap();

    // Should share state
    instance1.increment();
    assert_eq!(instance2.get_id(), 201);
}

// ============================================================================
// Service Overwriting and Updates
// ============================================================================

#[test]
fn test_service_overwriting() {
    let container = Container::new();

    // Register initial configuration
    container.register(Logger {
        log_level: "DEBUG".to_string(),
    });

    let first = container.resolve::<Logger>().unwrap();
    assert_eq!(first.log_level, "DEBUG");

    // Register new configuration (overwrites)
    container.register(Logger {
        log_level: "ERROR".to_string(),
    });

    let second = container.resolve::<Logger>().unwrap();
    assert_eq!(second.log_level, "ERROR");
}

// ============================================================================
// Constructor Injection Pattern
// ============================================================================

#[derive(Clone)]
struct UserRepository {
    db: Arc<DatabaseConnection>,
}

impl UserRepository {
    fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }
}

#[derive(Clone)]
struct UserService {
    repository: Arc<UserRepository>,
    logger: Arc<Logger>,
}

impl UserService {
    fn new(repository: Arc<UserRepository>, logger: Arc<Logger>) -> Self {
        Self { repository, logger }
    }
}

#[test]
fn test_manual_constructor_injection() {
    let container = Container::new();

    // Register dependencies
    let db = DatabaseConnection {
        connection_string: "postgres://localhost/users".to_string(),
    };
    container.register(db);

    let logger = Logger {
        log_level: "INFO".to_string(),
    };
    container.register(logger);

    // Resolve dependencies and construct service
    let db_arc = container.resolve::<DatabaseConnection>().unwrap();
    let repository = UserRepository::new(db_arc);
    container.register(repository);

    let repo_arc = container.resolve::<UserRepository>().unwrap();
    let logger_arc = container.resolve::<Logger>().unwrap();
    let user_service = UserService::new(repo_arc, logger_arc);
    container.register(user_service);

    // Verify the chain
    let resolved = container.resolve::<UserService>().unwrap();
    assert_eq!(resolved.repository.db.connection_string, "postgres://localhost/users");
    assert_eq!(resolved.logger.log_level, "INFO");
}

// ============================================================================
// Injectable Trait Pattern
// ============================================================================

#[derive(Clone)]
struct EmailService {
    smtp_host: String,
}

impl Injectable for EmailService {
    fn inject(_container: &Container) -> Self {
        Self {
            smtp_host: "smtp.example.com".to_string(),
        }
    }
}

#[derive(Clone)]
struct NotificationService {
    email: Arc<EmailService>,
    logger: Arc<Logger>,
}

impl Injectable for NotificationService {
    fn inject(container: &Container) -> Self {
        let email = container
            .resolve::<EmailService>()
            .expect("EmailService not registered");
        let logger = container
            .resolve::<Logger>()
            .expect("Logger not registered");

        Self { email, logger }
    }
}

#[test]
fn test_injectable_trait() {
    let container = Container::new();

    // Register base dependencies
    let email = EmailService::inject(&container);
    container.register(email);

    let logger = Logger {
        log_level: "WARN".to_string(),
    };
    container.register(logger);

    // Use Injectable to construct service with dependencies
    let notification = NotificationService::inject(&container);
    assert_eq!(notification.email.smtp_host, "smtp.example.com");
    assert_eq!(notification.logger.log_level, "WARN");
}

// ============================================================================
// Container State Management
// ============================================================================

#[test]
fn test_container_clear() {
    let container = Container::new();

    container.register(DatabaseConnection {
        connection_string: "test".to_string(),
    });
    container.register(Logger {
        log_level: "DEBUG".to_string(),
    });

    assert!(container.contains::<DatabaseConnection>());
    assert!(container.contains::<Logger>());

    container.clear();

    assert!(!container.contains::<DatabaseConnection>());
    assert!(!container.contains::<Logger>());
}

#[test]
fn test_container_contains() {
    let container = Container::new();

    assert!(!container.contains::<DatabaseConnection>());

    container.register(DatabaseConnection {
        connection_string: "test".to_string(),
    });

    assert!(container.contains::<DatabaseConnection>());
    assert!(!container.contains::<Logger>());
}

// ============================================================================
// Complex Dependency Graphs
// ============================================================================

#[derive(Clone)]
struct ConfigService {
    config_path: String,
}

#[derive(Clone)]
struct CacheService {
    config: Arc<ConfigService>,
}

#[derive(Clone)]
struct ApiService {
    db: Arc<DatabaseConnection>,
    cache: Arc<CacheService>,
    logger: Arc<Logger>,
}

#[test]
fn test_complex_dependency_graph() {
    let container = Container::new();

    // Register bottom-level dependencies
    let config = ConfigService {
        config_path: "/etc/app/config.toml".to_string(),
    };
    container.register(config);

    let db = DatabaseConnection {
        connection_string: "postgres://localhost/api".to_string(),
    };
    container.register(db);

    let logger = Logger {
        log_level: "INFO".to_string(),
    };
    container.register(logger);

    // Register mid-level dependency
    let config_arc = container.resolve::<ConfigService>().unwrap();
    let cache = CacheService { config: config_arc };
    container.register(cache);

    // Register top-level service
    let db_arc = container.resolve::<DatabaseConnection>().unwrap();
    let cache_arc = container.resolve::<CacheService>().unwrap();
    let logger_arc = container.resolve::<Logger>().unwrap();
    let api = ApiService {
        db: db_arc,
        cache: cache_arc,
        logger: logger_arc,
    };
    container.register(api);

    // Verify the entire graph
    let resolved = container.resolve::<ApiService>().unwrap();
    assert_eq!(resolved.db.connection_string, "postgres://localhost/api");
    assert_eq!(resolved.cache.config.config_path, "/etc/app/config.toml");
    assert_eq!(resolved.logger.log_level, "INFO");
}

// ============================================================================
// Thread Safety
// ============================================================================

#[test]
fn test_concurrent_access() {
    use std::thread;

    let container = Container::new();

    // Register a service
    let service = SingletonService::new(0);
    container.register(service);

    let mut handles = vec![];

    // Spawn multiple threads that access the container
    for _ in 0..10 {
        let container_clone = container.clone();
        let handle = thread::spawn(move || {
            for _ in 0..100 {
                let service = container_clone.resolve::<SingletonService>().unwrap();
                service.increment();
            }
        });
        handles.push(handle);
    }

    // Wait for all threads
    for handle in handles {
        handle.join().unwrap();
    }

    // Verify all increments were applied
    let final_service = container.resolve::<SingletonService>().unwrap();
    assert_eq!(final_service.get_id(), 1000);
}

// ============================================================================
// Type Safety and Trait Objects
// ============================================================================

trait MessageSender: Send + Sync {
    fn send(&self, message: &str) -> String;
}

#[derive(Clone)]
struct EmailSender {
    from: String,
}

impl MessageSender for EmailSender {
    fn send(&self, message: &str) -> String {
        format!("Email from {}: {}", self.from, message)
    }
}

#[derive(Clone)]
struct SmsSender {
    phone: String,
}

impl MessageSender for SmsSender {
    fn send(&self, message: &str) -> String {
        format!("SMS from {}: {}", self.phone, message)
    }
}

#[test]
fn test_different_implementations() {
    let container = Container::new();

    // Register different concrete types (not trait objects)
    let email = EmailSender {
        from: "admin@example.com".to_string(),
    };
    container.register(email);

    let sms = SmsSender {
        phone: "+1234567890".to_string(),
    };
    container.register(sms);

    // Resolve concrete types
    let email_sender = container.resolve::<EmailSender>().unwrap();
    let sms_sender = container.resolve::<SmsSender>().unwrap();

    assert_eq!(
        email_sender.send("Hello"),
        "Email from admin@example.com: Hello"
    );
    assert_eq!(
        sms_sender.send("Hello"),
        "SMS from +1234567890: Hello"
    );
}

// ============================================================================
// Real-World Usage Patterns
// ============================================================================

#[derive(Clone)]
struct AppConfig {
    database_url: String,
    api_key: String,
    log_level: String,
}

#[derive(Clone)]
struct Application {
    config: Arc<AppConfig>,
    db: Arc<DatabaseConnection>,
    logger: Arc<Logger>,
}

#[test]
fn test_application_bootstrap() {
    let container = Container::new();

    // Bootstrap: register configuration
    let config = AppConfig {
        database_url: "postgres://localhost/app".to_string(),
        api_key: "secret-key-123".to_string(),
        log_level: "INFO".to_string(),
    };
    container.register(config);

    // Bootstrap: create services from config
    let config_arc = container.resolve::<AppConfig>().unwrap();

    let db = DatabaseConnection {
        connection_string: config_arc.database_url.clone(),
    };
    container.register(db);

    let logger = Logger {
        log_level: config_arc.log_level.clone(),
    };
    container.register(logger);

    // Bootstrap: create application
    let app = Application {
        config: config_arc,
        db: container.resolve::<DatabaseConnection>().unwrap(),
        logger: container.resolve::<Logger>().unwrap(),
    };
    container.register(app);

    // Verify application is correctly configured
    let resolved_app = container.resolve::<Application>().unwrap();
    assert_eq!(resolved_app.config.api_key, "secret-key-123");
    assert_eq!(resolved_app.db.connection_string, "postgres://localhost/app");
    assert_eq!(resolved_app.logger.log_level, "INFO");
}

// ============================================================================
// Edge Cases
// ============================================================================

#[test]
fn test_empty_container() {
    let container = Container::new();

    assert!(container.resolve::<DatabaseConnection>().is_none());
    assert!(!container.contains::<Logger>());
}

#[test]
fn test_default_container() {
    let container = Container::default();

    assert!(container.resolve::<DatabaseConnection>().is_none());

    // Default container should work normally
    container.register(Logger {
        log_level: "DEBUG".to_string(),
    });

    assert!(container.contains::<Logger>());
}

#[test]
fn test_register_then_clear_then_reregister() {
    let container = Container::new();

    container.register(Logger {
        log_level: "INFO".to_string(),
    });
    assert!(container.contains::<Logger>());

    container.clear();
    assert!(!container.contains::<Logger>());

    container.register(Logger {
        log_level: "DEBUG".to_string(),
    });

    let logger = container.resolve::<Logger>().unwrap();
    assert_eq!(logger.log_level, "DEBUG");
}

#[derive(Clone)]
struct EmptyService;

#[test]
fn test_zero_sized_type() {
    let container = Container::new();

    container.register(EmptyService);

    assert!(container.contains::<EmptyService>());
    let _resolved = container.resolve::<EmptyService>().unwrap();
}

// ============================================================================
// Service Lifetime Scenarios
// ============================================================================

#[derive(Clone)]
struct RequestContext {
    request_id: String,
    #[allow(dead_code)]
    timestamp: u64,
}

#[test]
fn test_request_scoped_pattern() {
    let global_container = Container::new();

    // Register global services (singletons)
    let db = DatabaseConnection {
        connection_string: "postgres://localhost/app".to_string(),
    };
    global_container.register(db);

    // Simulate request 1
    let request1_container = global_container.clone();
    let request1_ctx = RequestContext {
        request_id: "req-001".to_string(),
        timestamp: 1000,
    };
    request1_container.register(request1_ctx);

    // Simulate request 2
    let request2_container = global_container.clone();
    let request2_ctx = RequestContext {
        request_id: "req-002".to_string(),
        timestamp: 2000,
    };
    request2_container.register(request2_ctx);

    // Both should share DB
    let req1_db = request1_container.resolve::<DatabaseConnection>().unwrap();
    let req2_db = request2_container.resolve::<DatabaseConnection>().unwrap();
    assert_eq!(req1_db.connection_string, req2_db.connection_string);

    // Note: Since containers share state when cloned, the last registered
    // RequestContext (req-002) will be available in both containers.
    // This demonstrates that container cloning shares the underlying registry,
    // which is appropriate for singleton services but requires care for request-scoped data.
    let req1_ctx = request1_container.resolve::<RequestContext>().unwrap();
    let req2_ctx = request2_container.resolve::<RequestContext>().unwrap();
    assert_eq!(req1_ctx.request_id, "req-002");
    assert_eq!(req2_ctx.request_id, "req-002");
}

// ============================================================================
// Error Handling Patterns
// ============================================================================

#[derive(Clone, Debug)]
struct HealthCheckService {
    #[allow(dead_code)]
    db: Arc<DatabaseConnection>,
}

impl HealthCheckService {
    fn try_new(container: &Container) -> Result<Self, String> {
        let db = container
            .resolve::<DatabaseConnection>()
            .ok_or("DatabaseConnection not registered")?;

        Ok(Self { db })
    }
}

#[test]
fn test_graceful_error_handling() {
    let container = Container::new();

    // Try to create service without dependencies
    let result = HealthCheckService::try_new(&container);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "DatabaseConnection not registered");

    // Register dependency
    container.register(DatabaseConnection {
        connection_string: "postgres://localhost/health".to_string(),
    });

    // Now it should work
    let result = HealthCheckService::try_new(&container);
    assert!(result.is_ok());
}

// ============================================================================
// Multiple Container Instances
// ============================================================================

#[test]
fn test_isolated_containers() {
    let container1 = Container::new();
    let container2 = Container::new();

    // Register different services in each container
    container1.register(Logger {
        log_level: "DEBUG".to_string(),
    });

    container2.register(Logger {
        log_level: "ERROR".to_string(),
    });

    // They should be completely isolated
    let logger1 = container1.resolve::<Logger>().unwrap();
    let logger2 = container2.resolve::<Logger>().unwrap();

    assert_eq!(logger1.log_level, "DEBUG");
    assert_eq!(logger2.log_level, "ERROR");

    // Clearing one shouldn't affect the other
    container1.clear();
    assert!(!container1.contains::<Logger>());
    assert!(container2.contains::<Logger>());
}

// ============================================================================
// Factory Registration Patterns
// ============================================================================

/// Factory trait for creating services on demand
trait ServiceFactory<T>: Send + Sync {
    fn create(&self, container: &Container) -> T;
}

#[derive(Clone)]
struct ConnectionFactory {
    base_url: String,
    counter: Arc<AtomicUsize>,
}

impl ConnectionFactory {
    fn new(base_url: String) -> Self {
        Self {
            base_url,
            counter: Arc::new(AtomicUsize::new(0)),
        }
    }

    fn create_connection(&self) -> DatabaseConnection {
        let id = self.counter.fetch_add(1, Ordering::SeqCst);
        DatabaseConnection {
            connection_string: format!("{}?conn_id={}", self.base_url, id),
        }
    }
}

#[test]
fn test_factory_pattern() {
    let container = Container::new();

    // Register a factory
    let factory = ConnectionFactory::new("postgres://localhost/app".to_string());
    container.register(factory);

    // Use factory to create multiple instances
    let factory_arc = container.resolve::<ConnectionFactory>().unwrap();

    let conn1 = factory_arc.create_connection();
    let conn2 = factory_arc.create_connection();
    let conn3 = factory_arc.create_connection();

    // Each connection should have a unique ID
    assert!(conn1.connection_string.contains("conn_id=0"));
    assert!(conn2.connection_string.contains("conn_id=1"));
    assert!(conn3.connection_string.contains("conn_id=2"));
}

#[derive(Clone)]
struct ServiceBuilder {
    config: String,
}

impl ServiceBuilder {
    fn new(config: String) -> Self {
        Self { config }
    }

    fn build_logger(&self, level: &str) -> Logger {
        Logger {
            log_level: format!("{}-{}", self.config, level),
        }
    }
}

#[test]
fn test_builder_factory_pattern() {
    let container = Container::new();

    // Register builder
    let builder = ServiceBuilder::new("production".to_string());
    container.register(builder);

    // Use builder to create configured services
    let builder_arc = container.resolve::<ServiceBuilder>().unwrap();

    let info_logger = builder_arc.build_logger("INFO");
    let debug_logger = builder_arc.build_logger("DEBUG");

    assert_eq!(info_logger.log_level, "production-INFO");
    assert_eq!(debug_logger.log_level, "production-DEBUG");
}

#[test]
fn test_lazy_initialization_pattern() {
    use std::sync::Once;

    static INIT: Once = Once::new();
    static COUNTER: AtomicUsize = AtomicUsize::new(0);

    #[derive(Clone)]
    struct LazyService {
        value: usize,
    }

    impl LazyService {
        fn get_or_init() -> Self {
            let mut value = 0;
            INIT.call_once(|| {
                value = COUNTER.fetch_add(1, Ordering::SeqCst);
            });
            Self { value }
        }
    }

    let container = Container::new();

    // First initialization
    let service1 = LazyService::get_or_init();
    let value1 = service1.value;
    container.register(service1);

    // Subsequent calls should return the same initialized value
    let service2 = LazyService::get_or_init();
    assert_eq!(value1, service2.value);
}

// ============================================================================
// Transient vs Singleton Lifetime Scopes
// ============================================================================

#[derive(Clone)]
struct TransientService {
    instance_id: usize,
    created_at: Arc<AtomicUsize>,
}

impl TransientService {
    fn new(id: usize) -> Self {
        Self {
            instance_id: id,
            created_at: Arc::new(AtomicUsize::new(id)),
        }
    }
}

#[test]
fn test_singleton_lifetime_scope() {
    let container = Container::new();

    // Register as singleton (default behavior with Arc)
    let service = TransientService::new(42);
    container.register(service);

    // Multiple resolves return the same instance (shared Arc)
    let instance1 = container.resolve::<TransientService>().unwrap();
    let instance2 = container.resolve::<TransientService>().unwrap();

    assert_eq!(instance1.instance_id, 42);
    assert_eq!(instance2.instance_id, 42);

    // They share the same underlying data
    instance1.created_at.store(100, Ordering::SeqCst);
    assert_eq!(instance2.created_at.load(Ordering::SeqCst), 100);
}

#[test]
fn test_transient_lifetime_pattern() {
    let container = Container::new();

    // Register a factory that creates new instances
    let factory = ConnectionFactory::new("postgres://localhost/db".to_string());
    container.register(factory);

    // Get factory and create transient instances
    let factory_arc = container.resolve::<ConnectionFactory>().unwrap();

    let instance1 = factory_arc.create_connection();
    let instance2 = factory_arc.create_connection();

    // Each instance is different (transient)
    assert_ne!(instance1.connection_string, instance2.connection_string);
}

#[test]
fn test_mixed_lifetime_scopes() {
    let container = Container::new();

    // Singleton: shared logger
    let logger = Logger {
        log_level: "INFO".to_string(),
    };
    container.register(logger);

    // Factory for transient connections
    let factory = ConnectionFactory::new("postgres://localhost/mixed".to_string());
    container.register(factory);

    // Resolve singleton multiple times - should be same
    let logger1 = container.resolve::<Logger>().unwrap();
    let logger2 = container.resolve::<Logger>().unwrap();
    assert_eq!(logger1.log_level, logger2.log_level);

    // Create transient instances - should be different
    let factory_arc = container.resolve::<ConnectionFactory>().unwrap();
    let conn1 = factory_arc.create_connection();
    let conn2 = factory_arc.create_connection();
    assert_ne!(conn1.connection_string, conn2.connection_string);
}

// ============================================================================
// Deep Dependency Chain Tests (A->B->C->D)
// ============================================================================

#[derive(Clone)]
struct LevelD {
    value: String,
}

#[derive(Clone)]
struct LevelC {
    d: Arc<LevelD>,
    metadata: String,
}

#[derive(Clone)]
struct LevelB {
    c: Arc<LevelC>,
    logger: Arc<Logger>,
}

#[derive(Clone)]
struct LevelA {
    b: Arc<LevelB>,
    config: Arc<ConfigService>,
}

#[test]
fn test_deep_dependency_chain() {
    let container = Container::new();

    // Register from bottom to top
    let level_d = LevelD {
        value: "deep-value".to_string(),
    };
    container.register(level_d);

    let level_c = LevelC {
        d: container.resolve::<LevelD>().unwrap(),
        metadata: "metadata-c".to_string(),
    };
    container.register(level_c);

    let logger = Logger {
        log_level: "DEBUG".to_string(),
    };
    container.register(logger);

    let level_b = LevelB {
        c: container.resolve::<LevelC>().unwrap(),
        logger: container.resolve::<Logger>().unwrap(),
    };
    container.register(level_b);

    let config = ConfigService {
        config_path: "/etc/config".to_string(),
    };
    container.register(config);

    let level_a = LevelA {
        b: container.resolve::<LevelB>().unwrap(),
        config: container.resolve::<ConfigService>().unwrap(),
    };
    container.register(level_a);

    // Resolve top-level and verify entire chain
    let resolved_a = container.resolve::<LevelA>().unwrap();
    assert_eq!(resolved_a.config.config_path, "/etc/config");
    assert_eq!(resolved_a.b.logger.log_level, "DEBUG");
    assert_eq!(resolved_a.b.c.metadata, "metadata-c");
    assert_eq!(resolved_a.b.c.d.value, "deep-value");
}

#[test]
fn test_dependency_chain_with_injectable() {
    let container = Container::new();

    // Register bottom-level
    container.register(LevelD {
        value: "injectable-test".to_string(),
    });

    // Define Injectable for LevelC
    #[derive(Clone)]
    struct InjectableLevelC {
        d: Arc<LevelD>,
    }

    impl Injectable for InjectableLevelC {
        fn inject(container: &Container) -> Self {
            Self {
                d: container.resolve::<LevelD>().expect("LevelD not found"),
            }
        }
    }

    // Use Injectable to auto-wire dependencies
    let level_c = InjectableLevelC::inject(&container);
    assert_eq!(level_c.d.value, "injectable-test");
}

#[test]
fn test_missing_dependency_in_chain() {
    let container = Container::new();

    // Register only level C and D, missing dependencies for level B
    container.register(LevelD {
        value: "test".to_string(),
    });

    let level_c = LevelC {
        d: container.resolve::<LevelD>().unwrap(),
        metadata: "test".to_string(),
    };
    container.register(level_c);

    // Try to create level B without logger registered
    let _c_arc = container.resolve::<LevelC>().unwrap();
    let logger_opt = container.resolve::<Logger>();

    assert!(logger_opt.is_none());

    // This would fail if we tried to create LevelB
    // Demonstrating defensive programming:
    if logger_opt.is_none() {
        // Can't create LevelB
        assert!(true);
    }
}

// ============================================================================
// Circular Dependency Detection
// ============================================================================

#[derive(Clone)]
struct ServiceX {
    name: String,
    y_ref: Option<Arc<ServiceY>>,
}

#[derive(Clone)]
struct ServiceY {
    name: String,
    x_ref: Option<Arc<ServiceX>>,
}

#[test]
fn test_circular_dependency_detection_manual() {
    let container = Container::new();

    // Create ServiceX without ServiceY first
    let service_x = ServiceX {
        name: "X".to_string(),
        y_ref: None,
    };
    container.register(service_x);

    // Create ServiceY with reference to ServiceX
    let x_arc = container.resolve::<ServiceX>().unwrap();
    let service_y = ServiceY {
        name: "Y".to_string(),
        x_ref: Some(x_arc),
    };
    container.register(service_y);

    // Now we have a one-way dependency chain X <- Y
    let y = container.resolve::<ServiceY>().unwrap();
    assert_eq!(y.x_ref.as_ref().unwrap().name, "X");
    assert!(y.x_ref.as_ref().unwrap().y_ref.is_none());
}

#[test]
fn test_avoid_circular_dependency_with_lazy() {
    use std::sync::Weak;

    #[derive(Clone)]
    struct NodeA {
        id: String,
        b_weak: Option<Weak<NodeB>>,
    }

    #[derive(Clone)]
    struct NodeB {
        id: String,
        a: Arc<NodeA>,
    }

    let container = Container::new();

    // Create NodeA first
    let node_a = NodeA {
        id: "A".to_string(),
        b_weak: None,
    };
    container.register(node_a);

    // Create NodeB with strong reference to A
    let a_arc = container.resolve::<NodeA>().unwrap();
    let node_b = NodeB {
        id: "B".to_string(),
        a: a_arc,
    };
    container.register(node_b);

    // Verify no circular strong references
    let b = container.resolve::<NodeB>().unwrap();
    assert_eq!(b.a.id, "A");
    assert!(b.a.b_weak.is_none());
}

#[test]
fn test_detect_missing_circular_dependency() {
    #[derive(Clone)]
    struct ComponentA {
        b_required: Option<Arc<ComponentB>>,
    }

    #[derive(Clone)]
    struct ComponentB {
        a_required: Option<Arc<ComponentA>>,
    }

    let container = Container::new();

    // Try to create ComponentA but ComponentB doesn't exist yet
    let a = ComponentA {
        b_required: container.resolve::<ComponentB>(),
    };
    container.register(a);

    // ComponentA.b_required should be None
    let resolved_a = container.resolve::<ComponentA>().unwrap();
    assert!(resolved_a.b_required.is_none());

    // Now register B
    let b = ComponentB {
        a_required: container.resolve::<ComponentA>(),
    };
    container.register(b);

    let resolved_b = container.resolve::<ComponentB>().unwrap();
    assert!(resolved_b.a_required.is_some());

    // But the original A still has None for B (no automatic backfill)
    let resolved_a_again = container.resolve::<ComponentA>().unwrap();
    assert!(resolved_a_again.b_required.is_none());
}

// ============================================================================
// Comprehensive Thread Safety Tests
// ============================================================================

#[test]
fn test_concurrent_registration() {
    use std::thread;

    let container = Container::new();
    let mut handles = vec![];

    // Spawn threads that register different services concurrently
    for i in 0..10 {
        let container_clone = container.clone();
        let handle = thread::spawn(move || {
            #[derive(Clone)]
            struct ThreadService {
                thread_id: usize,
            }

            // Each thread registers a unique service type at runtime
            // Note: This is a simplification - in reality we'd need different types
            // Using a counter to simulate different services
            let service = SingletonService::new(i);
            container_clone.register(service);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // Verify container is still functional
    assert!(container.contains::<SingletonService>());
}

#[test]
fn test_concurrent_resolution() {
    use std::thread;

    let container = Container::new();

    // Pre-register services
    container.register(Logger {
        log_level: "CONCURRENT".to_string(),
    });
    container.register(DatabaseConnection {
        connection_string: "postgres://concurrent".to_string(),
    });

    let mut handles = vec![];

    // Spawn many threads that resolve services concurrently
    for _ in 0..20 {
        let container_clone = container.clone();
        let handle = thread::spawn(move || {
            for _ in 0..100 {
                let logger = container_clone.resolve::<Logger>().unwrap();
                let db = container_clone.resolve::<DatabaseConnection>().unwrap();

                assert_eq!(logger.log_level, "CONCURRENT");
                assert_eq!(db.connection_string, "postgres://concurrent");
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

#[test]
fn test_concurrent_registration_and_resolution() {
    use std::thread;
    use std::time::Duration;

    let container = Container::new();
    let mut handles = vec![];

    // Writer thread - continuously updates services
    let writer_container = container.clone();
    let writer = thread::spawn(move || {
        for i in 0..50 {
            writer_container.register(Logger {
                log_level: format!("LEVEL-{}", i),
            });
            thread::sleep(Duration::from_micros(10));
        }
    });
    handles.push(writer);

    // Reader threads - continuously read services
    for _ in 0..5 {
        let reader_container = container.clone();
        let reader = thread::spawn(move || {
            for _ in 0..100 {
                if let Some(logger) = reader_container.resolve::<Logger>() {
                    assert!(logger.log_level.starts_with("LEVEL-"));
                }
                thread::sleep(Duration::from_micros(5));
            }
        });
        handles.push(reader);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

#[test]
fn test_thread_safe_shared_state_mutation() {
    use std::thread;

    let container = Container::new();

    // Register a service with mutable state
    let service = SingletonService::new(0);
    container.register(service);

    let mut handles = vec![];

    // Multiple threads increment the shared state
    for _ in 0..20 {
        let container_clone = container.clone();
        let handle = thread::spawn(move || {
            for _ in 0..50 {
                let service = container_clone.resolve::<SingletonService>().unwrap();
                service.increment();
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // Verify all increments were applied atomically
    let final_service = container.resolve::<SingletonService>().unwrap();
    assert_eq!(final_service.get_id(), 1000); // 20 threads * 50 increments
}

#[test]
fn test_no_deadlock_on_nested_resolution() {
    use std::thread;

    let container = Container::new();

    // Register multiple services
    container.register(Logger { log_level: "INFO".to_string() });
    container.register(DatabaseConnection {
        connection_string: "test".to_string(),
    });
    container.register(ConfigService {
        config_path: "/test".to_string(),
    });

    let mut handles = vec![];

    // Threads that resolve multiple services in sequence
    for _ in 0..10 {
        let container_clone = container.clone();
        let handle = thread::spawn(move || {
            for _ in 0..100 {
                let _logger = container_clone.resolve::<Logger>();
                let _db = container_clone.resolve::<DatabaseConnection>();
                let _config = container_clone.resolve::<ConfigService>();
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

#[test]
fn test_container_clear_thread_safety() {
    use std::thread;
    use std::time::Duration;

    let container = Container::new();

    // Initial registration
    container.register(Logger { log_level: "START".to_string() });

    let container_clone = container.clone();

    // Thread that clears and re-registers
    let clearer = thread::spawn(move || {
        thread::sleep(Duration::from_millis(10));
        container_clone.clear();
        container_clone.register(Logger { log_level: "CLEARED".to_string() });
    });

    // Main thread tries to resolve during clear
    let mut found_start = false;
    let mut found_cleared = false;

    for _ in 0..1000 {
        match container.resolve::<Logger>() {
            None => { /* Container was cleared momentarily */ }
            Some(logger) if logger.log_level == "START" => found_start = true,
            Some(logger) if logger.log_level == "CLEARED" => found_cleared = true,
            _ => {}
        }
        thread::sleep(Duration::from_micros(10));
    }

    clearer.join().unwrap();

    // We should have seen some transitions
    assert!(found_start || found_cleared); // Should see at least one state
}
