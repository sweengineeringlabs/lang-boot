// Test file for the #[derive(Builder)] macro

#![allow(dead_code)]

use rustboot_macros::Builder;

// ============================================================================
// Standard Builder Pattern (separate XxxBuilder struct)
// ============================================================================

#[derive(Builder, Debug, PartialEq)]
struct StandardConfig {
    name: String,
    port: u16,
}

#[test]
fn test_standard_builder() {
    let config = StandardConfig::builder()
        .name("test".to_string())
        .port(8080)
        .build()
        .unwrap();

    assert_eq!(config.name, "test");
    assert_eq!(config.port, 8080);
}

#[test]
fn test_standard_builder_missing_field() {
    let result = StandardConfig::builder()
        .name("test".to_string())
        // missing port
        .build();

    assert!(result.is_err());
    assert!(result.unwrap_err().contains("port"));
}

// ============================================================================
// Fluent Builder Pattern (with_* methods on struct itself)
// ============================================================================

#[derive(Builder, Debug, PartialEq, Default)]
#[builder(fluent)]
struct FluentConfig {
    name: String,
    timeout: Option<u64>,
    retries: Option<usize>,
}

#[test]
fn test_fluent_builder() {
    let config = FluentConfig::default()
        .with_name("test".to_string())
        .with_timeout(5000)
        .with_retries(3);

    assert_eq!(config.name, "test");
    assert_eq!(config.timeout, Some(5000));
    assert_eq!(config.retries, Some(3));
}

#[test]
fn test_fluent_builder_partial() {
    let config = FluentConfig::default()
        .with_name("partial".to_string())
        .with_timeout(1000);
    // retries is None

    assert_eq!(config.name, "partial");
    assert_eq!(config.timeout, Some(1000));
    assert_eq!(config.retries, None);
}

#[test]
fn test_fluent_builder_chaining() {
    // Verify the fluent pattern returns Self for chaining
    let config = FluentConfig {
        name: "initial".to_string(),
        timeout: None,
        retries: None,
    }
    .with_timeout(100)
    .with_retries(5);

    assert_eq!(config.name, "initial");
    assert_eq!(config.timeout, Some(100));
    assert_eq!(config.retries, Some(5));
}

// ============================================================================
// Fluent Builder with non-Option fields
// ============================================================================

#[derive(Builder, Debug, PartialEq, Default)]
#[builder(fluent)]
struct QueryOptions {
    limit: usize,
    offset: usize,
    include_hidden: bool,
}

#[test]
fn test_fluent_non_option_fields() {
    let opts = QueryOptions::default()
        .with_limit(100)
        .with_offset(50)
        .with_include_hidden(true);

    assert_eq!(opts.limit, 100);
    assert_eq!(opts.offset, 50);
    assert!(opts.include_hidden);
}

// ============================================================================
// Mixed fields
// ============================================================================

#[derive(Builder, Debug, PartialEq)]
#[builder(fluent)]
struct MixedConfig {
    required: String,
    optional_num: Option<i32>,
    optional_str: Option<String>,
    flag: bool,
}

impl Default for MixedConfig {
    fn default() -> Self {
        Self {
            required: String::new(),
            optional_num: None,
            optional_str: None,
            flag: false,
        }
    }
}

#[test]
fn test_mixed_fluent_builder() {
    let config = MixedConfig::default()
        .with_required("hello".to_string())
        .with_optional_num(42)
        .with_flag(true);
    // optional_str left as None

    assert_eq!(config.required, "hello");
    assert_eq!(config.optional_num, Some(42));
    assert_eq!(config.optional_str, None);
    assert!(config.flag);
}
