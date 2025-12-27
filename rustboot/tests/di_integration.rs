//! Integration tests for Rustboot DI module

use rustboot::di::*;

trait Service: Injectable {
    fn execute(&self) -> String;
}

#[derive(Clone)]
struct EmailService;

impl Injectable for EmailService {}

impl Service for EmailService {
    fn execute(&self) -> String {
        "Sending email".to_string()
    }
}

#[derive(Clone)]
struct SmsService;

impl Injectable for SmsService {}

impl Service for SmsService {
    fn execute(&self) -> String {
        "Sending SMS".to_string()
    }
}

#[test]
fn test_container_registration() {
    let mut container = Container::new();
    container.register(EmailService);
    
    let service: Option<&EmailService> = container.resolve();
    assert!(service.is_some());
}

#[test]
fn test_container_multiple_services() {
    let mut container = Container::new();
    container.register(EmailService);
    container.register(SmsService);
    
    assert!(container.resolve::<EmailService>().is_some());
    assert!(container.resolve::<SmsService>().is_some());
}

#[test]
fn test_container_service_execution() {
    let mut container = Container::new();
    container.register(EmailService);
    
    let service = container.resolve::<EmailService>().unwrap();
    assert_eq!(service.execute(), "Sending email");
}

#[test]
fn test_container_resolve_missing() {
    let container = Container::new();
    let service: Option<&EmailService> = container.resolve();
    assert!(service.is_none());
}

#[test]
#[should_panic]
fn test_container_resolve_missing_panic() {
    let container = Container::new();
    let _service: &EmailService = container.resolve().expect("Should panic");
}
