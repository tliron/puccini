use super::super::errors::*;

use {
    depiction::*,
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
    pub fn parse(name: &str, representation: &str) -> Result<Self, CsarError> {
        if let Some((major, minor)) = representation.split_once('.') {
            let major = major
                .parse()
                .map_err(|_| MalformedKeyError::new(name.into(), "major not an unsigned integer".into()))?;

            let minor = minor
                .parse()
                .map_err(|_| MalformedKeyError::new(name.into(), "minor not an unsigned integer".into()))?;

            return Ok(Version::new(major, minor));
        }

        Err(MalformedKeyError::new(name.into(), "not <major>.<minor>".into()).into())
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
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}.{}", self.major, self.minor)
    }
}
