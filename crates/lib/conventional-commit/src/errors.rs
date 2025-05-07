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

use core::error::Error as CoreError;
use std::fmt::{Debug, Display, Formatter};

#[macro_export]
macro_rules! errors {
    ($($err:expr),+) => {
        Errors(vec![$($err),+])
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct Errors<E>(Vec<E>)
where
    E: CoreError + Debug + PartialEq;

impl<E> Display for Errors<E>
where
    E: CoreError + Debug + PartialEq,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.0.is_empty() {
            return Ok(());
        }

        write!(f, "error(s):")?;
        for err in &self.0 {
            write!(f, "\n  {err}")?;
        }

        Ok(())
    }
}

impl<E> CoreError for Errors<E>
where
    E: CoreError + Debug + PartialEq + 'static,
{
    fn source(&self) -> Option<&(dyn CoreError + 'static)> {
        self.0.first().map::<&(dyn CoreError + 'static), _>(|e| e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use prop::collection::vec;
    use proptest::prelude::*;
    use rstest::rstest;
    use thiserror::Error;

    #[rstest]
    #[case::single_error(
		errors!(TestError::String("boom".to_string())),
		vec![
			"error(s):",
			"  string error: boom"
		]
	)]
    #[case::multiple_errors(
        errors!(TestError::Numeric(1), TestError::Numeric(2)),
        vec![
			"error(s):",
			"  numeric error: 1",
            "  numeric error: 2",
        ],
	)]
    #[case::complex_error(
        errors!(TestError::Complex { msg: "failed".to_string(), number: 42 }),
        vec![
            "error(s):",
            "  complex error: failed: 42",
        ],
	)]
    #[case::struct_error(
        errors!(TestError::Struct(TestData {
            string: "test".to_string(),
            num: 100,
            float: 4.2
        })),
        vec![
            "error(s):",
            "  struct error: TestData { string: \"test\", num: 100, float: 4.2 }",
        ],
	)]
    #[case::nested_error(
        errors!(TestError::Nested(Box::new(TestError::String("inner error".to_string())))),
        vec![
            "error(s):",
            "  string error: inner error",
        ],
	)]
    #[case::mixed_errors(
        errors!(
            TestError::Numeric(42),
            TestError::String("text error".to_string()),
            TestError::Complex { msg: "complex".to_string(), number: 7 }
        ),
        vec![
            "error(s):",
            "  numeric error: 42",
            "  string error: text error",
            "  complex error: complex: 7",
        ],
	)]
    fn test_displays_error_list(#[case] errs: Errors<TestError>, #[case] expect_lines: Vec<&str>) {
        let expect = expect_lines.join("\n");
        assert_eq!(expect, format!("{errs}"));
    }

    #[rstest]
    #[case::single_error(errors!(TestError::String("boom".to_string())), TestError::String("boom".to_string()))]
    #[case::multiple_errors(errors!(TestError::Numeric(1), TestError::String("2".to_string())), TestError::Numeric(1))]
    #[case::complex_error(
        errors!(TestError::Complex { msg: "complex".to_string(), number: 8 }),
        TestError::Complex { msg: "complex".to_string(), number: 8 }
	)]
    #[case::struct_error(
        errors!(TestError::Struct(TestData {
            string: "data".to_string(),
            num: 200,
            float: 2.71
        })),
        TestError::Struct(TestData {
            string: "data".to_string(),
            num: 200,
            float: 2.71
        })
	)]
    #[case::nested_error(
        errors!(TestError::Nested(Box::new(TestError::Numeric(99)))),
        TestError::Nested(Box::new(TestError::Numeric(99)))
	)]
    fn test_get_first_error_as_source(#[case] errs: Errors<TestError>, #[case] expect: TestError) {
        let actual = errs
            .source()
            .expect("should have extracted source error")
            .downcast_ref::<TestError>()
            .expect("should be a TestError");

        assert_eq!(&expect, actual);
    }

    proptest! {
        #[test]
        fn prop_errors_display_has_correct_line_count(errors in vec(1..100i32, 1..50)) {
            let test_errors = errors.iter().map(|&i| TestError::Numeric(i)).collect::<Vec<_>>();
            let expected_line_count = test_errors.len() + 1;

            let errs = Errors(test_errors);

            let display_output = format!("{errs}");

            let line_count = display_output.lines().count();

            prop_assert_eq!(line_count, expected_line_count);
        }

        #[test]
        fn prop_errors_source_returns_first_error(errors in vec(1..100i32, 1..50)) {
            if errors.is_empty() {
                return Ok(());
            }

            let test_errors = errors.iter().map(|&i| TestError::Numeric(i)).collect::<Vec<_>>();
            let errors_struct = Errors(test_errors);

            let source = errors_struct.source()
                .expect("should have extracted source error")
                .downcast_ref::<TestError>()
                .expect("should be a TestError");

            prop_assert_eq!(source, &TestError::Numeric(errors[0]));
        }

        #[test]
        fn prop_errors_display_starts_with_header(errors in vec(1..100i32, 1..50)) {
            let test_errors = errors.iter().map(|&i| TestError::Numeric(i)).collect::<Vec<_>>();
            let errs = Errors(test_errors);

            let display_output = format!("{errs}");

            prop_assert!(display_output.starts_with("error(s):"));
        }

        #[test]
        fn prop_errors_display_has_correct_indentation(errors in vec(1..100i32, 1..50)) {
            let test_errors = errors.iter().map(|&i| TestError::Numeric(i)).collect::<Vec<_>>();
            let errs = Errors(test_errors);

            let display_output = format!("{errs}");
            let error_lines = display_output.lines().skip(1);

            for line in error_lines {
                prop_assert!(line.starts_with("  "));
            }
        }
    }

    #[derive(Error, Debug)]
    enum TestError {
        #[error("numeric error: {0}")]
        Numeric(i32),
        #[error("string error: {0}")]
        String(String),
        #[error("complex error: {msg}: {number}")]
        Complex { msg: String, number: u8 },
        #[error("struct error: {0:?}")]
        Struct(TestData),
        #[error(transparent)]
        Nested(#[from] Box<dyn CoreError>),
    }

    #[derive(Debug)]
    struct TestData {
        string: String,
        num: i32,
        float: f32,
    }

    impl PartialEq for TestData {
        fn eq(&self, other: &Self) -> bool {
            self.string == other.string && self.num == other.num && self.float.to_bits() == other.float.to_bits()
        }
    }

    impl PartialEq for TestError {
        fn eq(&self, other: &Self) -> bool {
            match (self, other) {
                (Self::Numeric(a), Self::Numeric(b)) => a == b,
                (Self::String(a), Self::String(b)) => a == b,
                (Self::Complex { msg: a_msg, number: a_num }, Self::Complex { msg: b_msg, number: b_num }) => a_msg == b_msg && a_num == b_num,
                (Self::Struct(a), Self::Struct(b)) => a == b,
                (Self::Nested(a), Self::Nested(b)) => format!("{a}") == format!("{b}"),
                _ => false,
            }
        }
    }
}
