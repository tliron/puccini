use {compris::ser::*, floria::*, kutil::cli::run::*, puccini_tosca::grammar::*, read_url::*, std::io, thiserror::*};

//
// MainError
//

/// Main error.
#[derive(Debug, Error)]
pub enum MainError {
    /// Exit.
    #[error("{0}")]
    Exit(#[from] ExitError),

    #[error("I/O: {0}")]
    IO(#[from] io::Error),

    #[error("puccini: {0}")]
    Puccini(String),

    #[error("imperative: {0}")]
    Floria(#[from] FloriaError),

    #[cfg(feature = "plugins")]
    #[error("plugin: {0}")]
    Plugin(#[from] floria::plugins::PluginError),

    #[error("URL: {0}")]
    URL(#[from] UrlError),

    #[error("store: {0}")]
    Store(#[from] StoreError),

    #[error("serialize: {0}")]
    Serialize(#[from] SerializeError),
}

// impl<AnnotatedT> From<PucciniError<AnnotatedT>> for MainError {
//     fn from(error: PucciniError<AnnotatedT>) -> Self {
//         Self::Puccini(error.to_string())
//     }
// }

impl<AnnotatedT> From<ToscaError<AnnotatedT>> for MainError {
    fn from(error: ToscaError<AnnotatedT>) -> Self {
        Self::Puccini(error.to_string())
    }
}

handle_exit_error!(MainError, Exit);
