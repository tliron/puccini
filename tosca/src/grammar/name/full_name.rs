use super::{name::*, namespace::*, to_namespace::*};

use {
    compris::impl_resolve_from_str,
    depiction::*,
    kutil::std::string::*,
    std::{fmt, io, str::*},
};

/// Namespace delimiter.
pub const NAMESPACE_DELIMITER: &str = ":";

//
// FullName
//

/// [Name] with [Namespace].
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FullName {
    /// Namespace.
    pub namespace: Namespace,

    /// Name.
    pub name: Name,
}

impl_resolve_from_str!(FullName);

impl FullName {
    /// Constructor.
    pub fn new(namespace: Namespace, name: Name) -> Self {
        Self { namespace, name }
    }

    /// True if name is empty.
    pub fn is_empty(&self) -> bool {
        self.name.is_empty()
    }

    /// Put inside a [Namespace].
    pub fn into_namespace(self, mut namespace: Namespace) -> Self {
        namespace.segments.extend(self.namespace.segments);
        Self::new(namespace, self.name)
    }

    /// To Floria class ID.
    pub fn to_floria_class_id(&self, prefix: &str) -> floria::ID {
        let (mut directory, name) = (self.namespace.to_floria_directory(), self.name.clone());
        directory.add_first_segment(prefix.into());
        directory.add_first_segment("tosca".into());
        floria::ID::new_for(floria::EntityKind::Class, directory, name.into())
    }
}

impl ToNamespace<FullName> for FullName {
    fn to_namespace(&self, namespace: Option<&Namespace>) -> Self {
        let clone = self.clone();
        match namespace {
            Some(namespace) => clone.into_namespace(namespace.clone()),
            None => clone,
        }
    }
}

impl Depict for FullName {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        context.separate(writer)?;
        self.namespace.depict(writer, context)?;
        self.name.depict(writer, &context.clone().with_separator(false))
    }
}

impl fmt::Display for FullName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}{}", self.namespace, self.name)
    }
}

impl FromStr for FullName {
    type Err = ParseError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let segments: Vec<&str> = string.split(NAMESPACE_DELIMITER).collect();
        let length = segments.len();

        Ok(if length > 0 {
            let mut namespace = Vec::with_capacity(length - 1);
            for segment in &segments[..length - 1] {
                namespace.push(segment.parse()?);
            }

            Self::new(Namespace::from(namespace), segments[length - 1].parse()?)
        } else {
            Self::new(Default::default(), string.parse()?)
        })
    }
}

impl From<Name> for FullName {
    fn from(name: Name) -> Self {
        Self::new(Default::default(), name)
    }
}

impl From<&Name> for FullName {
    fn from(name: &Name) -> Self {
        Self::new(Default::default(), name.clone())
    }
}
