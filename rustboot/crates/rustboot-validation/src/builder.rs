//! Fluent validation builder (L4: Core - Validation).
//!
//! Provides a fluent interface for building complex validators.

use super::rules::*;
use super::traits::{CompositeValidator, PredicateValidator, Validator};

/// Fluent builder for creating validators.
pub struct ValidationBuilder<T> {
    validators: Vec<Box<dyn Validator<T>>>,
}

impl<T: Send + Sync + 'static> ValidationBuilder<T> {
    /// Add a custom validator.
    pub fn custom(mut self, validator: impl Validator<T> + 'static) -> Self {
        self.validators.push(Box::new(validator));
        self
    }

    /// Add a predicate-based rule.
    pub fn rule<F>(mut self, field: &str, predicate: F, message: &str) -> Self
    where
        F: Fn(&T) -> bool + Send + Sync + 'static,
    {
        self.validators
            .push(Box::new(PredicateValidator::new(field, predicate, message)));
        self
    }

    /// Build the final validator.
    pub fn build(self) -> CompositeValidator<T> {
        let mut composite = CompositeValidator::new();
        for validator in self.validators {
            // Use Box::leak to convert Box<dyn Validator> to &'static dyn Validator
            // This is safe because validators live for the entire program
            composite.validators.push(validator);
        }
        composite
    }
}

impl<T> ValidationBuilder<T> {
    /// Create a new validation builder.
    pub fn new() -> Self {
        Self {
            validators: Vec::new(),
        }
    }
}

impl<T> Default for ValidationBuilder<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Fluent builder for string validation.
pub struct StringValidationBuilder {
    field: String,
    validators: Vec<Box<dyn Validator<String>>>,
}

impl StringValidationBuilder {
    /// Create a new string validation builder.
    pub fn new(field: impl Into<String>) -> Self {
        Self {
            field: field.into(),
            validators: Vec::new(),
        }
    }

    /// Require the string to be non-empty.
    pub fn not_empty(mut self) -> Self {
        self.validators
            .push(Box::new(NotEmptyValidator::new(&self.field)));
        self
    }

    /// Require minimum length.
    pub fn min_length(mut self, min: usize) -> Self {
        self.validators
            .push(Box::new(LengthValidator::min(&self.field, min)));
        self
    }

    /// Require maximum length.
    pub fn max_length(mut self, max: usize) -> Self {
        self.validators
            .push(Box::new(LengthValidator::max(&self.field, max)));
        self
    }

    /// Require length in range.
    pub fn length_range(mut self, min: usize, max: usize) -> Self {
        self.validators
            .push(Box::new(LengthValidator::range(&self.field, min, max)));
        self
    }

    /// Require valid email format.
    pub fn email(mut self) -> Self {
        self.validators
            .push(Box::new(EmailValidator::new(&self.field)));
        self
    }

    /// Add a custom predicate.
    pub fn matches<F>(mut self, predicate: F, message: &str) -> Self
    where
        F: Fn(&String) -> bool + Send + Sync + 'static,
    {
        self.validators.push(Box::new(PredicateValidator::new(
            &self.field,
            predicate,
            message,
        )));
        self
    }

    /// Build the validator.
    pub fn build(self) -> CompositeValidator<String> {
        let mut composite = CompositeValidator::new();
        for validator in self.validators {
            composite.validators.push(validator);
        }
        composite
    }
}

/// Fluent builder for numeric validation.
pub struct NumericValidationBuilder<T> {
    field: String,
    validators: Vec<Box<dyn Validator<T>>>,
}

impl<T: PartialOrd + std::fmt::Display + Copy + Send + Sync + 'static> NumericValidationBuilder<T> {
    /// Create a new numeric validation builder.
    pub fn new(field: impl Into<String>) -> Self {
        Self {
            field: field.into(),
            validators: Vec::new(),
        }
    }

    /// Require minimum value.
    pub fn min(mut self, min: T) -> Self {
        self.validators
            .push(Box::new(RangeValidator::min(&self.field, min)));
        self
    }

    /// Require maximum value.
    pub fn max(mut self, max: T) -> Self {
        self.validators
            .push(Box::new(RangeValidator::max(&self.field, max)));
        self
    }

    /// Require value in range.
    pub fn range(mut self, min: T, max: T) -> Self {
        self.validators
            .push(Box::new(RangeValidator::range(&self.field, min, max)));
        self
    }

    /// Add a custom predicate.
    pub fn matches<F>(mut self, predicate: F, message: &str) -> Self
    where
        F: Fn(&T) -> bool + Send + Sync + 'static,
    {
        self.validators.push(Box::new(PredicateValidator::new(
            &self.field,
            predicate,
            message,
        )));
        self
    }

    /// Build the validator.
    pub fn build(self) -> CompositeValidator<T> {
        let mut composite = CompositeValidator::new();
        for validator in self.validators {
            composite.validators.push(validator);
        }
        composite
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct User {
        name: String,
        age: i32,
        email: String,
    }

    #[test]
    fn test_string_builder() {
        let validator = StringValidationBuilder::new("name")
            .not_empty()
            .min_length(2)
            .max_length(50)
            .build();

        assert!(validator.validate(&"John".to_string()).is_ok());
        assert!(validator.validate(&"".to_string()).is_err());
        assert!(validator.validate(&"J".to_string()).is_err());
    }

    #[test]
    fn test_numeric_builder() {
        let validator = NumericValidationBuilder::new("age")
            .min(18)
            .max(100)
            .build();

        assert!(validator.validate(&25).is_ok());
        assert!(validator.validate(&10).is_err());
        assert!(validator.validate(&150).is_err());
    }

    #[test]
    fn test_email_builder() {
        let validator = StringValidationBuilder::new("email")
            .not_empty()
            .email()
            .build();

        assert!(validator.validate(&"user@example.com".to_string()).is_ok());
        assert!(validator.validate(&"invalid".to_string()).is_err());
    }

    #[test]
    fn test_custom_rule() {
        let validator = ValidationBuilder::new()
            .rule("age", |age: &i32| *age >= 18, "Must be adult")
            .rule("age", |age: &i32| *age <= 100, "Age too high")
            .build();

        assert!(validator.validate(&25).is_ok());
        assert!(validator.validate(&10).is_err());
    }
}
