use super::{super::meta::*, compression_level::*, format::*};

use {indicatif::*, std::mem::*};

//
// CsarCreator
//

/// CSAR creator.
#[derive(Clone, Debug)]
pub struct CsarCreator {
    /// Format.
    pub format: Option<Format>,

    /// Compression level, from 1 to 10.
    pub compression: CompressionLevel,

    /// Created by.
    pub created_by: Option<String>,

    /// Entry definitions.
    pub entry_definitions: Option<String>,

    /// Additional other definitions.
    pub additional_other_definitions: Vec<String>,

    /// Maximum number of columns for formatting `TOSCA.meta` file.
    pub max_columns: Option<usize>,

    /// Whether to display progress bar.
    pub progress_bar: Option<ProgressBar>,

    /// Whether to display progress bar in color.
    pub colorize: bool,
}

impl CsarCreator {
    /// Constructor.
    pub fn new(
        format: Option<Format>,
        compression: CompressionLevel,
        created_by: Option<String>,
        entry_definitions: Option<String>,
        additional_other_definitions: Vec<String>,
        max_columns: Option<usize>,
        progress_bar: Option<ProgressBar>,
        colorize: bool,
    ) -> Self {
        Self {
            format,
            compression,
            created_by,
            entry_definitions,
            additional_other_definitions,
            max_columns,
            progress_bar,
            colorize,
        }
    }

    /// Move into a meta.
    pub fn into_meta(&mut self) -> Meta {
        let mut meta = Meta::default();

        if self.created_by.is_some() {
            meta.created_by = take(&mut self.created_by);
        }

        if self.entry_definitions.is_some() {
            meta.entry_definitions = take(&mut self.entry_definitions);
        }

        for other_definition in take(&mut self.additional_other_definitions) {
            meta.other_definitions.push(other_definition);
        }

        meta
    }
}

impl Default for CsarCreator {
    fn default() -> Self {
        Self {
            format: Some(Format::GzipTarball),
            compression: CompressionLevel::new_unchecked(7),
            created_by: Some("Puccini".into()),
            entry_definitions: None,
            additional_other_definitions: Default::default(),
            max_columns: Some(80),
            progress_bar: None,
            colorize: false,
        }
    }
}
