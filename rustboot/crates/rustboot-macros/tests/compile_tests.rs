// Basic compile tests for rustboot-macros
// These tests ensure macros compile and generate valid code

#![allow(dead_code, unused_variables)]

use rustboot_macros::{Injectable, Validate};

#[test]
fn test_injectable_basic() {
    // This test just needs to compile
    #[derive(Injectable)]
    struct TestService {
        value: i32,
    }
}

#[test]
fn test_validate_basic() {
    // This test just needs to compile
    #[derive(Validate)]
    struct TestStruct {
        #[validate(length(min = 1))]
        name: String,
    }
}

#[test]
fn test_multiple_derives() {
    // Test combining multiple derives
    #[derive(Debug, Clone, Injectable)]
    struct MultiService {
        id: u64,
    }
}

// More comprehensive tests would go in integration tests
