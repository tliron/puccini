use {
    compris::annotate::*,
    depiction::*,
    derive_more::*,
    problemo::*,
    std::{fmt, io},
};

//
// CyclicalDerivationError
//

/// Cyclical derivation error.
#[derive(Debug, Error, PartialEq)]
pub struct CyclicalDerivationError {
    /// Parent name.
    pub parent_name: String,
}

impl CyclicalDerivationError {
    /// Constructor.
    pub fn new<ParentNameT>(parent_name: ParentNameT) -> Self
    where
        ParentNameT: ToString,
    {
        Self { parent_name: parent_name.to_string() }
    }

    /// Constructor.
    #[track_caller]
    pub fn as_problem<ParentNameT>(parent_name: ParentNameT) -> Problem
    where
        ParentNameT: ToString,
    {
        Self::new(parent_name)
            .into_problem()
            .with(AnnotatedCauseEquality::new::<Self>())
            .with(ErrorDepiction::new::<Self>())
    }
}

impl Depict for CyclicalDerivationError {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        let parent_name = format!("{:?}", self.parent_name);
        write!(writer, "cyclical derivation: {}", context.theme.error(parent_name))
    }
}

impl fmt::Display for CyclicalDerivationError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.parent_name, formatter)
    }
}
