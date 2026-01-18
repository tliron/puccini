use super::{debug::*, format::*};

use {clap::*, problemo::*, std::path::*};

//
// Compile
//

/// Compile subcommand.
#[derive(Args)]
pub struct Compile {
    /// TOSCA or CSAR;
    /// can be a file path or a URL;
    /// when absent will read TOSCA YAML from stdin
    #[arg(verbatim_doc_comment)]
    pub input_file_or_url: Option<String>,

    /// URL to Floria instance;
    /// when empty will use an in-memory instance
    #[arg(long = "floria", short = 'f', verbatim_doc_comment)]
    pub floria: Option<String>,

    /// compile into Floria directory
    #[arg(long = "directory")]
    pub directory: Option<String>,

    /// output file path;
    /// when absent will write to stdout
    #[arg(long = "output-file", short = 'o', verbatim_doc_comment)]
    pub output_file: Option<PathBuf>,

    /// output format;
    /// when absent will try to use the output file extension
    #[arg(long = "format", verbatim_doc_comment, value_enum)]
    pub output_format: Option<OutputFormat>,

    /// plain output;
    /// avoid colors and whitespace
    #[arg(long = "plain", short = 'p', verbatim_doc_comment)]
    pub output_plain: bool,

    /// encode output to Base64;
    /// for "cbor" and "messagepack" formats only
    #[arg(long = "base64", verbatim_doc_comment)]
    pub output_base64: bool,

    /// set the TOSCA service inputs as a YAML or JSON map;
    /// when used multiple times the maps will be merged;
    /// requires `--instantiate`
    #[arg(long = "inputs", verbatim_doc_comment)]
    pub inputs: Vec<String>,

    /// load the TOSCA service inputs as a YAML or JSON map;
    /// can be a file path or a URL;
    /// when used multiple times the maps will be merged;
    /// requires `--instantiate`
    #[arg(long = "inputs-from", verbatim_doc_comment)]
    pub inputs_from: Vec<String>,

    /// output a TOSCA service output;
    /// can be used multiple times;
    /// requires `--instantiate`
    #[arg(long = "output", verbatim_doc_comment)]
    pub outputs: Vec<String>,

    /// instantiate the compiled Floria template
    #[arg(long = "instantiate", short = 'i')]
    pub instantiate: bool,

    /// propagate an event on the Floria instance;
    /// when used multiple times the events will be propagates in sequence;
    /// requires `--instantiate`
    #[arg(long = "event", verbatim_doc_comment)]
    pub events: Vec<String>,

    /// alias for `--event=floria:update`
    #[arg(long = "update", short = 'u', verbatim_doc_comment)]
    pub update: bool,

    /// whether to use YAML annotations
    #[arg(long = "annotations", action = clap::ArgAction::Set, default_value_t = true)]
    pub annotations: bool,

    /// output debug information
    #[arg(long, short = 'd', value_enum)]
    pub debug: Option<Debug>,

    /// URL to TOSCA plugin override
    #[arg(long = "tosca-plugin")]
    pub tosca_plugin: Option<String>,

    /// whether the `--tosca-plugin` is precompiled (.cwasm file);
    /// when absent will try to use the file extension
    #[arg(long = "tosca-plugin-precompiled", action = clap::ArgAction::Set, verbatim_doc_comment)]
    pub tosca_plugin_precompiled: Option<bool>,

    /// whether to support debug information in Wasm
    #[cfg(feature = "wasm-debug")]
    #[arg(long = "wasm-debug", default_value_t = true, hide = true)]
    pub wasm_debug: bool,

    /// whether to support debug information in Wasm
    #[cfg(not(feature = "wasm-debug"))]
    #[arg(long = "wasm-debug", action = clap::ArgAction::Set, default_value_t = false)]
    pub wasm_debug: bool,

    /// whether to cache compiled Wasm to disk
    #[cfg(feature = "plugins")]
    #[arg(long = "wasm-cache", action = clap::ArgAction::Set, default_value_t = true)]
    pub wasm_cache: bool,

    /// show this help
    #[arg(long, short = 'h', action = ArgAction::Help)]
    pub help: Option<bool>,
}

impl Compile {
    /// True if we should run completion phase.
    pub fn should_complete(&self) -> bool {
        match self.debug {
            Some(Debug::Namespaces | Debug::Parsed) => false,
            _ => true,
        }
    }

    /// True if we should run compilation phase.
    pub fn should_compile(&self) -> bool {
        match self.debug {
            Some(Debug::Completed | Debug::Namespaces | Debug::Parsed) => false,
            _ => true,
        }
    }

    pub fn floria_directory(&self) -> Result<floria::Directory, Problem> {
        Ok(match &self.directory {
            Some(directory) => directory.parse()?,
            None => Default::default(),
        })
    }
}
