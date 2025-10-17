use {clap::*, kutil::cli::clap::*, std::path::*};

// https://docs.rs/clap/latest/clap/_derive/index.html

//
// CLI
//

/// CSAR packaging tool
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
pub struct CLI {
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
    /// create a CSAR from a directory of artifacts
    Create(Create),

    /// extract an artifact from a CSAR
    Extract(Extract),

    /// extract meta information from a CSAR
    Meta(Meta),

    /// show the version of puccini-csar
    #[command(action = ArgAction::Version)]
    Version(Version),

    /// output the shell autocompletion script
    Completion(Completion),

    /// output the manual pages (in the troff format)
    Manual(Manual),
}

//
// Create
//

/// Create subcommand.
#[derive(Args)]
pub struct Create {
    /// path to CSAR file
    #[arg()]
    pub csar_path: PathBuf,

    /// path to directory
    #[arg()]
    pub directory: PathBuf,

    /// show this help
    #[arg(long, short = 'h', action = ArgAction::Help)]
    pub help: Option<bool>,
}

//
// Extract
//

/// Extract subcommand.
#[derive(Args)]
pub struct Extract {
    /// CSAR file;
    /// can be a file path or a URL;
    /// when absent will read from stdin
    #[arg(verbatim_doc_comment)]
    pub csar_path_or_url: Option<String>,

    /// artifact path in CSAR
    #[arg(long = "artifact", short = 'a')]
    pub artifact: Option<String>,

    /// output file path;
    /// when absent will write to stdout
    #[arg(long = "output", short = 'o', verbatim_doc_comment)]
    pub output_path: Option<PathBuf>,

    /// show this help
    #[arg(long, short = 'h', action = ArgAction::Help)]
    pub help: Option<bool>,
}

//
// Meta
//

/// Meta subcommand.
#[derive(Args)]
pub struct Meta {
    /// CSAR file;
    /// can be a file path or a URL;
    /// when absent will read from stdin
    #[arg(verbatim_doc_comment)]
    pub csar_path_or_url: Option<String>,

    /// output file path;
    /// when absent will write to stdout
    #[arg(long = "output", short = 'o', verbatim_doc_comment)]
    pub output_path: Option<PathBuf>,

    /// output format;
    /// when absent will be set to input format
    #[arg(long = "format", short = 'f', verbatim_doc_comment, value_enum)]
    pub output_format: Option<OutputFormat>,

    /// plain output;
    /// avoid whitespace and colors
    #[arg(long = "plain", short = 'p')]
    pub output_plain: bool,

    /// encode output to Base64;
    /// for "cbor" and "messagepack" formats
    #[arg(long = "base64", short = 'b', verbatim_doc_comment)]
    pub output_base64: bool,

    /// show this help
    #[arg(long, short = 'h', action = ArgAction::Help)]
    pub help: Option<bool>,
}

//
// OutputFormat
//

#[derive(Clone, ValueEnum)]
pub enum OutputFormat {
    YAML,
    JSON,
    XJSON,
    //XML,
    CBOR,
    #[value(name = "messagepack")]
    MessagePack,
    Depict,
}

impl OutputFormat {
    /// To Compris format.
    pub fn to_compris(&self) -> Option<compris::Format> {
        match self {
            OutputFormat::YAML => Some(compris::Format::YAML),
            OutputFormat::JSON => Some(compris::Format::JSON),
            OutputFormat::XJSON => Some(compris::Format::XJSON),
            //OutputFormat::XML => Some(compris::Format::XML),
            OutputFormat::CBOR => Some(compris::Format::CBOR),
            OutputFormat::MessagePack => Some(compris::Format::MessagePack),
            OutputFormat::Depict => None,
        }
    }
}

impl ToString for OutputFormat {
    fn to_string(&self) -> String {
        self.to_possible_value().expect("to_possible_value").get_name().into()
    }
}
