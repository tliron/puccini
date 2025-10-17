use super::{invalid_key::*, malformed_key::*, tosca_meta::*};

use {depiction::*, std::io, thiserror::*};

//
// CsarError
//

/// CSAR error.
#[derive(Debug, Depict, Error)]
pub enum CsarError {
    /// I/O.
    #[error("I/O: {0}")]
    IO(#[from] io::Error),

    /// Unsupported format.
    #[cfg(feature = "creator")]
    #[error("unsupported format: {0}")]
    UnsupportedFormat(super::super::creator::Format),

    /// Missing.
    #[error("missing: {0}")]
    Missing(String),

    /// Invalid.
    #[error("invalid: {0}")]
    Invalid(String),

    /// TOSCA meta.
    #[error("TOSCA meta: {0}")]
    #[depict(as(depict))]
    ToscaMeta(#[from] ToscaMetaError),

    /// Walk directory.
    #[error("walk directory: {0}")]
    WalkDirectory(#[from] walkdir::Error),

    /// URL.
    #[cfg(feature = "url")]
    #[error("URL: {0}")]
    URL(#[from] read_url::UrlError),
}

impl From<InvalidKeyError> for CsarError {
    fn from(error: InvalidKeyError) -> Self {
        Self::ToscaMeta(error.into())
    }
}

impl From<MalformedKeyError> for CsarError {
    fn from(error: MalformedKeyError) -> Self {
        Self::ToscaMeta(error.into())
    }
}
