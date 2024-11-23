use std::str::*;

//
// ToscaKind
//

/// TOSCA kind.
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum ToscaKind {
    // Vertexes
    Node,
    Capability,
    Service,

    // Edges
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
            "NodeTemplate" => Ok(Self::Node),
            "Capability" => Ok(Self::Capability),
            "ServiceTemplate" => Ok(Self::Service),
            "RelationshipTemplate" => Ok(Self::Relationship),
            _ => Err(()),
        }
    }
}
