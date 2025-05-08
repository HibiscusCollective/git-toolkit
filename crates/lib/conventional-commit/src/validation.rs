/*
 * Git Toolkit extends Git's user experience to be more friendly while integrating with conventional commits specification
 * Copyright (c) 2025 Pierre Fouilloux, Hibiscus Collective
 *
 * This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
 * See the GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License along with this program.
 * If not, see https://www.gnu.org/licenses/.
 */

//! Validation for conventional commit components.
//!
//! This module provides a standardized way to validate components of conventional commits
//! and collect validation errors.

use crate::errors::Errors;
use anyhow::Error as AnyError;
use thiserror::Error;

/// A trait for validating conventional commit components.
///
/// Types implementing this trait can be validated to ensure they meet
/// the requirements of the conventional commit specification.
///
/// The `validate` method should collect all validation errors rather than
/// returning on the first error, allowing for more comprehensive feedback
/// during validation.
///
/// # Implementation Guidelines
///
/// When implementing this trait:
/// 1. Create an empty `Errors` collection
/// 2. Check all validation rules, adding errors to the collection as needed
/// 3. Return `Ok(())` if no errors were found, or `Err(errors)` otherwise
pub trait Validate {
    /// Validates the implementing type.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If validation passes
    /// * `Err(Errors<ValidationError>)` - A collection of validation errors if validation fails
    fn validate(&self) -> Result<(), Errors<ValidationError>>;
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
