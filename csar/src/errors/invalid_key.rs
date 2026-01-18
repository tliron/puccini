use {
    compris::annotate::*,
    depiction::*,
    derive_more::*,
    problemo::*,
    std::{fmt, io},
};

//
// InvalidKeyError
//

/// Invalid key error.
#[derive(Debug, Error, PartialEq)]
pub struct InvalidKeyError {
    /// Keyname.
    pub keyname: String,

    /// Reason.
    pub reason: String,
}

impl InvalidKeyError {
    /// Constructor.
    pub fn new(keyname: String, reason: String) -> Self {
        Self { keyname, reason: reason }
    }

    /// Constructor.
    #[track_caller]
    pub fn as_problem(keyname: String, reason: String) -> Problem {
        Self::new(keyname, reason)
            .into_problem()
            .with(AnnotatedCauseEquality::new::<Self>())
            .with(ErrorDepiction::new::<Self>())
    }
}

impl Depict for InvalidKeyError {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        context.separate(writer)?;
        write!(writer, "{} invalid: {}", context.theme.meta(&self.keyname), context.theme.error(&self.reason))
    }
}

impl fmt::Display for InvalidKeyError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}: {}", self.keyname, self.reason)
    }
}
