use super::{debug::*, format::*};

use {clap::*, std::path::*};

//
// Compile
//

/// Compile subcommand.
#[derive(Args)]
pub struct Compile {
    /// TOSCA or CSAR;
    /// can be a file path or a URL;
    /// when absent will read from stdin
    #[arg(verbatim_doc_comment)]
    pub input_file_or_url: Option<String>,

    /// output file path;
    /// when absent will write to stdout
    #[arg(long = "output", short = 'o', verbatim_doc_comment)]
    pub output_file: Option<PathBuf>,

    /// output format;
    /// when absent will try to use the output file extension
    #[arg(long = "format", short = 'f', verbatim_doc_comment, value_enum)]
    pub output_format: Option<OutputFormat>,

    /// plain output;
    /// avoid whitespace and colors
    #[arg(long = "plain", short = 'p', verbatim_doc_comment)]
    pub output_plain: bool,

    /// encode output to Base64;
    /// for "cbor" and "messagepack" formats only
    #[arg(long = "base64", verbatim_doc_comment)]
    pub output_base64: bool,

    /// compile into Floria directory
    #[arg(long = "directory")]
    pub directory: Option<String>,

    /// simulate instantiation into Floria directory;
    /// only if there are no compilation errors
    #[arg(long = "instantiate", short = 'i', verbatim_doc_comment)]
    pub instantiate: bool,

    /// simulate update of the Floria instance;
    /// requires `--instantiate`
    #[arg(long = "update", short = 'u', verbatim_doc_comment)]
    pub update: bool,

    /// disable annotations
    #[arg(long = "no-annotations")]
    pub no_annotations: bool,

    /// output debug information
    #[arg(long, short = 'd', value_enum)]
    pub debug: Option<Debug>,

    /// path to plugin override
    #[arg(long = "plugin")]
    pub plugin: Option<PathBuf>,

    /// plugin override is precompiled for this platform (.cwasm file)
    #[arg(long = "plugin-precompiled")]
    pub plugin_precompiled: bool,

    /// enable support for plugin debug information in Wasm
    #[cfg(feature = "wasm-debug-info")]
    #[arg(long = "plugin-debug", default_value_t = true, hide = true)]
    pub plugin_debug: bool,

    /// enable support for plugin debug information in Wasm
    #[cfg(not(feature = "wasm-debug-info"))]
    #[arg(long = "plugin-debug")]
    pub plugin_debug: bool,

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

    pub fn floria_directory(&self) -> floria::Directory {
        self.directory
            .as_ref()
            .map(|directory| {
                let Ok(directory) = directory.parse();
                directory
            })
            .unwrap_or_else(|| Default::default())
    }
}
