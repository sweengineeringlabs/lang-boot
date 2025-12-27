// Derive macro compile tests for rustboot-macros
// Tests ensure derive macros compile and generate valid code

#![allow(dead_code, unused_variables)]

use rustboot_macros::{Injectable, Validate, Builder, Event};

// ============================================================================
// Injectable Tests
// ============================================================================

#[test]
fn test_injectable_basic() {
    #[derive(Injectable)]
    struct TestService {
        value: i32,
    }
}

#[test]
fn test_injectable_multiple_fields() {
    #[derive(Injectable)]
    struct UserService {
        repository: String,
        cache: i32,
        logger: bool,
    }
}

#[test]
fn test_injectable_with_generics() {
    #[derive(Injectable)]
    struct GenericService<T> {
        data: T,
    }
}

// ============================================================================
// Validate Tests
// ============================================================================

#[test]
fn test_validate_basic() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(length(min = 1))]
        name: String,
    }
}

#[test]
fn test_validate_email() {
    #[derive(Validate)]
    struct User {
        #[validate(email)]
        email: String,
    }
}

#[test]
fn test_validate_range() {
    #[derive(Validate)]
    struct Person {
        #[validate(range(min = 18, max = 120))]
        age: u8,
    }
}

#[test]
fn test_validate_multiple_fields() {
    #[derive(Validate)]
    struct CreateUser {
        #[validate(length(min = 3, max = 50))]
        username: String,
        
        #[validate(email)]
        email: String,
        
        #[validate(range(min = 18, max = 120))]
        age: u8,
    }
}

#[test]
fn test_validate_optional_field() {
    #[derive(Validate)]
    struct Profile {
        #[validate(length(max = 500))]
        bio: Option<String>,
    }
}

// ============================================================================
// Builder Tests
// ============================================================================

#[test]
fn test_builder_basic() {
    #[derive(Builder)]
    struct Config {
        host: String,
        port: u16,
    }
}

#[test]
fn test_builder_multiple_fields() {
    #[derive(Builder)]
    struct UserConfig {
        name: String,
        email: String,
        age: u8,
        active: bool,
    }
}

#[test]
fn test_builder_with_generics() {
    #[derive(Builder)]
    struct GenericConfig<T> {
        value: T,
        count: usize,
    }
}

#[test]
fn test_builder_optional_fields() {
    #[derive(Builder)]
    struct Settings {
        required: String,
        optional: Option<i32>,
    }
}

// ============================================================================
// Event Tests
// ============================================================================

#[test]
fn test_event_basic() {
    #[derive(Event)]
    struct UserCreated {
        user_id: u64,
    }
}

#[test]
fn test_event_multiple_fields() {
    #[derive(Event)]
    struct OrderPlaced {
        order_id: u64,
        user_id: u64,
        amount: f64,
    }
}

#[test]
fn test_event_with_string() {
    #[derive(Event)]
    struct MessageSent {
        message_id: String,
        content: String,
    }
}

// ============================================================================
// Combined Derives Tests
// ============================================================================

#[test]
fn test_multiple_derives() {
    #[derive(Debug, Clone, Injectable)]
    struct MultiService {
        id: u64,
    }
}

#[test]
fn test_validate_and_builder() {
    #[derive(Validate, Builder)]
    struct UserRequest {
        #[validate(length(min = 3))]
        name: String,
        
        #[validate(email)]
        email: String,
    }
}

#[test]
fn test_all_custom_derives() {
    #[derive(Injectable, Validate, Builder)]
    struct ComplexStruct {
        #[validate(length(min = 1))]
        field1: String,
        field2: i32,
    }
}
