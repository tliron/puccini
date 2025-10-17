use {
    compris::annotate::*,
    depiction::*,
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

impl_annotated!(OverrideProhibitedError);

impl<AnnotatedT> OverrideProhibitedError<AnnotatedT>
where
    AnnotatedT: Default,
{
    /// Constructor.
    pub fn new(name: String) -> Self {
        Self { name, annotated: Default::default() }
    }
}

impl<AnnotatedT, NewAnnotatedT> IntoAnnotated<OverrideProhibitedError<NewAnnotatedT>>
    for OverrideProhibitedError<AnnotatedT>
where
    AnnotatedT: Annotated,
    NewAnnotatedT: Annotated + Default,
{
    fn into_annotated(self) -> OverrideProhibitedError<NewAnnotatedT> {
        OverrideProhibitedError::new(self.name).with_annotations_from(&self.annotated)
    }
}

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
