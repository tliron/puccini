use std::str::*;

//
// ToscaKind
//

/// TOSCA kind.
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum ToscaKind {
    /// Service.
    Service,

    /// Node.
    Node,

    /// Capability.
    Capability,

    /// Interface.
    Interface,

    /// Relationship.
    Relationship,

    /// Parameter.
    Parameter,
}

impl ToscaKind {
    /// As string.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Service => "service",
            Self::Node => "node",
            Self::Capability => "capability",
            Self::Interface => "interface",
            Self::Relationship => "relationship",
            Self::Parameter => "parameter",
        }
    }
}

impl FromStr for ToscaKind {
    type Err = ();

    fn from_str(kind: &str) -> Result<Self, Self::Err> {
        match kind {
            "service" => Ok(Self::Service),
            "node" => Ok(Self::Node),
            "capability" => Ok(Self::Capability),
            "interface" => Ok(Self::Interface),
            "relationship" => Ok(Self::Relationship),
            "parameter" => Ok(Self::Parameter),
            _ => Err(()),
        }
    }
}
