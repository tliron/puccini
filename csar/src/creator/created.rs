use super::{super::tosca_meta::*, compression_level::*, format::*};

//
// CreatedCsar
//

/// Created CSAR.
#[derive(Clone, Debug)]
pub struct CreatedCsar {
    /// TOSCA meta.
    pub tosca_meta: ToscaMeta,

    /// Format.
    pub format: Format,

    /// Compression level.
    pub compression_level: Option<CompressionLevel>,

    /// Size.
    pub size: Option<u64>,
}

impl CreatedCsar {
    /// Constructor.
    pub fn new(
        tosca_meta: ToscaMeta,
        format: Format,
        compression_level: Option<CompressionLevel>,
        size: Option<u64>,
    ) -> Self {
        Self { tosca_meta, format, compression_level, size }
    }
}
