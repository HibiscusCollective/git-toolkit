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
fn test_prints_version_when_the_version_flag_is_set(#[case] flag: &str) {
    Command::new(BINARY.clone()).arg(flag).assert().success().stdout("Git Ticket 0.1.0\n");
}

#[test]
fn test_prints_help_with_short_description_when_the_short_help_flag_is_set() {
    Command::new(BINARY.clone()).arg("-h").assert().success().stdout(indoc!(
        "
            	Attaches ticket(s) to your commit messages.

            	Usage: git-ticket

            	Options:
            	  -h, --help     Print help (see more with '--help')
            	  -V, --version  Print version
		  	"
    ));
}

#[test]
fn test_prints_long_description_when_the_full_help_flag_is_set() {
    Command::new(BINARY.clone()).arg("--help").assert().success().stdout(indoc!(
        "
			Attaches ticket(s) to your commit messages. This is done via the git commit message template.
			Please ensure to set the path to this file in your git configuration using `git config --global commit.template ~/.gitmessage.txt`.
			By default the file will be created in your home directory with the name ~/.gitmessage.txt, but this can be overridden.

			Usage: git-ticket

			Options:
			  -h, --help
			          Print help (see a summary with '-h')

			  -V, --version
			          Print version
		"
    ));
}
