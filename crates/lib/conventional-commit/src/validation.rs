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
use crate::errors::Errors;
use thiserror::Error;

pub trait Validate {
    fn validate() -> Result<(), Errors<ValidationError>>;
}

#[derive(Error, Debug, PartialEq)]
pub enum ValidationError {
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
