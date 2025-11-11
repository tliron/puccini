use super::{full_name::*, namespace::*};

use {
    compris::impl_resolve_from_str,
    depiction::*,
    kutil::std::{immutable::*, string::*},
    std::{fmt, io, str::*},
};

/// Invalid name characters.
pub const INVALID_NAME_CHARACTERS: [char; 1] = [NAMESPACE_DELIMITER];

//
// Name
//

/// Name.
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Name(pub ByteString);

impl_resolve_from_str!(Name);

impl Name {
    /// Constructor.
    ///
    /// Note that it does not validate the name!
    pub const fn new_static_unchecked(name: &'static str) -> Self {
        Self(ByteString::from_static(name))
    }

    /// True if empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Convert to a [FullName].
    pub fn into_full_name(self, namespace: Namespace) -> FullName {
        FullName::new(namespace, self)
    }

    /// As [ByteString].
    pub fn as_byte_string(self) -> ByteString {
        self.0
    }
}

impl Depict for Name {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        context.separate(writer)?;
        context.theme.write_name(writer, &self.0)
    }
}

impl FromStr for Name {
    type Err = ParseError;

    fn from_str(name: &str) -> Result<Self, Self::Err> {
        for c in INVALID_NAME_CHARACTERS {
            if name.contains(c) {
                return Err(format!("name contains invalid character: {}", c).into());
            }
        }

        Ok(Name(ByteString::from(name)))
    }
}

// Delegation

impl fmt::Display for Name {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, formatter)
    }
}

// Conversions

impl AsRef<str> for Name {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Into<ByteString> for Name {
    fn into(self) -> ByteString {
        self.0
    }
}

impl From<&'static str> for Name {
    fn from(string: &'static str) -> Self {
        Name::new_static_unchecked(string)
    }
}

impl From<FullName> for Name {
    fn from(full_name: FullName) -> Self {
        full_name.name
    }
}

impl From<&FullName> for Name {
    fn from(full_name: &FullName) -> Self {
        full_name.name.clone()
    }
}
