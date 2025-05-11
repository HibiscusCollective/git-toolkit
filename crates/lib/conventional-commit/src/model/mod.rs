//! Model types and traits for conventional commit parsing and validation.
//!
//! This module defines core data structures, builders, and validation traits for conventional commits

use crate::errors::Errors;
use anyhow::Error as AnyError;
use thiserror::Error;

// mod footer;
mod person;

pub use person::{Person, PersonBuilder};

type ValidationErrors = Errors<ValidationError>;

/// A trait for building validated objects from builder types.
///
/// Implementers of this trait provide a `build` method that attempts to build an instance of type `T`,
/// returning validation errors if the instance is invalid according to the model's rules.
pub trait Build<T> {
    /// Attempts to build an instance of type `T` from the builder.
    ///
    /// # Returns
    /// * `Ok(T)` if the builder contains valid data and the instance can be constructed.
    /// * `Err(ValidationErrors)` if validation fails for any fields in the builder.
    ///
    /// # Errors
    ///
    /// `ValidationErrors` contain information about the specific validation
    /// rule violations that occurred and how to fix them.
    fn build(&mut self) -> Result<T, ValidationErrors>;
}

/// Errors that can occur during validation of conventional commit components.
///
/// These errors represent specific validation failures that can occur
/// when validating conventional commit components.
///
/// The error variants are designed to provide clear, actionable feedback
/// about what validation rules were violated and how to fix them.
#[derive(Error, Debug)]
pub enum ValidationError {
    /// Error indicating a required field is missing.
    ///
    /// # Parameters
    ///
    /// * `0` - The name of the missing field
    #[error("field '{0}' is required")]
    MissingRequiredField(String),

    /// Error indicating a field contains an invalid value.
    ///
    /// # Parameters
    ///
    /// * `0` - The name of the field with the invalid value
    /// * `1` - The reason it's invalid
    #[error("field '{0}' has invalid value: {1}")]
    InvalidFieldValue(String, #[source] AnyError),
}

/// Implementation of `PartialEq` for `ValidationError` to enable comparison in tests.
///
/// Two `ValidationError` instances are considered equal if:
/// - They are both `MissingRequiredField` errors with the same field name
/// - They are both `InvalidFieldValue` errors with the same field name and error message
impl PartialEq for ValidationError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ValidationError::MissingRequiredField(a), ValidationError::MissingRequiredField(b)) => b == a,
            (ValidationError::InvalidFieldValue(a_str, a_err), ValidationError::InvalidFieldValue(b_str, b_err)) => a_str == b_str && a_err.to_string() == b_err.to_string(),
            (_, _) => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use anyhow::anyhow;
    use rstest::rstest;

    #[rstest]
    #[case::missing_required_field(ValidationError::MissingRequiredField("test".into()), "field 'test' is required")]
    #[case::invalid_field_value(ValidationError::InvalidFieldValue("test".into(), anyhow!("boom")), "field 'test' has invalid value: boom")]
    fn test_display_error(#[case] err: ValidationError, #[case] expect: impl Into<String>) {
        assert_eq!(expect.into(), format!("{err}"));
    }
}
