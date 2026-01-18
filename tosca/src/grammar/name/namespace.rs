use super::super::name::*;

use {
    depiction::*,
    std::{fmt, io},
};

//
// Namespace
//

/// Namespace.
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Namespace {
    /// Segments.
    pub segments: Vec<Name>,
}

impl Namespace {
    /// True if empty.
    pub fn is_empty(&self) -> bool {
        self.segments.is_empty()
    }

    // /// To Floria directory.
    // pub fn to_floria_directory(&self) -> Result<floria::Directory, floria::MalformedError> {
    //     floria::Directory::new(self.segments.iter().map(|segment| segment.clone().into()).collect())
    // }
}

impl Depict for Namespace {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        for segment in &self.segments {
            context.theme.write_name(writer, segment)?;
            context.theme.write_delimiter(writer, ':')?;
        }
        Ok(())
    }
}

impl fmt::Display for Namespace {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        for segment in &self.segments {
            write!(formatter, "{}:", segment)?;
        }
        Ok(())
    }
}

// Conversions

impl From<Vec<Name>> for Namespace {
    fn from(segments: Vec<Name>) -> Self {
        Self { segments }
    }
}

impl From<Name> for Namespace {
    fn from(name: Name) -> Self {
        Self { segments: vec![name] }
    }
}

impl From<&Name> for Namespace {
    fn from(name: &Name) -> Self {
        Self { segments: vec![name.clone()] }
    }
}
