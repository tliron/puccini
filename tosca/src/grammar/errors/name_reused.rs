use {
    compris::annotate::*,
    depiction::*,
    derive_more::*,
    problemo::*,
    std::{fmt, io},
};

//
// NameReusedError
//

/// Name reused error.
#[derive(Debug, Error, PartialEq)]
pub struct NameReusedError {
    /// Name.
    pub name: String,
}

impl NameReusedError {
    /// Constructor.
    pub fn new<NameT>(name: NameT) -> Self
    where
        NameT: ToString,
    {
        Self { name: name.to_string() }
    }

    /// Constructor.
    #[track_caller]
    pub fn as_problem<NameT>(name: NameT) -> Problem
    where
        NameT: ToString,
    {
        Self::new(name).into_problem().with(AnnotatedCauseEquality::new::<Self>()).with(ErrorDepiction::new::<Self>())
    }
}

impl Depict for NameReusedError {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        let name = format!("{:?}", self.name);
        write!(writer, "name reused: {}", context.theme.error(name))
    }
}

impl fmt::Display for NameReusedError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.name, formatter)
    }
}
