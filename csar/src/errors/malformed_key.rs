use {
    depiction::*,
    std::{fmt, io},
    thiserror::*,
};

//
// MalformedKeyError
//

/// Malformed key error.
#[derive(Debug, Error)]
pub struct MalformedKeyError {
    /// Keyname.
    pub keyname: String,

    /// Reason.
    pub reason: String,
}

impl MalformedKeyError {
    /// Constructor.
    pub fn new(keyname: String, reason: String) -> Self {
        Self { keyname, reason: reason }
    }
}

impl fmt::Display for MalformedKeyError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}: {}", self.keyname, self.reason)
    }
}

impl Depict for MalformedKeyError {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        context.separate(writer)?;
        write!(writer, "{} malformed: {}", context.theme.meta(&self.keyname), context.theme.error(&self.reason))
    }
}
