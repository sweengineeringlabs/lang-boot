//! Validation traits and types (L4: Core - Validation).
//!
//! This module provides trait-based validation for any type.

use std::fmt;

/// Result of a validation operation.
pub type ValidationResult = Result<(), ValidationErrors>;

/// Collection of validation errors.
#[derive(Debug, Clone, Default)]
pub struct ValidationErrors {
    errors: Vec<ValidationError>,
}

impl ValidationErrors {
    /// Create a new empty validation errors collection.
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
        }
    }

    /// Add a validation error.
    pub fn add(&mut self, error: ValidationError) {
        self.errors.push(error);
    }

    /// Add a simple error with field and message.
    pub fn add_error(&mut self, field: impl Into<String>, message: impl Into<String>) {
        self.errors.push(ValidationError {
            field: field.into(),
            message: message.into(),
        });
    }

    /// Check if there are any errors.
    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }

    /// Get the number of errors.
    pub fn len(&self) -> usize {
        self.errors.len()
    }

    /// Get all errors.
    pub fn errors(&self) -> &[ValidationError] {
        &self.errors
    }

    /// Convert to a result.
    pub fn into_result(self) -> ValidationResult {
        if self.is_empty() {
            Ok(())
        } else {
            Err(self)
        }
    }
}

impl fmt::Display for ValidationErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.errors.is_empty() {
            return write!(f, "No validation errors");
        }
        
        writeln!(f, "Validation failed with {} error(s):", self.errors.len())?;
        for error in &self.errors {
            writeln!(f, "  - {}: {}", error.field, error.message)?;
        }
        Ok(())
    }
}

impl std::error::Error for ValidationErrors {}

/// A single validation error.
#[derive(Debug, Clone)]
pub struct ValidationError {
    /// The field that failed validation.
    pub field: String,
    /// The error message.
    pub message: String,
}

impl ValidationError {
    /// Create a new validation error.
    pub fn new(field: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            field: field.into(),
            message: message.into(),
        }
    }
}

/// Trait for types that can validate themselves.
pub trait Validate {
    /// Validate this instance.
    fn validate(&self) -> ValidationResult;
}

/// Trait for custom validators.
///
/// Validators can be composed and reused across different types.
pub trait Validator<T>: Send + Sync {
    /// Validate a value.
    fn validate(&self, value: &T) -> ValidationResult;
}

/// A validator that always succeeds.
pub struct AlwaysValid;

impl<T> Validator<T> for AlwaysValid {
    fn validate(&self, _value: &T) -> ValidationResult {
        Ok(())
    }
}

/// A validator that uses a predicate function.
pub struct PredicateValidator<T, F>
where
    F: Fn(&T) -> bool + Send + Sync,
{
    predicate: F,
    message: String,
    field: String,
    _phantom: std::marker::PhantomData<T>,
}

impl<T, F> PredicateValidator<T, F>
where
    F: Fn(&T) -> bool + Send + Sync,
{
    /// Create a new predicate validator.
    pub fn new(field: impl Into<String>, predicate: F, message: impl Into<String>) -> Self {
        Self {
            predicate,
            message: message.into(),
            field: field.into(),
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T, F> Validator<T> for PredicateValidator<T, F>
where
    T: Send + Sync,
    F: Fn(&T) -> bool + Send + Sync,
{
    fn validate(&self, value: &T) -> ValidationResult {
        if (self.predicate)(value) {
            Ok(())
        } else {
            let mut errors = ValidationErrors::new();
            errors.add_error(&self.field, &self.message);
            Err(errors)
        }
    }
}

/// Combine multiple validators.
pub struct CompositeValidator<T> {
    pub(crate) validators: Vec<Box<dyn Validator<T>>>,
}

impl<T> CompositeValidator<T> {
    /// Create a new composite validator.
    pub fn new() -> Self {
        Self {
            validators: Vec::new(),
        }
    }

    /// Add a validator to the composite.
    pub fn with_validator(mut self, validator: impl Validator<T> + 'static) -> Self {
        self.validators.push(Box::new(validator));
        self
    }
}

impl<T> Default for CompositeValidator<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Validator<T> for CompositeValidator<T> {
    fn validate(&self, value: &T) -> ValidationResult {
        let mut all_errors = ValidationErrors::new();
        
        for validator in &self.validators {
            if let Err(errors) = validator.validate(value) {
                for error in errors.errors() {
                    all_errors.add(error.clone());
                }
            }
        }
        
        all_errors.into_result()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_errors() {
        let mut errors = ValidationErrors::new();
        assert!(errors.is_empty());
        
        errors.add_error("name", "Name is required");
        assert!(!errors.is_empty());
        assert_eq!(errors.len(), 1);
        
        errors.add_error("age", "Age must be positive");
        assert_eq!(errors.len(), 2);
    }

    #[test]
    fn test_predicate_validator() {
        let validator = PredicateValidator::new(
            "age",
            |age: &i32| *age >= 18,
            "Must be 18 or older"
        );
        
        assert!(validator.validate(&25).is_ok());
        assert!(validator.validate(&10).is_err());
    }

    #[test]
    fn test_composite_validator() {
        let validator = CompositeValidator::new()
            .with_validator(PredicateValidator::new("age", |age: &i32| *age >= 18, "Too young"))
            .with_validator(PredicateValidator::new("age", |age: &i32| *age <= 100, "Too old"));
        
        assert!(validator.validate(&25).is_ok());
        assert!(validator.validate(&10).is_err());
        assert!(validator.validate(&150).is_err());
    }
}
