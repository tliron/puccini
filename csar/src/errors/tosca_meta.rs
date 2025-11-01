use super::{super::tosca_meta::*, invalid_key::*, malformed_key::*};

use {kutil::cli::depict::*, std::io, thiserror::*};

//
// ToscaMetaError
//

/// TOSCA.meta error.
#[derive(Debug, Depict, Error)]
pub enum ToscaMetaError {
    /// I/O.
    #[error("I/O: {0}")]
    IO(#[from] io::Error),

    /// Unsupported version.
    #[error("unsupported version: {0}")]
    UnsupportedVersion(Version),

    /// Unsupported keyname.
    #[error("unsupported keyname: {0:?}")]
    UnsupportedKeyname(String),

    /// Required keyname.
    #[error("required keyname: {0:?}")]
    RequiredKeyname(String),

    /// Invalid key.
    #[error("invalid key: {0:?}")]
    #[depict(as(depict))]
    InvalidKey(#[from] InvalidKeyError),

    /// Malformed key.
    #[error("malformed key: {0}")]
    #[depict(as(depict))]
    MalformedKey(#[from] MalformedKeyError),

    /// Malformed.
    #[error("malformed: {0}")]
    Malformed(String),

    /// File not found.
    #[error("file not found: {0}")]
    FileNotFound(String),
}
