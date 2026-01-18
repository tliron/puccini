use {
    compris::annotate::*,
    depiction::*,
    derive_more::*,
    problemo::*,
    std::{fmt, io},
};

//
// UndeclaredError
//

/// Undeclared error.
#[derive(Debug, Error, PartialEq)]
pub struct UndeclaredError {
    /// Type name.
    pub type_name: String,

    /// Name.
    pub name: String,
}

impl UndeclaredError {
    /// Constructor.
    pub fn new<TypeNameT, NameT>(type_name: TypeNameT, name: NameT) -> Self
    where
        TypeNameT: ToString,
        NameT: ToString,
    {
        Self { type_name: type_name.to_string(), name: name.to_string() }
    }

    /// Constructor.
    #[track_caller]
    pub fn as_problem<TypeNameT, NameT>(type_name: TypeNameT, name: NameT) -> Problem
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

impl Depict for UndeclaredError {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        let name = format!("{:?}", self.name);
        write!(writer, "undeclared {}: {}", context.theme.name(&self.type_name), context.theme.error(name))
    }
}

impl fmt::Display for UndeclaredError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}: {}", self.type_name, self.name)
    }
}
