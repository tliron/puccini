use super::super::dialect::*;

use {
    compris::annotate::*,
    depiction::*,
    std::{fmt, io},
    thiserror::*,
};

//
// UnsupportedDialectError
//

/// Unsupported dialect error.
#[derive(Debug, Error)]
pub struct UnsupportedDialectError<AnnotatedT> {
    /// Dialect ID.
    pub dialect_id: DialectID,

    /// Annotated.
    pub annotated: AnnotatedT,
}

impl_annotated!(UnsupportedDialectError);

impl<AnnotatedT> UnsupportedDialectError<AnnotatedT> {
    /// Constructor.
    pub fn new(dialect_id: DialectID) -> Self
    where
        AnnotatedT: Default,
    {
        Self { dialect_id, annotated: Default::default() }
    }
}

impl<AnnotatedT, NewAnnotatedT> IntoAnnotated<UnsupportedDialectError<NewAnnotatedT>>
    for UnsupportedDialectError<AnnotatedT>
where
    AnnotatedT: Annotated,
    NewAnnotatedT: Annotated + Default,
{
    fn into_annotated(self) -> UnsupportedDialectError<NewAnnotatedT> {
        UnsupportedDialectError::new(self.dialect_id).with_annotations_from(&self.annotated)
    }
}

impl<AnnotatedT> Depict for UnsupportedDialectError<AnnotatedT> {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        write!(writer, "unsupported dialect: {}", context.theme.error(&self.dialect_id))
    }
}

impl<AnnotatedT> fmt::Display for UnsupportedDialectError<AnnotatedT> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.dialect_id, formatter)
    }
}
