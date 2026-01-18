use super::format::*;

use {clap::*, clap_num::*, puccini_csar::creator::*, std::path::*};

//
// Create
//

/// Create subcommand.
#[derive(Args)]
pub struct Create {
    /// path to source directory
    pub directory: PathBuf,

    /// path to target CSAR file
    pub file: Option<PathBuf>,

    /// archive format;
    /// when absent will attempt to select according to target file extension
    /// or default to "tarball" when target file not specified
    #[arg(long = "format", short = 'f', verbatim_doc_comment, value_enum)]
    pub format: Option<CsarFormat>,

    /// compression level from 1 to 10;
    /// 1 = least compression, fastest;
    /// 10 = most compression, slowest;
    /// leave empty to use format's default
    #[arg(long = "compression", short = 'C', value_parser = compression_level_parser, verbatim_doc_comment)]
    pub compression_level: Option<usize>,

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
    #[arg(long = "max-columns", short = 'c', default_value_t = 80)]
    pub max_columns: usize,

    /// enable dry run;
    /// do everything except write the CSAR
    #[arg(long = "dry-run", short = 'd', verbatim_doc_comment)]
    pub dry_run: bool,

    /// show this help
    #[arg(long, short = 'h', action = ArgAction::Help)]
    pub help: Option<bool>,
}

impl Create {
    pub fn csar_format(&self) -> Option<Format> {
        self.format.as_ref().map(|format| format.to_puccini())
    }

    pub fn compression_level(&self) -> Option<CompressionLevel> {
        self.compression_level.map(CompressionLevel::new_unchecked)
    }
}

fn compression_level_parser(representation: &str) -> Result<usize, String> {
    number_range(representation, 1, 10)
}
