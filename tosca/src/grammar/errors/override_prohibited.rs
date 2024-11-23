use {
    compris::annotate::*,
    kutil::cli::depict::*,
    std::{fmt, io},
    thiserror::*,
};

//
// OverrideProhibitedError
//

/// Override prohibited error.
#[derive(Debug, Error)]
pub struct OverrideProhibitedError<AnnotatedT> {
    /// Name.
    pub name: String,

    /// Annotated.
    pub annotated: AnnotatedT,
}

impl<AnnotatedT> OverrideProhibitedError<AnnotatedT>
where
    AnnotatedT: Default,
{
    /// Constructor.
    pub fn new(name: String) -> Self {
        Self { name, annotated: Default::default() }
    }

    /// Into different [Annotated] implementation.
    pub fn into_annotated<NewAnnotationsT>(self) -> OverrideProhibitedError<NewAnnotationsT>
    where
        AnnotatedT: Annotated,
        NewAnnotationsT: Annotated + Default,
    {
        OverrideProhibitedError::new(self.name).with_annotations_from(&self.annotated)
    }
}

impl_dyn_annotated_error!(OverrideProhibitedError);

impl<AnnotatedT> Depict for OverrideProhibitedError<AnnotatedT> {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        write!(writer, "override prohibited here: {}", context.theme.error(&self.name))
    }
}

impl<AnnotatedT> fmt::Display for OverrideProhibitedError<AnnotatedT> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.name, formatter)
    }
}
