use {
    compris::annotate::*,
    kutil::cli::depict::*,
    std::{fmt, io},
    thiserror::*,
};

//
// NumberOverflowError
//

/// Number overflow error.
#[derive(Debug, Error)]
pub struct NumberOverflowError<AnnotatedT> {
    /// Annotated.
    pub annotated: AnnotatedT,
}

impl_annotated!(NumberOverflowError);

impl<AnnotatedT> NumberOverflowError<AnnotatedT> {
    /// Constructor.
    pub fn new() -> Self
    where
        AnnotatedT: Default,
    {
        Self { annotated: Default::default() }
    }
}

impl<AnnotatedT, NewAnnotatedT> IntoAnnotated<NumberOverflowError<NewAnnotatedT>> for NumberOverflowError<AnnotatedT>
where
    AnnotatedT: Annotated,
    NewAnnotatedT: Annotated + Default,
{
    fn into_annotated(self) -> NumberOverflowError<NewAnnotatedT> {
        NumberOverflowError::new().with_annotations_from(&self.annotated)
    }
}

impl<AnnotatedT> Depict for NumberOverflowError<AnnotatedT> {
    fn depict<WriteT>(&self, writer: &mut WriteT, _context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        write!(writer, "number overflow")
    }
}

impl<AnnotatedT> fmt::Display for NumberOverflowError<AnnotatedT> {
    fn fmt(&self, _formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(())
    }
}
