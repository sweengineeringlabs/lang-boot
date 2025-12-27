//! Dependency injection container (L4: Core - DI).
//!
//! Simple service locator pattern for dependency injection.

use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Type alias for a thread-safe service instance.
type Service = Arc<dyn Any + Send + Sync>;
/// Type alias for the service registry.
type ServiceRegistry = HashMap<TypeId, Service>;

/// Dependency injection container.
#[derive(Clone)]
pub struct Container {
    services: Arc<RwLock<ServiceRegistry>>,
}

impl Container {
    /// Create a new container.
    pub fn new() -> Self {
        Self {
            services: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Register a service.
    pub fn register<T: Any + Send + Sync>(&self, service: T) {
        let type_id = TypeId::of::<T>();
        let mut services = self.services.write().unwrap();
        services.insert(type_id, Arc::new(service));
    }
    
    /// Resolve a service.
    pub fn resolve<T: Any + Send + Sync>(&self) -> Option<Arc<T>> {
        let type_id = TypeId::of::<T>();
        let services = self.services.read().unwrap();
        
        services.get(&type_id).and_then(|service| {
            service.clone().downcast::<T>().ok()
        })
    }
    
    /// Check if a service is registered.
    pub fn contains<T: Any + Send + Sync>(&self) -> bool {
        let type_id = TypeId::of::<T>();
        let services = self.services.read().unwrap();
        services.contains_key(&type_id)
    }
    
    /// Clear all services.
    pub fn clear(&self) {
        let mut services = self.services.write().unwrap();
        services.clear();
    }
}

impl Default for Container {
    fn default() -> Self {
        Self::new()
    }
}

/// Trait for types that can be injected.
pub trait Injectable: Any + Send + Sync {
    /// Get dependencies from container.
    fn inject(container: &Container) -> Self
    where
        Self: Sized;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone)]
    struct MyService {
        value: i32,
    }

    #[derive(Clone)]
    struct AnotherService {
        name: String,
    }

    #[test]
    fn test_container() {
        let container = Container::new();

        container.register(MyService { value: 42 });

        let service = container.resolve::<MyService>().unwrap();
        assert_eq!(service.value, 42);
    }

    #[test]
    fn test_missing_service() {
        let container = Container::new();
        assert!(container.resolve::<MyService>().is_none());
    }

    #[test]
    fn test_contains() {
        let container = Container::new();

        assert!(!container.contains::<MyService>());

        container.register(MyService { value: 10 });

        assert!(container.contains::<MyService>());
        assert!(!container.contains::<AnotherService>());
    }

    #[test]
    fn test_clear() {
        let container = Container::new();

        container.register(MyService { value: 1 });
        container.register(AnotherService { name: "test".to_string() });

        assert!(container.contains::<MyService>());
        assert!(container.contains::<AnotherService>());

        container.clear();

        assert!(!container.contains::<MyService>());
        assert!(!container.contains::<AnotherService>());
    }

    #[test]
    fn test_multiple_services() {
        let container = Container::new();

        container.register(MyService { value: 100 });
        container.register(AnotherService { name: "hello".to_string() });

        let my_service = container.resolve::<MyService>().unwrap();
        let another_service = container.resolve::<AnotherService>().unwrap();

        assert_eq!(my_service.value, 100);
        assert_eq!(another_service.name, "hello");
    }

    #[test]
    fn test_container_clone_shares_state() {
        let container1 = Container::new();
        container1.register(MyService { value: 42 });

        let container2 = container1.clone();

        // Both containers should see the same service
        let service1 = container1.resolve::<MyService>().unwrap();
        let service2 = container2.resolve::<MyService>().unwrap();

        assert_eq!(service1.value, service2.value);

        // Registering on cloned container should be visible on original
        container2.register(AnotherService { name: "shared".to_string() });
        assert!(container1.contains::<AnotherService>());
    }

    #[test]
    fn test_container_default() {
        let container = Container::default();
        assert!(!container.contains::<MyService>());
    }

    #[test]
    fn test_register_overwrites() {
        let container = Container::new();

        container.register(MyService { value: 1 });
        let first = container.resolve::<MyService>().unwrap();
        assert_eq!(first.value, 1);

        container.register(MyService { value: 2 });
        let second = container.resolve::<MyService>().unwrap();
        assert_eq!(second.value, 2);
    }
}
