use super::{create::*, inspect::*, meta::*};

use {clap::*, kutil::cli::clap::*, std::path::*};

//
// Root
//

/// CSAR creator and validator
#[derive(Parser)]
#[command(
    name = "puccini-csar",
    version,
    propagate_version = true,
    disable_help_flag = true,
    disable_help_subcommand = true,
    disable_version_flag = true,
    arg_required_else_help = true,
    styles = clap_styles())
]
pub struct Root {
    #[command(subcommand)]
    pub subcommand: Option<SubCommand>,

    /// suppress console output
    #[arg(long, short = 'q', global = true)]
    pub quiet: bool,

    /// add a log verbosity level;
    /// can be used 3 times
    #[arg(long, short, verbatim_doc_comment, action = ArgAction::Count, global = true)]
    pub verbose: u8,

    /// colorize output
    #[arg(long = "colorize", short = 'z', default_value_t = Colorize::True, value_enum, global = true)]
    pub colorize: Colorize,

    /// log to file path;
    /// defaults to stderr
    #[arg(long, long = "log", short = 'l', verbatim_doc_comment, global = true)]
    pub log_path: Option<PathBuf>,

    // TODO not used
    /// timeout in seconds;
    /// 0 for no timeout
    #[arg(long, short = 't', verbatim_doc_comment, default_value_t = 0.0, global = true)]
    pub timeout: f64,

    /// show this help
    #[arg(long, short = 'h', action = ArgAction::Help)]
    pub help: Option<bool>,
}

//
// SubCommand
//

#[derive(Subcommand)]
#[command()]
pub enum SubCommand {
    /// create a CSAR from a source directory
    Create(Create),

    /// create a TOSCA.meta file in a source directory
    Meta(Meta),

    /// inspect and extract meta information from a CSAR
    Inspect(Inspect),

    /// show the version of puccini-csar
    #[command(action = ArgAction::Version)]
    Version(Version),

    /// output the shell autocompletion script
    Completion(Completion),

    /// output the manual pages (in the troff format)
    Manual(Manual),
}
