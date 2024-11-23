use super::super::{errors::*, name::*, source::*};

//
// DerivationPath
//

/// Derivation path.
#[derive(Clone, Debug, Default)]
pub struct DerivationPath(pub Vec<DerivationPathSegment>);

impl DerivationPath {
    /// Constructor.
    pub fn new(source_id: SourceID, name: Name) -> Self {
        Self(vec![DerivationPathSegment::new(source_id, name)])
    }

    /// Add a segment to the path if it's not already there.
    pub fn add<AnnotatedT>(
        &mut self,
        source_id: SourceID,
        name: Name,
    ) -> Result<(), CyclicalDerivationError<AnnotatedT>>
    where
        AnnotatedT: Default,
    {
        tracing::trace!(source = source_id.to_string(), "add segment: {}", name);

        let segment = DerivationPathSegment::new(source_id, name.clone());
        if !self.0.contains(&segment) {
            self.0.push(segment);
            Ok(())
        } else {
            Err(CyclicalDerivationError::new(name.to_string()))
        }
    }

    /// True is path contains the segment.
    pub fn contains(&self, source_id: SourceID, name: Name) -> bool {
        self.0.contains(&DerivationPathSegment::new(source_id, name))
    }
}

//
// DerivationPathSegment
//

/// [DerivationPath] segment.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct DerivationPathSegment {
    /// Source ID.
    pub source_id: SourceID,

    /// Name.
    pub name: Name,
}

impl DerivationPathSegment {
    /// Constructor.
    pub fn new(source_id: SourceID, name: Name) -> Self {
        Self { source_id, name }
    }
}
