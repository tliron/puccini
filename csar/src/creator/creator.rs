use super::{super::tosca_meta::*, compression_level::*, format::*, tracker::*};

use std::mem::*;

//
// CsarCreator
//

/// CSAR creator.
pub struct CsarCreator {
    /// Format.
    pub format: Option<Format>,

    /// Compression level.
    pub compression_level: Option<CompressionLevel>,

    /// Created by.
    pub created_by: Option<String>,

    /// Entry definitions.
    pub entry_definitions: Option<String>,

    /// Additional other definitions.
    pub additional_other_definitions: Vec<String>,

    /// Maximum number of columns for formatting `TOSCA.meta` file.
    pub max_columns: Option<usize>,

    /// Reader tracker.
    pub read_tracker: Option<ReadTrackerRef>,
}

impl CsarCreator {
    /// Constructor.
    pub fn new(
        format: Option<Format>,
        compression_level: Option<CompressionLevel>,
        created_by: Option<String>,
        entry_definitions: Option<String>,
        additional_other_definitions: Vec<String>,
        max_columns: Option<usize>,
        read_tracker: Option<ReadTrackerRef>,
    ) -> Self {
        Self {
            format,
            compression_level,
            created_by,
            entry_definitions,
            additional_other_definitions,
            max_columns,
            read_tracker,
        }
    }

    /// Move relevant fields into a TOSCA meta.
    pub fn into_tosca_meta(&mut self) -> ToscaMeta {
        let mut tosca_meta = ToscaMeta::default();

        if self.created_by.is_some() {
            tosca_meta.created_by = take(&mut self.created_by);
        }

        if self.entry_definitions.is_some() {
            tosca_meta.entry_definitions = take(&mut self.entry_definitions);
        }

        for other_definition in take(&mut self.additional_other_definitions) {
            tosca_meta.other_definitions.push(other_definition);
        }

        tosca_meta
    }
}

impl Default for CsarCreator {
    fn default() -> Self {
        Self {
            format: Some(Format::GzipTarball),
            compression_level: None,
            created_by: Some("Puccini".into()),
            entry_definitions: None,
            additional_other_definitions: Default::default(),
            max_columns: Some(80),
            read_tracker: None,
        }
    }
}
