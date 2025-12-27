//! Integration tests for rustboot-validation
//! 
//! Tests the public API as an external user would use it

use dev_engineeringlabs_rustboot_validation::*;

// ============================================================================
// User Registration Example
// ============================================================================

#[derive(Debug)]
struct UserRegistration {
    username: String,
    email: String,
    age: i32,
    password: String,
}

#[test]
fn test_valid_user_registration() {
    let user = UserRegistration {
        username: "johndoe".to_string(),
        email: "john@example.com".to_string(),
        age: 25,
        password: "SecurePassword123!".to_string(),
    };

    // Validate username
    let username_validator = StringValidationBuilder::new("username")
        .not_empty()
        .min_length(3)
        .max_length(20)
        .matches(
            |s: &String| s.chars().all(|c| c.is_alphanumeric()),
            "Username must be alphanumeric",
        )
        .build();

    assert!(username_validator.validate(&user.username).is_ok());

    // Validate email
    let email_validator = StringValidationBuilder::new("email")
        .not_empty()
        .email()
        .build();

    assert!(email_validator.validate(&user.email).is_ok());

    // Validate age
    let age_validator = NumericValidationBuilder::new("age")
        .min(18)
        .max(120)
        .build();

    assert!(age_validator.validate(&user.age).is_ok());

    // Validate password
    let password_validator = StringValidationBuilder::new("password")
        .not_empty()
        .min_length(8)
        .matches(
            |s: &String| s.chars().any(|c| c.is_uppercase()),
            "Password must contain uppercase",
        )
        .matches(
            |s: &String| s.chars().any(|c| c.is_lowercase()),
            "Password must contain lowercase",
        )
        .matches(
            |s: &String| s.chars().any(|c| c.is_numeric()),
            "Password must contain number",
        )
        .build();

    assert!(password_validator.validate(&user.password).is_ok());
}

#[test]
fn test_invalid_username() {
    let username_validator = StringValidationBuilder::new("username")
        .not_empty()
        .min_length(3)
        .max_length(20)
        .build();

    // Too short
    assert!(username_validator.validate(&"ab".to_string()).is_err());
    
    // Too long
    assert!(username_validator.validate(&"verylongusernamethatexceedslimit".to_string()).is_err());
    
    // Empty
    assert!(username_validator.validate(&"".to_string()).is_err());
}

#[test]
fn test_invalid_email() {
    let email_validator = StringValidationBuilder::new("email")
        .not_empty()
        .email()
        .build();

    assert!(email_validator.validate(&"invalid".to_string()).is_err());
    assert!(email_validator.validate(&"@example.com".to_string()).is_err());
    assert!(email_validator.validate(&"user@".to_string()).is_err());
}

#[test]
fn test_age_constraints() {
    let age_validator = NumericValidationBuilder::new("age")
        .min(18)
        .max(120)
        .build();

    // Valid ages
    assert!(age_validator.validate(&18).is_ok());
    assert!(age_validator.validate(&25).is_ok());
    assert!(age_validator.validate(&120).is_ok());

    // Invalid ages
    assert!(age_validator.validate(&17).is_err());
    assert!(age_validator.validate(&121).is_err());
}

// ============================================================================
// API Request Validation
// ============================================================================

#[derive(Debug)]
struct CreateProductRequest {
    name: String,
    price: f64,
    quantity: i32,
    description: String,
}

#[test]
fn test_product_validation() {
    let product = CreateProductRequest {
        name: "Widget".to_string(),
        price: 29.99,
        quantity: 100,
        description: "A useful widget".to_string(),
    };

    // Validate product name
    let name_validator = StringValidationBuilder::new("name")
        .not_empty()
        .min_length(2)
        .max_length(100)
        .build();

    assert!(name_validator.validate(&product.name).is_ok());

    // Validate price
    let price_validator = NumericValidationBuilder::new("price")
        .min(0.01)
        .max(999999.99)
        .build();

    assert!(price_validator.validate(&product.price).is_ok());

    // Validate quantity
    let quantity_validator = NumericValidationBuilder::new("quantity")
        .min(1)
        .build();

    assert!(quantity_validator.validate(&product.quantity).is_ok());
}

// ============================================================================
// Custom Validation Rules
// ============================================================================

#[test]
fn test_custom_business_rules() {
    // Custom rule: discount percentage must be between 0 and 100
    let discount_validator = ValidationBuilder::new()
        .rule("discount", |d: &f64| *d >= 0.0, "Discount cannot be negative")
        .rule("discount", |d: &f64| *d <= 100.0, "Discount cannot exceed 100%")
        .build();

    assert!(discount_validator.validate(&50.0).is_ok());
    assert!(discount_validator.validate(&-10.0).is_err());
    assert!(discount_validator.validate(&150.0).is_err());
}

#[test]
fn test_composite_validation() {
    // Multiple rules on same field
    let username_validator = StringValidationBuilder::new("username")
        .not_empty()
        .min_length(3)
        .max_length(15)
        .matches(
            |s: &String| !s.contains(' '),
            "Username cannot contain spaces",
        )
        .matches(
            |s: &String| s.chars().next().unwrap().is_alphabetic(),
            "Username must start with a letter",
        )
        .build();

    assert!(username_validator.validate(&"johndoe".to_string()).is_ok());
    assert!(username_validator.validate(&"john doe".to_string()).is_err()); // Has space
    assert!(username_validator.validate(&"123abc".to_string()).is_err()); // Starts with number
}
