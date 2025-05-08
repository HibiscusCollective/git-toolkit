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

//! Person representation for conventional commits.
//!
//! This module provides functionality for representing and validating a person
//! in the context of Git commits, with support for name and email validation
//! according to conventional commit standards.
//!
//! A `Person` typically represents an author or committer in a Git commit,
//! consisting of a name and an optional email address.

use crate::errors::Errors;
use crate::validation::{Validate, ValidationError};
use anyhow::anyhow;
use email_address::EmailAddress;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

/// Represents a person in a Git commit.
///
/// A `Person` consists of a name and an optional email address. The name is required,
/// and if an email is provided, it must be a valid email address according to RFC 5322.
#[derive(Debug)]
pub struct Person {
    /// The name of the person.
    name: String,
    /// The optional email address of the person.
    email: Option<String>,
}

impl Person {
    /// Creates a new `Person` instance from a name and optional email.
    ///
    /// # Parameters
    ///
    /// * `name` - The name of the person
    /// * `email` - An optional email address for the person
    ///
    /// # Returns
    ///
    /// * `Ok(Person)` - A valid Person instance
    /// * `Err(Errors<ValidationError>)` - A collection of validation errors if validation fails
    pub fn parse(name: impl Into<String>, email: Option<impl Into<String>>) -> Result<Self, Errors<ValidationError>> {
        let person = Person {
            name: name.into(),
            email: email.map(Into::into),
        };

        person.validate().map(|()| person)
    }

    /// Returns the name of the person.
    ///
    /// # Returns
    ///
    /// The name of the person as a string slice.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the email of the person, if available.
    ///
    /// # Returns
    ///
    /// An optional reference to the email string.
    pub fn email(&self) -> Option<&str> {
        self.email.as_deref()
    }
}

/// Implementation of the `Validate` trait for `Person`.
///
/// This implementation validates that:
/// - The name is not empty
/// - If an email is provided, it is a valid email address according to RFC 5322
impl Validate for Person {
    /// Validates the `Person` instance.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If validation passes
    /// * `Err(Errors<ValidationError>)` - A collection of validation errors if validation fails
    fn validate(&self) -> Result<(), Errors<ValidationError>> {
        let mut errs = Errors::new();
        if self.name.is_empty() {
            errs.append(ValidationError::MissingRequiredField("name".to_string()));
        }

        if let Some(email) = self.email.clone() {
            if let Err(e) = EmailAddress::from_str(email.as_str()) {
                errs.append(ValidationError::InvalidFieldValue("email".to_string(), anyhow!(e)));
            }
        }

        if errs.is_empty() { Ok(()) } else { Err(errs) }
    }
}

/// Implementation of the `Display` trait for `Person`.
///
/// This implementation formats a `Person` instance as a string in the standard Git author/committer format:
/// - If only a name is present, it returns just the name
/// - If both name and email are present, it returns the format "Name <email>"
///
/// # Examples
///
/// ```
/// # use conventional_commit::person::Person;
/// # let person_name_only = Person::parse("Alice Bob", None::<String>).unwrap();
/// # let person_with_email = Person::parse("Charlie Delta", Some("charlie@delta.io")).unwrap();
///
/// assert_eq!(format!("{}", person_name_only), "Alice Bob");
/// assert_eq!(format!("{}", person_with_email), "Charlie Delta <charlie@delta.io>");
/// ```
impl Display for Person {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)?;

        if let Some(email) = self.email.clone() {
            write!(f, " <{email}>")?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::{errors::Errors, multi_error, validation::ValidationError};
    use email_address::Error as EmailError;
    use rstest::rstest;

    #[rstest]
    #[case::when_name_and_email_empty("", Option::<String>::None, multi_error!(ValidationError::MissingRequiredField("name".to_string())))]
    #[case::when_only_name_empty("", Some("test@test.com"), multi_error!(ValidationError::MissingRequiredField("name".to_string())))]
    #[case::when_only_email_invalid("Alice Bob", Some("invalid"), multi_error!(ValidationError::InvalidFieldValue("email".to_string(), EmailError::MissingSeparator.into())))]
    #[case::when_name_is_empty_and_email_invalid("", Some("invalid"), multi_error!(ValidationError::MissingRequiredField("name".to_string()), ValidationError::InvalidFieldValue("email".to_string(), EmailError::MissingSeparator.into())))]
    fn test_return_error_parsing_person(#[case] name: impl Into<String>, #[case] email: Option<impl Into<String>>, #[case] expect: Errors<ValidationError>) {
        let errs = Person::parse(name, email).expect_err("should have failed to create Person");
        assert_eq!(expect, errs);
    }

    #[rstest]
    #[case::name_only(Person{name: "Alice Bob".into(), email: None}, "Alice Bob")]
    #[case::name_and_email(Person{name: "Alice Bob".into(), email: Some("alice.bob@test.io".into())}, "Alice Bob <alice.bob@test.io>")]
    fn test_display_person(#[case] person: Person, #[case] expect: impl Into<String>) {
        assert_eq!(expect.into(), format!("{person}"));
    }

    #[rstest]
    #[case::name_only("John Doe", Option::<String>::None)]
    #[case::name_and_email("Jane Smith", Some("jane.smith@example.com"))]
    fn test_name_getter(#[case] name: impl Into<String>, #[case] email: Option<impl Into<String>>) {
        let person = Person::parse(name, email).expect("should not have failed to parse");
        assert_eq!(person.name, person.name());
    }

    #[rstest]
    #[case::no_email("John Doe", Option::<String>::None)]
    #[case::with_email("Jane Smith", Some("jane.smith@example.com"))]
    fn test_email_getter(#[case] name: impl Into<String>, #[case] email: Option<impl Into<String>>) {
        let person = Person::parse(name, email).expect("should not have failed to parse");
        assert_eq!(person.email.as_deref(), person.email());
    }
}
