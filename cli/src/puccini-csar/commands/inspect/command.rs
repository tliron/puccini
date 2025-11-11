use {clap::*, std::path::*};

//
// Inspect
//

/// Inspect subcommand.
#[derive(Args)]
pub struct Inspect {
    /// CSAR file;
    /// can be a file path or a URL;
    /// when absent will read from stdin
    #[arg(verbatim_doc_comment)]
    pub file_or_url: Option<String>,

    /// output file path;
    /// when absent will write to stdout
    #[arg(long = "output-file", short = 'o', verbatim_doc_comment)]
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
