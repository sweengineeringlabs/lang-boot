//! Validation framework (L4: Core).
//!
//! Provides trait-based validation with composable validators and fluent builders.

pub mod builder;
pub mod rules;
pub mod traits;

// Re-export main types
pub use builder::{NumericValidationBuilder, StringValidationBuilder, ValidationBuilder};
pub use rules::{EmailValidator, LengthValidator, NotEmptyValidator, RangeValidator, RequiredValidator};
pub use traits::{Validate, ValidationError, ValidationErrors, ValidationResult, Validator};
