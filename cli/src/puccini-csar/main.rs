#![warn(missing_docs)]

/*!
Puccini
*/

mod commands;
mod errors;
mod run;

use run::*;

use std::process::*;

/// Main.
pub fn main() -> ExitCode {
    kutil::cli::run::run(run)
}
