use std::str::*;

//
// ToscaKind
//

/// TOSCA kind.
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum ToscaKind {
    /// Node.
    Node,

    /// Capability.
    Capability,

    /// Service.
    Service,

    /// Relationship.
    Relationship,
}

impl ToscaKind {
    /// As string.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Node => "node",
            Self::Capability => "capability",
            Self::Service => "service",
            Self::Relationship => "relationship",
        }
    }
}

impl FromStr for ToscaKind {
    type Err = ();

    fn from_str(kind: &str) -> Result<Self, Self::Err> {
        match kind {
            "node" => Ok(Self::Node),
            "capability" => Ok(Self::Capability),
            "service" => Ok(Self::Service),
            "relationship" => Ok(Self::Relationship),
            _ => Err(()),
        }
    }
}
