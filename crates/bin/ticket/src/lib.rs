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
use clap::Parser;
use std::ffi::OsString;

#[derive(Parser)]
#[command(name = "Git Ticket")]
#[command(version)]
#[command(about = "Attaches ticket(s) to your commit messages.")]
pub struct Args {}

impl Args {
    pub fn parse_from_args<ITER, ARG>(args: ITER) -> Self
    where
        ITER: IntoIterator<Item = ARG>,
        ARG: Into<OsString> + Clone,
    {
        Args::parse_from(args)
    }
}
