#![warn(missing_docs)]

/*!
Puccini
*/

mod cli;
mod create;
mod errors;
mod meta;
mod run;

use run::*;

use std::process::*;

/// Main.
pub fn main() -> ExitCode {
    kutil::cli::run::run(run)
}
