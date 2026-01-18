use {kutil::std::immutable::*, std::fmt, uuid::*};

//
// SourceID
//

/// Source ID.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SourceID {
    /// URL.
    URL(ByteString),

    /// Profile.
    Profile(ByteString),

    /// Universally unique ID.
    ID(Uuid),

    /// Internal.
    Internal(ByteString),
}

impl SourceID {
    /// Constructor.
    pub fn url_or_default(url: Option<ByteString>) -> Self {
        match url {
            Some(url) => Self::URL(url),
            None => Default::default(),
        }
    }

    /// True if internal.
    pub fn is_internal(&self) -> bool {
        matches!(self, SourceID::Internal(_))
    }
}

impl Default for SourceID {
    fn default() -> Self {
        Self::ID(Uuid::new_v4())
    }
}

impl fmt::Display for SourceID {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::URL(url) => write!(formatter, "url:{}", url),
            Self::Profile(profile) => write!(formatter, "profile:{}", profile),
            Self::ID(uuid) => write!(formatter, "id:{}", uuid),
            Self::Internal(internal) => write!(formatter, "internal:{}", internal),
        }
    }
}

impl Into<ByteString> for SourceID {
    fn into(self) -> ByteString {
        match self {
            Self::URL(url) => url,
            Self::Profile(profile) => profile,
            Self::ID(id) => id.to_string().into(),
            Self::Internal(internal) => internal,
        }
    }
}

impl Into<ByteString> for &SourceID {
    fn into(self) -> ByteString {
        match self {
            SourceID::URL(url) => url.clone(),
            SourceID::Profile(profile) => profile.clone(),
            SourceID::ID(id) => id.to_string().into(),
            SourceID::Internal(internal) => internal.clone(),
        }
    }
}
