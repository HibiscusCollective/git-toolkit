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
//! This module provides functionality for representing and validating a model
//! in the context of Git commits, with support for name and email validation
//! according to conventional commit standards.
//!
//! A `Person` typically represents an author or committer in a Git commit,
//! consisting of a name and an optional email address.

use crate::{
    errors::Errors,
    model::{Build, ValidationError, ValidationErrors},
};
use anyhow::anyhow;
use derive_builder::Builder;
use email_address::EmailAddress;
use std::{
    default::Default,
    fmt::{Display, Formatter},
    str::FromStr,
};

const DEFAULT_RELATIONSHIP: &str = "Co-Authored-By";

/// Represents a model in a Git commit.
///
/// A `Person` consists of a name and an optional email address. The name is required,
/// and if an email is provided, it must be a valid email address according to RFC 5322.
#[derive(Builder, Clone, Debug)]
#[builder(build_fn(skip))]
pub struct Person {
    /// The name of the model.
    #[builder(setter(custom))]
    name: String,
    /// The model's relationship to the commit (ex: co-author, reviewer, etc.), defaults to 'Co-Authored-By'.
    #[builder(setter(into), default=DEFAULT_RELATIONSHIP.into())]
    relationship: String,
    /// The optional email address of the model.\
    #[builder(setter(into, strip_option), default)]
    email: Option<String>,
}

impl Person {
    pub fn builder(name: impl Into<String>) -> PersonBuilder {
        PersonBuilder {
            name: Some(name.into()),
            ..Default::default()
        }
    }

    /// Returns the name of the model.
    ///
    /// # Returns
    ///
    /// The name of the model as a string slice.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the relationship of the model to the commit
    ///
    /// # Returns
    ///
    /// A string representing the relationship, ex: "Co-Authored-By"
    pub fn relationship(&self) -> &str {
        &self.relationship
    }

    /// Returns the email of the model, if available.
    ///
    /// # Returns
    ///
    /// An optional reference to the email string.
    pub fn email(&self) -> Option<&str> {
        self.email.as_deref()
    }
}

impl PersonBuilder {
    fn validate_name(&mut self) -> Result<String, ValidationError> {
        let err = ValidationError::MissingRequiredField("name".into());

        if let Some(name) = self.name.clone() {
            if name.is_empty() { Err(err) } else { Ok(name) }
        } else {
            Err(err)
        }
    }

    fn validate_email(&mut self) -> Result<Option<String>, ValidationError> {
        if let Some(Some(email)) = self.email.clone() {
            if let Err(e) = EmailAddress::from_str(email.as_str()) {
                Err(ValidationError::InvalidFieldValue("email".to_string(), anyhow!(e)))
            } else {
                Ok(Some(email))
            }
        } else {
            Ok(None)
        }
    }

    fn get_relationship_or_default(&mut self) -> String {
        if let Some(relationship) = self.relationship.clone() {
            if relationship.is_empty() { DEFAULT_RELATIONSHIP.to_string() } else { relationship }
        } else {
            DEFAULT_RELATIONSHIP.to_string()
        }
    }
}

/// Implementation of the `Validate` trait for `PersonBuilder`.
///
/// This implementation validates that:
/// - The name is not empty
/// - If an email is provided, it is a valid email address according to RFC 5322
impl Build<Person> for PersonBuilder {
    /// Validates the `Person` instance.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If validation passes
    /// * `Err(Errors<ValidationError>)` - A collection of validation errors if validation fails
    fn build(&mut self) -> Result<Person, ValidationErrors> {
        let mut errs = Errors::new();

        let name = self.validate_name().unwrap_or_else(|e| {
            errs.append(e);
            String::new()
        });

        let relationship = self.get_relationship_or_default();

        let email = self.validate_email().unwrap_or_else(|e| {
            errs.append(e);
            None
        });

        if errs.is_empty() { Ok(Person { name, relationship, email }) } else { Err(errs) }
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
/// # use std::fmt::{format, Display};
/// # use conventional_commit::model::{Build, Person};
/// #
/// # let person_name_only = Person::builder("Alice Bob").build().unwrap();
/// # let person_with_email = Person::builder("Charlie Delta").email("charlie@delta.io").build().unwrap();
/// # let person_with_email_and_relationship = Person::builder("Charlie Delta").email("charlie@delta.io").relationship("Reviewer").build().unwrap();
///
/// assert_eq!(format!("{}", person_name_only), "Co-Authored-By: Alice Bob");
/// assert_eq!(format!("{}", person_with_email), "Co-Authored-By: Charlie Delta <charlie@delta.io>");
/// assert_eq!(format!("{}", person_with_email_and_relationship), "Reviewer: Charlie Delta <charlie@delta.io>");
/// ```
impl Display for Person {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.relationship, self.name)?;

        if let Some(email) = self.email.clone() {
            write!(f, " <{email}>")?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::{model::ValidationError, multi_error};
    use email_address::Error as EmailError;
    use rstest::rstest;

    #[rstest]
    #[case::when_name_and_email_empty(Person::builder(""), multi_error!(ValidationError::MissingRequiredField("name".to_string())))]
    #[case::when_only_name_empty(Person::builder("").email("test@test.com").clone(), multi_error!(ValidationError::MissingRequiredField("name".to_string())))]
    #[case::when_only_email_invalid(Person::builder("Alice Bob").email("invalid").clone(), multi_error!(ValidationError::InvalidFieldValue("email".to_string(), EmailError::MissingSeparator.into())))]
    #[case::when_name_is_empty_and_email_invalid(Person::builder("").email("invalid").clone(), multi_error!(ValidationError::MissingRequiredField("name".to_string()), ValidationError::InvalidFieldValue("email".to_string(), EmailError::MissingSeparator.into())))]
    fn test_return_error_building_person(#[case] mut person: PersonBuilder, #[case] expect: ValidationErrors) {
        let errs = person.build().expect_err("should have failed");
        assert_eq!(expect, errs, "expected: {expect}\n but got: {errs}");
    }

    #[rstest]
    #[case::name_only(Person::builder("Alice Bob").build().expect("should have built a person"), "Co-Authored-By: Alice Bob")]
    #[case::name_and_email(Person::builder("Alice Bob").email("alice.bob@test.io").build().expect("should have built a person"), "Co-Authored-By: Alice Bob <alice.bob@test.io>")]
    #[case::custom_relationship(Person::builder("Alice Bob").relationship("Reviewer").build().expect("should have built a person"), "Reviewer: Alice Bob")]
    #[case::full(Person::builder("Alice Bob").relationship("Reviewer").email("alice.bob@test.io").build().expect("should have built a person"), "Reviewer: Alice Bob <alice.bob@test.io>")]
    fn test_display_person(#[case] person: Person, #[case] expect: impl Into<String>) {
        assert_eq!(expect.into(), format!("{person}"));
    }

    #[rstest]
    #[case::name_only(Person::builder("Alice Bob").build().expect("should have built a person"), "Alice Bob")]
    #[case::name_and_email(Person::builder("Alice Bob").email("alice.bob@test.io").build().expect("should have built a person"), "Alice Bob")]
    fn test_name_getter(#[case] person: Person, #[case] expect: impl Into<String>) {
        assert_eq!(expect.into(), person.name());
    }

    #[rstest]
    #[case::no_email(Person::builder("Alice Bob").build().expect("should have built a person"), Option::<String>::None)]
    #[case::with_email(Person::builder("Alice Bob").email("alice.bob@test.io").build().expect("should have built a person"), Some("alice.bob@test.io"))]
    fn test_email_getter(#[case] person: Person, #[case] expect: Option<impl Into<String>>) {
        match (expect, person.email()) {
            (None, Some(_)) => panic!("expected no email but got one"),
            (Some(_), None) => panic!("expected an email but got none"),
            (Some(expect), Some(email)) => assert_eq!(expect.into(), email),
            (None, None) => {}
        }
    }

    #[rstest]
    #[case::no_email(Person::builder("Alice Bob").build().expect("should have built a person"), "Co-Authored-By")]
    #[case::with_email(Person::builder("Alice Bob").relationship("Reviewer").build().expect("should have built a person"), "Reviewer")]
    fn test_relationship_getter(#[case] person: Person, #[case] expect: &str) {
        assert_eq!(expect, person.relationship());
    }
}
