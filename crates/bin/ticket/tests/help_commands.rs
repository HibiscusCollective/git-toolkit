/*
 * Copyright (C) 2025 Pierre Fouilloux, Hibiscus Collective
 *
 * This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
 * See the GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License along with this program.
 * If not, see https://www.gnu.org/licenses/.
 */

use std::{path::Path, sync::LazyLock};

use assert_cmd::{Command, cargo_bin};
use indoc::indoc;
use rstest::rstest;

static BINARY: LazyLock<&Path> = LazyLock::new(|| cargo_bin!("git-ticket"));

#[rstest]
#[case::short("-V")]
#[case::full("--version")]
fn test_prints_version_given_version_flag_is_set(#[case] flag: &str) {
    Command::new(BINARY.clone()).arg(flag).assert().success().stdout("Git Ticket 0.1.0\n");
}

#[rstest]
#[case::short("-h")]
#[case::full("--help")]
fn test_prints_help_given_help_flag_is_set(#[case] flag: &str) {
    Command::new(BINARY.clone()).arg(flag).assert().success().stdout(indoc!(
        "
            	Attaches ticket(s) to your commit messages.

            	Usage: git-ticket

            	Options:
            	  -h, --help     Print help
            	  -V, --version  Print version
		  	"
    ));
}
