use {
    compris::annotate::*,
    depiction::*,
    derive_more::*,
    problemo::*,
    std::{fmt, io},
};

//
// MissingRequiredError
//

/// Missing required error.
#[derive(Debug, Error, PartialEq)]
pub struct MissingRequiredError {
    /// Type name.
    pub type_name: String,

    /// Name.
    pub name: Option<String>,
}

impl MissingRequiredError {
    /// Constructor.
    pub fn new<TypeNameT, NameT>(type_name: TypeNameT, name: Option<NameT>) -> Self
    where
        TypeNameT: ToString,
        NameT: ToString,
    {
        Self { type_name: type_name.to_string(), name: name.map(|name| name.to_string()) }
    }

    /// Constructor.
    #[track_caller]
    pub fn as_problem<TypeNameT, NameT>(type_name: TypeNameT, name: Option<NameT>) -> Problem
    where
        TypeNameT: ToString,
        NameT: ToString,
    {
        Self::new(type_name, name)
            .into_problem()
            .with(AnnotatedCauseEquality::new::<Self>())
            .with(ErrorDepiction::new::<Self>())
    }
}

impl Depict for MissingRequiredError {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        match &self.name {
            Some(name) => {
                let name = format!("{:?}", name);
                write!(writer, "missing required {}: {}", self.type_name, context.theme.error(name))
            }

            None => write!(writer, "missing required {}", self.type_name),
        }
    }
}

impl fmt::Display for MissingRequiredError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match &self.name {
            Some(name) => write!(formatter, "{}: {}", self.type_name, name),
            None => write!(formatter, "{}", self.type_name),
        }
    }
}
