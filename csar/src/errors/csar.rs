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

    /// Meta.
    #[error("meta: {0}")]
    #[depict(as(depict))]
    Meta(#[from] MetaError),
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
