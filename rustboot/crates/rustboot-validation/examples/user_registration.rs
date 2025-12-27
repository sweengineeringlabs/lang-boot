//! User Registration Validation Example
//!
//! Demonstrates comprehensive validation for user registration forms

use dev_engineeringlabs_rustboot_validation::*;

#[derive(Debug)]
struct UserRegistration {
    username: String,
    email: String,
    password: String,
    age: i32,
    phone: Option<String>,
}

fn main() {
    println!("=== Rustboot Validation Example: User Registration ===\n");

    // Valid user
    let valid_user = UserRegistration {
        username: "johndoe".to_string(),
        email: "john@example.com".to_string(),
        password: "Secure123!".to_string(),
        age: 25,
        phone: Some("+1234567890".to_string()),
    };

    validate_user(&valid_user);

    println!("\n---\n");

    // Invalid user (too young)
    let invalid_user = UserRegistration {
        username: "jane".to_string(),
        email: "jane@example.com".to_string(),
        password: "weak".to_string(),
        age: 15,
        phone: None,
    };

    validate_user(&invalid_user);
}

fn validate_user(user: &UserRegistration) {
    println!("Validating user: {:?}\n", user.username);

    // Username validation
    let username_validator = StringValidationBuilder::new("username")
        .not_empty()
        .min_length(3)
        .max_length(20)
        .matches(
            |s: &String| s.chars().all(|c| c.is_alphanumeric()),
            "Username must be alphanumeric only",
        )
        .build();

    match username_validator.validate(&user.username) {
        Ok(_) => println!("✓ Username is valid"),
        Err(errors) => println!("✗ Username validation failed: {:?}", errors),
    }

    // Email validation
    let email_validator = StringValidationBuilder::new("email")
        .not_empty()
        .email()
        .build();

    match email_validator.validate(&user.email) {
        Ok(_) => println!("✓ Email is valid"),
        Err(errors) => println!("✗ Email validation failed: {:?}", errors),
    }

    // Password validation
    let password_validator = StringValidationBuilder::new("password")
        .not_empty()
        .min_length(8)
        .matches(
            |s: &String| s.chars().any(|c| c.is_uppercase()),
            "Password must contain at least one uppercase letter",
        )
        .matches(
            |s: &String| s.chars().any(|c| c.is_lowercase()),
            "Password must contain at least one lowercase letter",
        )
        .matches(
            |s: &String| s.chars().any(|c| c.is_numeric()),
            "Password must contain at least one number",
        )
        .matches(
            |s: &String| s.chars().any(|c| !c.is_alphanumeric()),
            "Password must contain at least one special character",
        )
        .build();

    match password_validator.validate(&user.password) {
        Ok(_) => println!("✓ Password is strong"),
        Err(errors) => println!("✗ Password validation failed: {:?}", errors),
    }

    // Age validation
    let age_validator = NumericValidationBuilder::new("age")
        .min(18)
        .max(120)
        .build();

    match age_validator.validate(&user.age) {
        Ok(_) => println!("✓ Age is valid"),
        Err(errors) => println!("✗ Age validation failed: {:?}", errors),
    }

    // Phone validation (optional field)
    if let Some(phone) = &user.phone {
        let phone_validator = StringValidationBuilder::new("phone")
            .not_empty()
            .min_length(10)
            .matches(
                |s: &String| s.chars().all(|c| c.is_numeric() || c == '+' || c == '-'),
                "Phone must contain only numbers, +, and -",
            )
            .build();

        match phone_validator.validate(phone) {
            Ok(_) => println!("✓ Phone is valid"),
            Err(errors) => println!("✗ Phone validation failed: {:?}", errors),
        }
    } else {
        println!("ⓘ Phone number not provided (optional)");
    }
}
