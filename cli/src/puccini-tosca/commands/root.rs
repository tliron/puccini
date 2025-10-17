use super::compile::*;

use {clap::*, kutil::cli::clap::*, std::path::*};

//
// Root
//

/// TOSCA parser and compiler
#[derive(Parser)]
#[command(
    name = "puccini-tosca",
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
// SubCommands
//

#[derive(Subcommand)]
#[command()]
pub enum SubCommand {
    /// compile a TOSCA service template to a Floria template
    Compile(Compile),

    /// show the version of puccini-tosca
    Version(Version),

    /// output the shell auto-completion script
    Completion(Completion),

    /// output the manual pages (in the troff format)
    Manual(Manual),
}
