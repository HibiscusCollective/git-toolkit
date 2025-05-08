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
use thiserror::Error;

/// A trait for validating conventional commit components.
///
/// Types implementing this trait can be validated to ensure they meet
/// the requirements of the conventional commit specification.
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
#[derive(Error, Debug, PartialEq)]
pub enum ValidationError {
    /// Error indicating a required field is missing.
    ///
    /// # Parameters
    ///
    /// * `0` - The name of the missing field
    #[error("field '{0}' is required")]
    MissingRequiredField(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case::missing_required_field(ValidationError::MissingRequiredField("test".into()), "field 'test' is required")]
    fn test_display_error(#[case] err: ValidationError, #[case] expect: impl Into<String>) {
        assert_eq!(expect.into(), format!("{err}"));
    }
}
