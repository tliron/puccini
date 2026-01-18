use super::super::errors::*;

use {
    depiction::*,
    problemo::*,
    std::{fmt, io},
};

//
// Version
//

/// Version.
#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct Version {
    /// Major.
    pub major: usize,

    /// Minor.
    pub minor: usize,
}

impl Version {
    /// Constructor.
    pub fn new(major: usize, minor: usize) -> Self {
        Version { major, minor }
    }

    /// Parse.
    pub fn parse(name: &str, representation: &str) -> Result<Self, Problem> {
        if let Some((major, minor)) = representation.split_once('.') {
            let major = major.parse().map_err(|_| {
                MalformedKeyError::as_problem(name.into(), "major not an unsigned integer".into()).via(CsarError)
            })?;

            let minor = minor.parse().map_err(|_| {
                MalformedKeyError::as_problem(name.into(), "minor not an unsigned integer".into()).via(CsarError)
            })?;

            return Ok(Version::new(major, minor));
        }

        Err(MalformedKeyError::as_problem(name.into(), "not <major>.<minor>".into()).via(CsarError))
    }
}

impl Depict for Version {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        context.separate(writer)?;
        context.theme.write_number(writer, self.major)?;
        context.theme.write_delimiter(writer, '.')?;
        context.theme.write_number(writer, self.minor)
    }
}

impl fmt::Display for Version {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}.{}", self.major, self.minor)
    }
}
