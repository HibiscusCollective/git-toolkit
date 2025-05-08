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
use crate::validation::{Validate, ValidationError};

#[derive(Debug)]
pub struct Person {
    name: String,
    email: String,
}

impl Person {
    fn parse(name: impl Into<String>, email: impl Into<String>) -> Result<Self, Errors<ValidationError>> {
        let person = Person {
            name: name.into(),
            email: email.into(),
        };

        person.validate().map(|()| person)
    }
}

impl Validate for Person {
    fn validate(&self) -> Result<(), Errors<ValidationError>> {
        let mut errs = Errors::new();
        if self.name.is_empty() {
            errs.append(ValidationError::MissingRequiredField("name".to_string()));
        }

        if errs.is_empty() { Ok(()) } else { Err(errs) }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::{errors::Errors, multi_error, validation::ValidationError};
    use rstest::rstest;

    #[rstest]
    #[case::when_name_and_email_empty("", "", multi_error!(ValidationError::MissingRequiredField("name".to_string())))]
    #[case::when_only_name_empty("", "test@test.com", multi_error!(ValidationError::MissingRequiredField("name".to_string())))]
    fn test_return_error_parsing_person(#[case] name: impl Into<String>, #[case] email: impl Into<String>, #[case] expect: Errors<ValidationError>) {
        let errs = Person::parse(name, email).expect_err("should have failed to create Person");
        assert_eq!(expect, errs);
    }
}
