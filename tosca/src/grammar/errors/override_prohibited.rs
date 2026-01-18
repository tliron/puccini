use {
    compris::annotate::*,
    depiction::*,
    derive_more::*,
    problemo::*,
    std::{fmt, io},
};

//
// OverrideProhibitedError
//

/// Override prohibited error.
#[derive(Debug, Error, PartialEq)]
pub struct OverrideProhibitedError {
    /// Name.
    pub name: String,
}

impl OverrideProhibitedError {
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

impl Depict for OverrideProhibitedError {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        write!(writer, "override prohibited here: {}", context.theme.error(&self.name))
    }
}

impl fmt::Display for OverrideProhibitedError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.name, formatter)
    }
}
