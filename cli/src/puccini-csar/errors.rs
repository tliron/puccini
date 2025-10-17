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

    #[error("walk directory: {0}")]
    WalkDirectory(#[from] walkdir::Error),
}

impl RunError for MainError {
    fn handle(&self) -> (bool, u8) {
        (
            false,
            match self {
                MainError::Exit(exit) => exit.code,
                _ => 1,
            },
        )
    }
}
