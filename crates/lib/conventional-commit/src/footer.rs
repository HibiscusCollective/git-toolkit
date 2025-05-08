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
use derive_builder::Builder;
use std::fmt::{Display, Formatter};

#[derive(Builder)]
struct Footer {}

impl Footer {
    fn builder() -> FooterBuilder {
        FooterBuilder::default()
    }
}

impl Display for Footer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case::empty(Footer::builder(), "")]
    fn test_displays_footer_in_conventional_commit_format(#[case] builder: FooterBuilder, #[case] expect: impl Into<String>) {
        let footer = builder.build().expect("should have build a footer");
        assert_eq!(expect.into(), format!("{footer}"));
    }
}
