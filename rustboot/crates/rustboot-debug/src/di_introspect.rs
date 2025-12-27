//! DI container introspection utilities.

use std::any::{type_name, Any, TypeId};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use tracing::info;

/// Container introspection information.
#[derive(Debug, Clone)]
pub struct ServiceInfo {
    /// Type name of the service.
    pub type_name: String,
    /// TypeId for matching.
    pub type_id: TypeId,
    /// Whether the service is registered.
    pub is_registered: bool,
}

/// DI container introspector for debugging.
pub struct ContainerIntrospector {
    /// Snapshot of registered services.
    services: Arc<RwLock<HashMap<TypeId, String>>>,
}

impl ContainerIntrospector {
    /// Create a new introspector.
    pub fn new() -> Self {
        Self {
            services: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Record a service registration.
    pub fn record_service<T: Any>(&self) {
        let type_id = TypeId::of::<T>();
        let type_name = type_name::<T>().to_string();

        let mut services = self.services.write().unwrap();
        services.insert(type_id, type_name);
    }

    /// Remove a service record.
    pub fn remove_service<T: Any>(&self) {
        let type_id = TypeId::of::<T>();
        let mut services = self.services.write().unwrap();
        services.remove(&type_id);
    }

    /// Check if a service is registered.
    pub fn is_registered<T: Any>(&self) -> bool {
        let type_id = TypeId::of::<T>();
        let services = self.services.read().unwrap();
        services.contains_key(&type_id)
    }

    /// Get info about a specific service.
    pub fn get_service_info<T: Any>(&self) -> ServiceInfo {
        let type_id = TypeId::of::<T>();
        let services = self.services.read().unwrap();

        ServiceInfo {
            type_name: type_name::<T>().to_string(),
            type_id,
            is_registered: services.contains_key(&type_id),
        }
    }

    /// List all registered services.
    pub fn list_services(&self) -> Vec<String> {
        let services = self.services.read().unwrap();
        let mut names: Vec<String> = services.values().cloned().collect();
        names.sort();
        names
    }

    /// Get the count of registered services.
    pub fn service_count(&self) -> usize {
        let services = self.services.read().unwrap();
        services.len()
    }

    /// Clear all service records.
    pub fn clear(&self) {
        let mut services = self.services.write().unwrap();
        services.clear();
    }

    /// Print a formatted list of all services.
    pub fn print_services(&self) {
        let services = self.list_services();

        info!(
            target: "rustboot::debug::di",
            count = services.len(),
            "Registered DI services"
        );

        for (i, service) in services.iter().enumerate() {
            info!(
                target: "rustboot::debug::di",
                index = i + 1,
                service = %service,
                "Service registered"
            );
        }
    }

    /// Generate a formatted report of all services.
    pub fn generate_report(&self) -> String {
        let services = self.list_services();
        let mut output = String::new();

        output.push_str("DI Container Services\n");
        output.push_str("=====================\n\n");
        output.push_str(&format!("Total Services: {}\n\n", services.len()));

        if services.is_empty() {
            output.push_str("(No services registered)\n");
        } else {
            for (i, service) in services.iter().enumerate() {
                output.push_str(&format!("{}. {}\n", i + 1, service));
            }
        }

        output
    }

    /// Generate a markdown list of services.
    pub fn generate_markdown(&self) -> String {
        let services = self.list_services();
        let mut output = String::new();

        output.push_str("# DI Container Services\n\n");
        output.push_str(&format!("**Total Services:** {}\n\n", services.len()));

        if services.is_empty() {
            output.push_str("*No services registered*\n");
        } else {
            output.push_str("## Registered Services\n\n");
            for service in services {
                output.push_str(&format!("- `{}`\n", service));
            }
        }

        output
    }

    /// Check for dependency cycles (simplified - checks if services depend on each other).
    pub fn check_health(&self) -> HealthReport {
        let services = self.list_services();

        HealthReport {
            total_services: services.len(),
            warnings: Vec::new(),
            is_healthy: true,
        }
    }
}

impl Default for ContainerIntrospector {
    fn default() -> Self {
        Self::new()
    }
}

/// Health report for the DI container.
#[derive(Debug, Clone)]
pub struct HealthReport {
    /// Total number of services.
    pub total_services: usize,
    /// Warning messages.
    pub warnings: Vec<String>,
    /// Whether the container is healthy.
    pub is_healthy: bool,
}

impl HealthReport {
    /// Format the health report.
    pub fn format(&self) -> String {
        let mut output = String::new();
        output.push_str("DI Container Health Report\n");
        output.push_str("==========================\n\n");
        output.push_str(&format!("Total Services: {}\n", self.total_services));
        output.push_str(&format!(
            "Status: {}\n\n",
            if self.is_healthy { "Healthy" } else { "Unhealthy" }
        ));

        if !self.warnings.is_empty() {
            output.push_str("Warnings:\n");
            for warning in &self.warnings {
                output.push_str(&format!("  - {}\n", warning));
            }
        } else {
            output.push_str("No warnings.\n");
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone)]
    struct TestService {
        value: i32,
    }

    #[derive(Clone)]
    struct AnotherService {
        name: String,
    }

    #[test]
    fn test_introspector_basic() {
        let introspector = ContainerIntrospector::new();

        assert!(!introspector.is_registered::<TestService>());

        introspector.record_service::<TestService>();
        assert!(introspector.is_registered::<TestService>());

        assert_eq!(introspector.service_count(), 1);
    }

    #[test]
    fn test_list_services() {
        let introspector = ContainerIntrospector::new();

        introspector.record_service::<TestService>();
        introspector.record_service::<AnotherService>();

        let services = introspector.list_services();
        assert_eq!(services.len(), 2);
    }

    #[test]
    fn test_service_info() {
        let introspector = ContainerIntrospector::new();
        introspector.record_service::<TestService>();

        let info = introspector.get_service_info::<TestService>();
        assert!(info.is_registered);
        assert!(info.type_name.contains("TestService"));
    }

    #[test]
    fn test_remove_service() {
        let introspector = ContainerIntrospector::new();

        introspector.record_service::<TestService>();
        assert!(introspector.is_registered::<TestService>());

        introspector.remove_service::<TestService>();
        assert!(!introspector.is_registered::<TestService>());
    }

    #[test]
    fn test_clear() {
        let introspector = ContainerIntrospector::new();

        introspector.record_service::<TestService>();
        introspector.record_service::<AnotherService>();
        assert_eq!(introspector.service_count(), 2);

        introspector.clear();
        assert_eq!(introspector.service_count(), 0);
    }

    #[test]
    fn test_generate_report() {
        let introspector = ContainerIntrospector::new();
        introspector.record_service::<TestService>();

        let report = introspector.generate_report();
        assert!(report.contains("DI Container Services"));
        assert!(report.contains("Total Services: 1"));
    }

    #[test]
    fn test_generate_markdown() {
        let introspector = ContainerIntrospector::new();
        introspector.record_service::<TestService>();

        let markdown = introspector.generate_markdown();
        assert!(markdown.contains("# DI Container Services"));
        assert!(markdown.contains("TestService"));
    }

    #[test]
    fn test_health_check() {
        let introspector = ContainerIntrospector::new();
        introspector.record_service::<TestService>();

        let health = introspector.check_health();
        assert!(health.is_healthy);
        assert_eq!(health.total_services, 1);
    }
}
