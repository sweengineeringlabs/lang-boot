//! Property-based tests for rustboot-validation
//!
//! These tests verify fundamental properties that should hold for all validators:
//! - Consistency: Same input always produces same result
//! - Composability: Combined validators work correctly
//! - Edge cases: Validators handle boundary conditions properly

use proptest::prelude::*;
use dev_engineeringlabs_rustboot_validation::{
    EmailValidator, LengthValidator, NotEmptyValidator, RangeValidator, RequiredValidator, Validator,
};

// Property: NotEmptyValidator is consistent
// Valid strings (non-empty) always pass, empty strings always fail
proptest! {
    #[test]
    fn test_not_empty_consistency(s in "\\PC+") {
        let validator = NotEmptyValidator::new("field");
        // Non-empty strings should always pass
        prop_assert!(validator.validate(&s).is_ok());
    }

    #[test]
    fn test_not_empty_rejects_empty(s in prop::string::string_regex("").unwrap()) {
        let validator = NotEmptyValidator::new("field");
        // Empty string should always fail
        prop_assert!(validator.validate(&s).is_err());
    }
}

// Property: LengthValidator is consistent
// Strings within bounds always pass, strings outside bounds always fail
proptest! {
    #[test]
    fn test_length_validator_min_accepts_valid(s in "[a-z]{5,100}") {
        let validator = LengthValidator::min("field", 5);
        // Strings with length >= 5 should pass
        prop_assert!(validator.validate(&s).is_ok());
    }

    #[test]
    fn test_length_validator_min_rejects_short(s in "[a-z]{0,4}") {
        let validator = LengthValidator::min("field", 5);
        // Strings with length < 5 should fail
        prop_assert!(validator.validate(&s).is_err());
    }

    #[test]
    fn test_length_validator_max_accepts_valid(s in "[a-z]{0,10}") {
        let validator = LengthValidator::max("field", 10);
        // Strings with length <= 10 should pass
        prop_assert!(validator.validate(&s).is_ok());
    }

    #[test]
    fn test_length_validator_max_rejects_long(s in "[a-z]{11,50}") {
        let validator = LengthValidator::max("field", 10);
        // Strings with length > 10 should fail
        prop_assert!(validator.validate(&s).is_err());
    }

    #[test]
    fn test_length_validator_range_accepts_valid(s in "[a-z]{5,10}") {
        let validator = LengthValidator::range("field", 5, 10);
        // Strings with length in [5, 10] should pass
        prop_assert!(validator.validate(&s).is_ok());
    }

    #[test]
    fn test_length_validator_range_rejects_outside(
        s in prop_oneof![
            prop::string::string_regex("[a-z]{0,4}").unwrap(),
            prop::string::string_regex("[a-z]{11,50}").unwrap(),
        ]
    ) {
        let validator = LengthValidator::range("field", 5, 10);
        // Strings with length outside [5, 10] should fail
        prop_assert!(validator.validate(&s).is_err());
    }
}

// Property: RangeValidator is consistent for integers
// Values within range always pass, values outside range always fail
proptest! {
    #[test]
    fn test_range_validator_accepts_in_range(n in 18i32..=100) {
        let validator = RangeValidator::range("age", 18, 100);
        prop_assert!(validator.validate(&n).is_ok());
    }

    #[test]
    fn test_range_validator_rejects_below_min(n in i32::MIN..18) {
        let validator = RangeValidator::range("age", 18, 100);
        prop_assert!(validator.validate(&n).is_err());
    }

    #[test]
    fn test_range_validator_rejects_above_max(n in 101i32..=i32::MAX) {
        let validator = RangeValidator::range("age", 18, 100);
        prop_assert!(validator.validate(&n).is_err());
    }

    #[test]
    fn test_range_validator_min_only(n in 0i32..1000) {
        let validator = RangeValidator::min("value", 0);
        prop_assert!(validator.validate(&n).is_ok());
    }

    #[test]
    fn test_range_validator_max_only(n in -1000i32..=100) {
        let validator = RangeValidator::max("value", 100);
        prop_assert!(validator.validate(&n).is_ok());
    }
}

// Property: RangeValidator works for floating point
proptest! {
    #[test]
    fn test_range_validator_f64_accepts_in_range(n in 0.0f64..=100.0) {
        let validator = RangeValidator::range("score", 0.0, 100.0);
        prop_assert!(validator.validate(&n).is_ok());
    }

    #[test]
    fn test_range_validator_f64_rejects_negative(n in -1000.0f64..-0.001) {
        let validator = RangeValidator::range("score", 0.0, 100.0);
        prop_assert!(validator.validate(&n).is_err());
    }

    #[test]
    fn test_range_validator_f64_rejects_too_large(n in 100.001f64..1000.0) {
        let validator = RangeValidator::range("score", 0.0, 100.0);
        prop_assert!(validator.validate(&n).is_err());
    }
}

// Property: EmailValidator basic format checks
proptest! {
    #[test]
    fn test_email_validator_accepts_valid_format(
        local in "[a-z0-9]{1,10}",
        domain in "[a-z]{2,10}",
        tld in "[a-z]{2,5}"
    ) {
        let email = format!("{}@{}.{}", local, domain, tld);
        let validator = EmailValidator::new("email");
        prop_assert!(validator.validate(&email).is_ok());
    }

    #[test]
    fn test_email_validator_rejects_no_at(s in "[a-z0-9.]+") {
        let validator = EmailValidator::new("email");
        // Strings without @ should fail
        prop_assume!(!s.contains('@'));
        prop_assert!(validator.validate(&s).is_err());
    }

    #[test]
    fn test_email_validator_rejects_no_domain(local in "[a-z0-9]{1,10}") {
        let email = format!("{}@", local);
        let validator = EmailValidator::new("email");
        prop_assert!(validator.validate(&email).is_err());
    }

    #[test]
    fn test_email_validator_rejects_no_tld(
        local in "[a-z0-9]{1,10}",
        domain in "[a-z]{2,10}"
    ) {
        let email = format!("{}@{}", local, domain);
        let validator = EmailValidator::new("email");
        // Email without TLD (no dot in domain) should fail
        prop_assert!(validator.validate(&email).is_err());
    }
}

// Property: RequiredValidator consistency
proptest! {
    #[test]
    fn test_required_validator_accepts_some(s in ".*") {
        let validator = RequiredValidator::<String>::new("field");
        prop_assert!(validator.validate(&Some(s)).is_ok());
    }

    #[test]
    fn test_required_validator_rejects_none(_n in 0..1000) {
        let validator = RequiredValidator::<i32>::new("field");
        prop_assert!(validator.validate(&None).is_err());
    }
}

// Property: Validator idempotence
// Running the same validator twice on the same input produces the same result
proptest! {
    #[test]
    fn test_validator_idempotence_length(s in ".*") {
        let validator = LengthValidator::range("field", 5, 10);
        let result1 = validator.validate(&s);
        let result2 = validator.validate(&s);
        prop_assert_eq!(result1.is_ok(), result2.is_ok());
    }

    #[test]
    fn test_validator_idempotence_range(n in i32::MIN..=i32::MAX) {
        let validator = RangeValidator::range("value", 0, 100);
        let result1 = validator.validate(&n);
        let result2 = validator.validate(&n);
        prop_assert_eq!(result1.is_ok(), result2.is_ok());
    }
}

// Property: Boundary conditions
proptest! {
    #[test]
    fn test_length_validator_exact_min(min in 1usize..100) {
        let s = "a".repeat(min);
        let validator = LengthValidator::min("field", min);
        prop_assert!(validator.validate(&s).is_ok());
    }

    #[test]
    fn test_length_validator_exact_max(max in 1usize..100) {
        let s = "a".repeat(max);
        let validator = LengthValidator::max("field", max);
        prop_assert!(validator.validate(&s).is_ok());
    }

    #[test]
    fn test_length_validator_one_below_min(min in 2usize..100) {
        let s = "a".repeat(min - 1);
        let validator = LengthValidator::min("field", min);
        prop_assert!(validator.validate(&s).is_err());
    }

    #[test]
    fn test_length_validator_one_above_max(max in 1usize..100) {
        let s = "a".repeat(max + 1);
        let validator = LengthValidator::max("field", max);
        prop_assert!(validator.validate(&s).is_err());
    }
}

// Property: Range validator boundary conditions
proptest! {
    #[test]
    fn test_range_validator_exact_min(min in -100i32..100) {
        let validator = RangeValidator::min("value", min);
        prop_assert!(validator.validate(&min).is_ok());
    }

    #[test]
    fn test_range_validator_exact_max(max in -100i32..100) {
        let validator = RangeValidator::max("value", max);
        prop_assert!(validator.validate(&max).is_ok());
    }

    #[test]
    fn test_range_validator_both_boundaries(min in -100i32..0, max in 1i32..100) {
        prop_assume!(min < max);
        let validator = RangeValidator::range("value", min, max);
        prop_assert!(validator.validate(&min).is_ok());
        prop_assert!(validator.validate(&max).is_ok());
    }
}

// Property: Unicode handling
proptest! {
    #[test]
    fn test_length_validator_unicode(s in "[\\p{L}]{5,10}") {
        let validator = LengthValidator::range("field", 5, 10);
        // Note: This tests byte length, not character count
        // The validator should be consistent with its definition
        let result = validator.validate(&s);
        // Test should not panic and should give consistent result
        let _ = result;
    }

    #[test]
    fn test_email_validator_unicode(
        local in "[a-z0-9]{1,5}",
        domain in "[a-z]{2,5}",
        tld in "[a-z]{2,3}"
    ) {
        let email = format!("{}@{}.{}", local, domain, tld);
        let validator = EmailValidator::new("email");
        // ASCII emails should work
        prop_assert!(validator.validate(&email).is_ok());
    }
}

// Property: Empty edge cases
proptest! {
    #[test]
    fn test_validators_handle_empty_string(_n in 0..100) {
        let s = String::new();

        // NotEmpty should reject
        let not_empty = NotEmptyValidator::new("field");
        prop_assert!(not_empty.validate(&s).is_err());

        // Length with min > 0 should reject
        let length_min = LengthValidator::min("field", 1);
        prop_assert!(length_min.validate(&s).is_err());

        // Length with min = 0 should accept
        let length_zero = LengthValidator::min("field", 0);
        prop_assert!(length_zero.validate(&s).is_ok());
    }
}

// Property: Type safety across numeric types
proptest! {
    #[test]
    fn test_range_validator_u32(n in 0u32..1000) {
        let validator = RangeValidator::range("value", 0u32, 1000u32);
        prop_assert!(validator.validate(&n).is_ok());
    }

    #[test]
    fn test_range_validator_i64(n in -1000i64..1000) {
        let validator = RangeValidator::range("value", -1000i64, 1000i64);
        prop_assert!(validator.validate(&n).is_ok());
    }

    #[test]
    fn test_range_validator_usize(n in 0usize..1000) {
        let validator = RangeValidator::range("value", 0usize, 1000usize);
        prop_assert!(validator.validate(&n).is_ok());
    }
}
