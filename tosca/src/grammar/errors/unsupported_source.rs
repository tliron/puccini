use super::super::source::*;

use {
    compris::annotate::*,
    depiction::*,
    derive_more::*,
    problemo::*,
    std::{fmt, io},
};

//
// UnsupportedSourceError
//

/// Unsupported source error.
#[derive(Debug, Error, PartialEq)]
pub struct UnsupportedSourceError {
    /// Source ID.
    pub source_id: SourceID,
}

impl UnsupportedSourceError {
    /// Constructor.
    pub fn new(source_id: SourceID) -> Self {
        Self { source_id }
    }

    /// Constructor.
    #[track_caller]
    pub fn as_problem(source_id: SourceID) -> Problem {
        Self::new(source_id)
            .into_problem()
            .with(AnnotatedCauseEquality::new::<Self>())
            .with(ErrorDepiction::new::<Self>())
    }
}

impl Depict for UnsupportedSourceError {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        write!(writer, "unsupported source: {}", context.theme.error(&self.source_id))
    }
}

impl fmt::Display for UnsupportedSourceError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.source_id, formatter)
    }
}
