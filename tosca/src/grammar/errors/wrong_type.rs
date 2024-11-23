use {
    compris::annotate::*,
    kutil::{cli::depict::*, std::string::*},
    std::{fmt, io},
    thiserror::*,
};

//
// WrongTypeError
//

/// Wrong type error.
#[derive(Debug, Error)]
pub struct WrongTypeError<AnnotatedT> {
    /// Entity.
    pub entity: String,

    /// Type name.
    pub type_name: String,

    /// Expected type names.
    pub expected_type_names: Vec<String>,

    /// Annotated.
    pub annotated: AnnotatedT,
}

impl<AnnotatedT> WrongTypeError<AnnotatedT> {
    /// Constructor.
    pub fn new(entity: String, type_name: String, expected_type_names: Vec<String>) -> Self
    where
        AnnotatedT: Default,
    {
        Self { entity, type_name, expected_type_names, annotated: Default::default() }
    }

    /// Into different [Annotated] implementation.
    pub fn into_annotated<NewAnnotationsT>(self) -> WrongTypeError<NewAnnotationsT>
    where
        AnnotatedT: Annotated,
        NewAnnotationsT: Annotated + Default,
    {
        WrongTypeError::new(self.entity, self.type_name, self.expected_type_names)
            .with_annotations_from(&self.annotated)
    }
}

impl_dyn_annotated_error!(WrongTypeError);

impl<AnnotatedT> Depict for WrongTypeError<AnnotatedT> {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        let type_name = format!("{:?}", self.type_name);
        if self.expected_type_names.is_empty() {
            write!(writer, "{} has wrong type: {}", self.entity, context.theme.error(type_name))
        } else {
            write!(
                writer,
                "{} has wrong type: is {}, expected {}",
                self.entity,
                context.theme.error(&self.type_name),
                self.expected_type_names.join_conjunction("or")
            )
        }
    }
}

impl<AnnotatedT> fmt::Display for WrongTypeError<AnnotatedT> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.expected_type_names.is_empty() {
            write!(formatter, "{} is {}", self.entity, self.type_name,)
        } else {
            write!(
                formatter,
                "{} is {}, expected {}",
                self.entity,
                self.type_name,
                self.expected_type_names.join_conjunction("or")
            )
        }
    }
}
