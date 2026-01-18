use {
    compris::annotate::*,
    depiction::*,
    derive_more::*,
    problemo::*,
    std::{fmt, io},
};

//
// UnknownTypeError
//

/// Unknown type error.
#[derive(Debug, Error, PartialEq)]
pub struct UnknownTypeError {
    /// Type name.
    pub type_name: String,

    /// Context.
    pub context: String,
}

impl UnknownTypeError {
    /// Constructor.
    pub fn new<TypeNameT, ContextT>(type_name: TypeNameT, context: ContextT) -> Self
    where
        TypeNameT: ToString,
        ContextT: ToString,
    {
        Self { type_name: type_name.to_string(), context: context.to_string() }
    }

    /// Constructor.
    #[track_caller]
    pub fn as_problem<TypeNameT, ContextT>(type_name: TypeNameT, context: ContextT) -> Problem
    where
        TypeNameT: ToString,
        ContextT: ToString,
    {
        Self::new(type_name, context)
            .into_problem()
            .with(AnnotatedCauseEquality::new::<Self>())
            .with(ErrorDepiction::new::<Self>())
    }
}

impl Depict for UnknownTypeError {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        let type_name = format!("{:?}", self.type_name);
        write!(writer, "unknown type for {}: {}", self.context, context.theme.error(type_name))
    }
}

impl fmt::Display for UnknownTypeError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}, {}", self.type_name, self.context)
    }
}
