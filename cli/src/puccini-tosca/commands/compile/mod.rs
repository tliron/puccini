mod command;
mod debug;
mod dialects;
mod format;
mod inputs;
#[cfg(feature = "plugins")]
mod instantiate;
mod output;
mod run;

pub use command::*;
