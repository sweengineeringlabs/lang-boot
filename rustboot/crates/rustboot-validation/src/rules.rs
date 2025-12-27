//! Common validation rules (L4: Core - Validation).
//!
//! Pre-built validators for common validation scenarios.

use super::traits::{ValidationErrors, ValidationResult, Validator};
use std::marker::PhantomData;

/// Validator that checks if a string is not empty.
pub struct NotEmptyValidator {
    field: String,
}

impl NotEmptyValidator {
    /// Create a new not-empty validator.
    pub fn new(field: impl Into<String>) -> Self {
        Self {
            field: field.into(),
        }
    }
}

impl Validator<String> for NotEmptyValidator {
    fn validate(&self, value: &String) -> ValidationResult {
        if value.is_empty() {
            let mut errors = ValidationErrors::new();
            errors.add_error(&self.field, format!("{} cannot be empty", self.field));
            Err(errors)
        } else {
            Ok(())
        }
    }
}

impl Validator<&str> for NotEmptyValidator {
    fn validate(&self, value: &&str) -> ValidationResult {
        if value.is_empty() {
            let mut errors = ValidationErrors::new();
            errors.add_error(&self.field, format!("{} cannot be empty", self.field));
            Err(errors)
        } else {
            Ok(())
        }
    }
}

/// Validator for string length constraints.
pub struct LengthValidator {
    field: String,
    min: Option<usize>,
    max: Option<usize>,
}

impl LengthValidator {
    /// Create a validator with minimum length.
    pub fn min(field: impl Into<String>, min: usize) -> Self {
        Self {
            field: field.into(),
            min: Some(min),
            max: None,
        }
    }

    /// Create a validator with maximum length.
    pub fn max(field: impl Into<String>, max: usize) -> Self {
        Self {
            field: field.into(),
            min: None,
            max: Some(max),
        }
    }

    /// Create a validator with both min and max length.
    pub fn range(field: impl Into<String>, min: usize, max: usize) -> Self {
        Self {
            field: field.into(),
            min: Some(min),
            max: Some(max),
        }
    }
}

impl Validator<String> for LengthValidator {
    fn validate(&self, value: &String) -> ValidationResult {
        let len = value.len();
        let mut errors = ValidationErrors::new();

        if let Some(min) = self.min {
            if len < min {
                errors.add_error(
                    &self.field,
                    format!("{} must be at least {} characters", self.field, min),
                );
            }
        }

        if let Some(max) = self.max {
            if len > max {
                errors.add_error(
                    &self.field,
                    format!("{} must be at most {} characters", self.field, max),
                );
            }
        }

        errors.into_result()
    }
}

/// Validator for numeric ranges.
pub struct RangeValidator<T> {
    field: String,
    min: Option<T>,
    max: Option<T>,
}

impl<T: PartialOrd + std::fmt::Display + Copy> RangeValidator<T> {
    /// Create a validator with minimum value.
    pub fn min(field: impl Into<String>, min: T) -> Self {
        Self {
            field: field.into(),
            min: Some(min),
            max: None,
        }
    }

    /// Create a validator with maximum value.
    pub fn max(field: impl Into<String>, max: T) -> Self {
        Self {
            field: field.into(),
            min: None,
            max: Some(max),
        }
    }

    /// Create a validator with both min and max.
    pub fn range(field: impl Into<String>, min: T, max: T) -> Self {
        Self {
            field: field.into(),
            min: Some(min),
            max: Some(max),
        }
    }
}

impl<T: PartialOrd + std::fmt::Display + Copy + Send + Sync> Validator<T> for RangeValidator<T> {
    fn validate(&self, value: &T) -> ValidationResult {
        let mut errors = ValidationErrors::new();

        if let Some(min) = self.min {
            if value < &min {
                errors.add_error(
                    &self.field,
                    format!("{} must be at least {}", self.field, min),
                );
            }
        }

        if let Some(max) = self.max {
            if value > &max {
                errors.add_error(
                    &self.field,
                    format!("{} must be at most {}", self.field, max),
                );
            }
        }

        errors.into_result()
    }
}

/// Validator for email format.
pub struct EmailValidator {
    field: String,
}

impl EmailValidator {
    /// Create a new email validator.
    pub fn new(field: impl Into<String>) -> Self {
        Self {
            field: field.into(),
        }
    }
}

impl Validator<String> for EmailValidator {
    fn validate(&self, value: &String) -> ValidationResult {
        // Simple email validation (not RFC compliant, but good enough for most cases)
        let has_at = value.contains('@');
        let parts: Vec<&str> = value.split('@').collect();
        let valid = has_at 
            && parts.len() == 2 
            && !parts[0].is_empty() 
            && !parts[1].is_empty()
            && parts[1].contains('.');

        if valid {
            Ok(())
        } else {
            let mut errors = ValidationErrors::new();
            errors.add_error(&self.field, format!("{} must be a valid email address", self.field));
            Err(errors)
        }
    }
}

/// Validator for regex patterns.
#[cfg(feature = "regex")]
pub struct RegexValidator {
    field: String,
    pattern: regex::Regex,
    message: String,
}

#[cfg(feature = "regex")]
impl RegexValidator {
    /// Create a new regex validator.
    pub fn new(field: impl Into<String>, pattern: regex::Regex, message: impl Into<String>) -> Self {
        Self {
            field: field.into(),
            pattern,
            message: message.into(),
        }
    }
}

#[cfg(feature = "regex")]
impl Validator<String> for RegexValidator {
    fn validate(&self, value: &String) -> ValidationResult {
        if self.pattern.is_match(value) {
            Ok(())
        } else {
            let mut errors = ValidationErrors::new();
            errors.add_error(&self.field, &self.message);
            Err(errors)
        }
    }
}

/// Validator for required fields (Option<T>).
pub struct RequiredValidator<T> {
    field: String,
    _phantom: PhantomData<T>,
}

impl<T> RequiredValidator<T> {
    /// Create a new required field validator.
    pub fn new(field: impl Into<String>) -> Self {
        Self {
            field: field.into(),
            _phantom: PhantomData,
        }
    }
}

impl<T: Send + Sync> Validator<Option<T>> for RequiredValidator<T> {
    fn validate(&self, value: &Option<T>) -> ValidationResult {
        if value.is_none() {
            let mut errors = ValidationErrors::new();
            errors.add_error(&self.field, format!("{} is required", self.field));
            Err(errors)
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_empty() {
        let validator = NotEmptyValidator::new("name");
        assert!(validator.validate(&"John".to_string()).is_ok());
        assert!(validator.validate(&String::new()).is_err());
    }

    #[test]
    fn test_length_validator() {
        let validator = LengthValidator::range("password", 8, 20);
        assert!(validator.validate(&"password123".to_string()).is_ok());
        assert!(validator.validate(&"short".to_string()).is_err());
        assert!(validator.validate(&"verylongpasswordthatexceedslimit".to_string()).is_err());
    }

    #[test]
    fn test_range_validator() {
        let validator = RangeValidator::range("age", 18, 100);
        assert!(validator.validate(&25).is_ok());
        assert!(validator.validate(&10).is_err());
        assert!(validator.validate(&150).is_err());
    }

    #[test]
    fn test_email_validator() {
        let validator = EmailValidator::new("email");
        assert!(validator.validate(&"user@example.com".to_string()).is_ok());
        assert!(validator.validate(&"invalid".to_string()).is_err());
        assert!(validator.validate(&"@example.com".to_string()).is_err());
        assert!(validator.validate(&"user@".to_string()).is_err());
    }

    #[test]
    fn test_required_validator() {
        let validator = RequiredValidator::<String>::new("name");
        assert!(validator.validate(&Some("John".to_string())).is_ok());
        assert!(validator.validate(&None).is_err());
    }
}
