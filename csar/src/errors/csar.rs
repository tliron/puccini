use super::{invalid_key::*, malformed_key::*, meta::*};

use {kutil::cli::depict::*, std::io, thiserror::*};

//
// CsarError
//

/// CSAR error.
#[derive(Debug, Depict, Error)]
pub enum CsarError {
    /// I/O.
    #[error("I/O: {0}")]
    IO(#[from] io::Error),

    /// Missing.
    #[error("missing: {0}")]
    Missing(String),

    /// Invalid.
    #[error("invalid: {0}")]
    Invalid(String),

    /// Meta.
    #[error("meta: {0}")]
    #[depict(as(depict))]
    Meta(#[from] MetaError),

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
        Self::Meta(error.into())
    }
}

impl From<MalformedKeyError> for CsarError {
    fn from(error: MalformedKeyError) -> Self {
        Self::Meta(error.into())
    }
}
