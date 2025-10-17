use {clap::*, std::path::*};

//
// Meta
//

/// Meta subcommand.
#[derive(Args)]
pub struct Meta {
    /// path to source directory
    pub directory: PathBuf,

    /// override or set the "Created-By" key
    #[arg(long = "created-by")]
    pub created_by: Option<String>,

    /// override or set the "Entry-Definitions" key
    #[arg(long = "entry-definitions")]
    pub entry_definitions: Option<String>,

    /// add an entry to the "Other-Definitions" key
    #[arg(long = "other-definitions")]
    pub other_definitions: Vec<String>,

    /// maximum number of columns;
    /// for "text" format only
    #[arg(long = "max-columns", default_value_t = 80, verbatim_doc_comment)]
    pub max_columns: usize,

    /// overwrite TOSCA.meta if it already exists
    #[arg(long, short = 'f')]
    pub force: bool,

    /// enable dry run;
    /// do everything except write the TOSCA.meta
    #[arg(long = "dry-run", short = 'd', verbatim_doc_comment)]
    pub dry_run: bool,

    /// show this help
    #[arg(long, short = 'h', action = ArgAction::Help)]
    pub help: Option<bool>,
}
