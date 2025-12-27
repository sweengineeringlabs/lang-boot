//! Rustboot Validation - Type-safe validation framework
//!
//! Provides composable validation with fluent builders.

pub mod builder;
pub mod rules;
pub mod traits;

pub use builder::{NumericValidationBuilder, StringValidationBuilder, ValidationBuilder};
pub use rules::{EmailValidator, LengthValidator, NotEmptyValidator, RangeValidator, RequiredValidator};
pub use traits::{Validate, ValidationError, ValidationErrors, ValidationResult, Validator};
