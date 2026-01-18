use super::super::dialect::*;

use {
    compris::annotate::*,
    depiction::*,
    derive_more::*,
    problemo::*,
    std::{fmt, io},
};

//
// UnsupportedDialectError
//

/// Unsupported dialect error.
#[derive(Debug, Error, PartialEq)]
pub struct UnsupportedDialectError {
    /// Dialect ID.
    pub dialect_id: DialectID,
}

impl UnsupportedDialectError {
    /// Constructor.
    pub fn new(dialect_id: DialectID) -> Self {
        Self { dialect_id }
    }

    /// Constructor.
    #[track_caller]
    pub fn as_problem(dialect_id: DialectID) -> Problem {
        Self::new(dialect_id)
            .into_problem()
            .with(AnnotatedCauseEquality::new::<Self>())
            .with(ErrorDepiction::new::<Self>())
    }
}

impl Depict for UnsupportedDialectError {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        write!(writer, "unsupported dialect: {}", context.theme.error(&self.dialect_id))
    }
}

impl fmt::Display for UnsupportedDialectError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.dialect_id, formatter)
    }
}
