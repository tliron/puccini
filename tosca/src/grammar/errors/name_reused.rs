use {
    compris::annotate::*,
    depiction::*,
    std::{fmt, io},
    thiserror::*,
};

//
// NameReusedError
//

/// Name reused error.
#[derive(Debug, Error)]
pub struct NameReusedError<AnnotatedT> {
    /// Name.
    pub name: String,

    /// Annotated.
    pub annotated: AnnotatedT,
}

impl_annotated!(NameReusedError);

impl<AnnotatedT> NameReusedError<AnnotatedT> {
    /// Constructor.
    pub fn new(name: String) -> Self
    where
        AnnotatedT: Default,
    {
        Self { name, annotated: Default::default() }
    }
}

impl<AnnotatedT, NewAnnotatedT> IntoAnnotated<NameReusedError<NewAnnotatedT>> for NameReusedError<AnnotatedT>
where
    AnnotatedT: Annotated,
    NewAnnotatedT: Annotated + Default,
{
    fn into_annotated(self) -> NameReusedError<NewAnnotatedT> {
        NameReusedError::new(self.name).with_annotations_from(&self.annotated)
    }
}

impl<AnnotatedT> Depict for NameReusedError<AnnotatedT> {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        let name = format!("{:?}", self.name);
        write!(writer, "name reused: {}", context.theme.error(name))
    }
}

impl<AnnotatedT> fmt::Display for NameReusedError<AnnotatedT> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.name, formatter)
    }
}
