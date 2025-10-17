use {
    depiction::*,
    std::{fmt, io},
    thiserror::*,
};

//
// InvalidKeyError
//

/// Invalid key error.
#[derive(Debug, Error)]
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
}

impl fmt::Display for InvalidKeyError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}: {}", self.keyname, self.reason)
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
