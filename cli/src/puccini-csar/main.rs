#![warn(missing_docs)]

/*!
Puccini
*/

mod cli;
mod create;
mod errors;
mod extract;
mod meta;
mod run;
mod utils;

use run::*;

use std::process::*;

/// Main.
pub fn main() -> ExitCode {
    kutil::cli::run::run(run)
}
