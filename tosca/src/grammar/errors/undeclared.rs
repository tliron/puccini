use {
    compris::annotate::*,
    depiction::*,
    std::{fmt, io},
    thiserror::*,
};

//
// UndeclaredError
//

/// Undeclared error.
#[derive(Debug, Error)]
pub struct UndeclaredError<AnnotatedT> {
    /// Type name.
    pub type_name: String,

    /// Name.
    pub name: String,

    /// Annotated.
    pub annotated: AnnotatedT,
}

impl_annotated!(UndeclaredError);

impl<AnnotatedT> UndeclaredError<AnnotatedT> {
    /// Constructor.
    pub fn new(type_name: String, name: String) -> Self
    where
        AnnotatedT: Default,
    {
        Self { type_name, name, annotated: Default::default() }
    }
}

impl<AnnotatedT, NewAnnotatedT> IntoAnnotated<UndeclaredError<NewAnnotatedT>> for UndeclaredError<AnnotatedT>
where
    AnnotatedT: Annotated,
    NewAnnotatedT: Annotated + Default,
{
    fn into_annotated(self) -> UndeclaredError<NewAnnotatedT> {
        UndeclaredError::new(self.type_name, self.name).with_annotations_from(&self.annotated)
    }
}

impl<AnnotatedT> Depict for UndeclaredError<AnnotatedT> {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        let name = format!("{:?}", self.name);
        write!(writer, "undeclared {}: {}", context.theme.name(&self.type_name), context.theme.error(name))
    }
}

impl<AnnotatedT> fmt::Display for UndeclaredError<AnnotatedT> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.type_name, self.name)
    }
}
