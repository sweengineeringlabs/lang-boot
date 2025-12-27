//! Integration tests for Rustboot validation module

use rustboot::validation::*;

#[test]
fn test_validation_builder_chaining() {
    let validator = StringValidationBuilder::new("email")
        .not_empty()
        .email()
        .build();
    
    assert!(validator.validate("test@example.com").is_ok());
    assert!(validator.validate("").is_err());
    assert!(validator.validate("not-an-email").is_err());
}

#[test]
fn test_numeric_validation_ranges() {
    let validator = NumericValidationBuilder::new("age")
        .required()
        .range(0, 120)
        .build();
    
    assert!(validator.validate(&50).is_ok());
    assert!(validator.validate(&0).is_ok());
    assert!(validator.validate(&120).is_ok());
    assert!(validator.validate(&-1).is_err());
    assert!(validator.validate(&121).is_err());
}

#[test]
fn test_validation_error_messages() {
    let validator = StringValidationBuilder::new("username")
        .not_empty()
        .length(3, 20)
        .build();
    
    match validator.validate("ab") {
        Err(errors) => {
            let field_errors = errors.field_errors("username");
            assert!(!field_errors.is_empty());
        }
        Ok(_) => panic!("Expected validation error"),
    }
}

#[test]
#[should_panic(expected = "validation")]
fn test_validation_panic_on_invalid() {
    let validator = StringValidationBuilder::new("test")
        .not_empty()
        .build();
    
    validator.validate("").expect("Should panic");
}

#[test]
fn test_multiple_validators_combined() {
    let email_validator = StringValidationBuilder::new("email")
        .not_empty()
        .email()
        .build();
    
    let age_validator = NumericValidationBuilder::new("age")
        .required()
        .range(18, 100)
        .build();
    
    let email_result = email_validator.validate("test@example.com");
    let age_result = age_validator.validate(&25);
    
    assert!(email_result.is_ok());
    assert!(age_result.is_ok());
}
