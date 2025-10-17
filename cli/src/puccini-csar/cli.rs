use puccini_csar::creator::*;

use {clap::*, clap_num::*, kutil::cli::clap::*, std::path::*};

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

    /// validate and extract meta information from a CSAR
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
    /// path to source directory
    #[arg()]
    pub directory: PathBuf,

    /// path to target CSAR file
    #[arg()]
    pub file: Option<PathBuf>,

    /// CSAR format;
    /// when absent will be selected according to CSAR file extension
    #[arg(long = "format", short = 'f', verbatim_doc_comment, value_enum)]
    pub format: Option<CsarFormat>,

    /// archive compression level from 1 to 10;
    /// 1 = least compression, fastest;
    /// 10 = most compression, slowest
    #[arg(long = "compression", short = 'c', default_value_t = DEFAULT_COMPRESSION_LEVEL, value_parser = compression_level_parser, verbatim_doc_comment)]
    pub compression_level: usize,

    /// override or set the "Created-By" key
    #[arg(long = "created-by")]
    pub created_by: Option<String>,

    /// override or set the "Entry-Definitions" key
    #[arg(long = "entry-definitions")]
    pub entry_definitions: Option<String>,

    /// add an entry to the "Other-Definitions" key
    #[arg(long = "other-definitions")]
    pub other_definitions: Vec<String>,

    /// maximum number of columns for TOSCA.meta file
    #[arg(long = "max-columns", default_value_t = 80)]
    pub max_columns: usize,

    /// dry run;
    /// do everything except create CSAR
    #[arg(long = "dry-run", short = 'd', verbatim_doc_comment)]
    pub dry_run: bool,

    /// show this help
    #[arg(long, short = 'h', action = ArgAction::Help)]
    pub help: Option<bool>,
}

fn compression_level_parser(representation: &str) -> Result<usize, String> {
    number_range(representation, 1, 10)
}

//
// CsarFormat
//

#[derive(Clone, ValueEnum)]
pub enum CsarFormat {
    #[value(name = "tar")]
    Tarball,
    #[value(name = "tgz")]
    GzipTarball,
    #[value(name = "zip")]
    Zip,
}

impl CsarFormat {
    /// To Puccini format.
    pub fn to_puccini(&self) -> puccini_csar::creator::Format {
        match self {
            CsarFormat::Tarball => puccini_csar::creator::Format::Tarball,
            CsarFormat::GzipTarball => puccini_csar::creator::Format::GzipTarball,
            CsarFormat::Zip => puccini_csar::creator::Format::ZIP,
        }
    }
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
    pub file_or_url: Option<String>,

    /// output file path;
    /// when absent will write to stdout
    #[arg(long = "output", short = 'o', verbatim_doc_comment)]
    pub output_file: Option<PathBuf>,

    /// output format;
    /// when absent will try to use the output file extension
    #[arg(long = "format", short = 'f', value_enum, verbatim_doc_comment)]
    pub output_format: Option<OutputFormat>,

    /// plain output;
    /// avoid whitespace and colors
    #[arg(long = "plain", short = 'p', verbatim_doc_comment)]
    pub output_plain: bool,

    /// encode output to Base64;
    /// for "cbor" and "messagepack" formats only
    #[arg(long = "base64", verbatim_doc_comment)]
    pub output_base64: bool,

    /// maximum number of columns;
    /// for "text" format only
    #[arg(long = "max-columns", default_value_t = 80, verbatim_doc_comment)]
    pub max_columns: usize,

    /// show this help
    #[arg(long, short = 'h', action = ArgAction::Help)]
    pub help: Option<bool>,
}

//
// OutputFormat
//

#[derive(Clone, ValueEnum)]
pub enum OutputFormat {
    Text,
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
            _ => None,
        }
    }
}

impl ToString for OutputFormat {
    fn to_string(&self) -> String {
        self.to_possible_value().expect("to_possible_value").get_name().into()
    }
}
