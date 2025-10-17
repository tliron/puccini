use super::super::super::{catalog::*, errors::*, name::*, source::*};

use {compris::annotate::*, kutil::std::immutable::*};

//
// Subentity
//

/// Subentity.
///
/// This trait is only used for *contained* entities. Named entities should implement
/// [Entity](super::entity::Entity) instead.
pub trait Subentity<ParentT> {
    /// Complete.
    fn complete(
        &mut self,
        name: Option<ByteString>,
        scope: Option<&Scope>,
        parent: Option<&ParentT>,
        catalog: &mut Catalog,
        source_id: &SourceID,
        errors: ToscaErrorRecipientRef,
    ) -> Result<(), ToscaError<WithAnnotations>>;
}
