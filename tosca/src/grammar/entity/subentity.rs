use super::super::{complete::*, errors::*, name::*};

use compris::annotate::*;

//
// Subentity
//

/// Subentity.
///
/// This trait is only used for *contained* entities. Named entities should implement
/// [Entity](super::entity::Entity) instead.
pub trait Subentity<ParentSubentityT> {
    /// Complete.
    fn complete(
        &mut self,
        name: Option<&Name>,
        parent: Option<&ParentSubentityT>,
        parent_namespace: Option<&Namespace>,
        context: &mut CompletionContext,
    ) -> Result<(), ToscaError<WithAnnotations>>;
}
