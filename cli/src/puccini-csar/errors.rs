use {compris::ser::*, kutil::cli::run::*, puccini_csar::*, read_url::*, std::io, thiserror::*};

//
// MainError
//

/// Main error.
#[derive(Debug, Error)]
pub enum MainError {
    #[error("{0}")]
    Exit(#[from] ExitError),

    #[error("I/O: {0}")]
    IO(#[from] io::Error),

    #[error("CSAR: {0}")]
    CSAR(#[from] CsarError),

    #[error("URL: {0}")]
    URL(#[from] UrlError),

    #[error("serialize: {0}")]
    Serialize(#[from] SerializeError),
}

handle_exit_error!(MainError, Exit);
