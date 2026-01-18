#![warn(missing_docs)]

/*!
Puccini
*/

mod commands;
mod run;

use run::*;

use {mimalloc::*, std::process::*};

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

/// Main.
pub fn main() -> ExitCode {
    kutil::cli::run::run(run)
}
