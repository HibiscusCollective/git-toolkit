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
use thiserror::Error;

#[macro_export]
macro_rules! errors {
    ($($err:expr),+) => {
        Errors(vec![$($err),+])
    }
}

#[derive(Error, Debug, PartialEq)]
struct Errors<E: CoreError + Debug + PartialEq>(Vec<E>);

impl<E: CoreError + Debug + PartialEq> Display for Errors<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0[0])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case::single_error(errors!(TestError("boom")), "test error: boom")]
    fn test_displays_error_list(#[case] errs: Errors<TestError>, #[case] expect: &str) {
        assert_eq!(expect, format!("{}", errs))
    }
    #[derive(Error, Debug, PartialEq)]
    #[error("test error: {0}")]
    struct TestError<'a>(&'a str);
}
