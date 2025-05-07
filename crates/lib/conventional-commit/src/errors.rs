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
        Errors(&[$($err),+])
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct Errors<E: CoreError + Debug + PartialEq + 'static>(&'static [E]);

impl<E: CoreError + Debug + PartialEq> Display for Errors<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.0.len() == 1 {
            return write!(f, "{}", self.0[0]);
        }

        for err in self.0.iter() {
            writeln!(f, "{}", err)?
        }

        Ok(())
    }
}

impl<E: CoreError + Debug + PartialEq + 'static> CoreError for Errors<E> {
    fn source(&self) -> Option<&(dyn CoreError + 'static)> {
        self.0.first().map::<&(dyn CoreError + 'static), _>(|e| e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;
    use thiserror::Error;

    #[rstest]
    #[case::single_error(errors!(TestError("boom")), vec!["test error: boom"])]
    #[case::multiple_errors(
        errors!(TestError("1"), TestError("2")), 
        vec![
			"test error: 1",
            "test error: 2",
			""
        ],
    )]
    fn test_displays_error_list(#[case] errs: Errors<TestError>, #[case] expect_lines: Vec<&str>) {
        let expect = expect_lines.join("\n");
        assert_eq!(expect, format!("{}", errs))
    }

    #[rstest]
    #[case::single_error(errors!(TestError("boom")), TestError("boom"))]
    #[case::multiple_errors(errors!(TestError("1"), TestError("2")), TestError("1"))]
    fn test_get_first_error_as_source(#[case] errs: Errors<TestError>, #[case] expect: TestError) {
        let actual = errs
            .source()
            .expect("should have extracted source error")
            .downcast_ref::<TestError>()
            .expect("should be a TestError");

        assert_eq!(&expect, actual)
    }

    #[derive(Error, Debug, PartialEq)]
    #[error("test error: {0}")]
    struct TestError<'a>(&'a str);
}
