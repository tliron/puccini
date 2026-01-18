use super::super::complete::*;

use {compris::annotate::*, depiction::*, problemo::*, std::any::*};

//
// Entity
//

/// Entity.
///
/// This trait is only used for *named* entities. Contained entities should implement
/// [Subentity](super::subentity::Subentity) instead.
pub trait Entity
where
    Self: Any + DynAnnotated + DynDepict,
{
    /// Completion state.
    fn completion_state(&self) -> CompletionState;

    /// Whether the entity is complete.
    fn is_complete(&self) -> bool {
        self.completion_state() == CompletionState::Complete
    }

    /// Whether the entity should be completed.
    fn should_complete(&self) -> bool {
        self.completion_state() == CompletionState::Incomplete
    }

    /// Complete.
    ///
    /// Note that we cannot allow the annotated type to be generic because this trait must be
    /// `dyn`-compatible.
    ///
    /// If you need a different annotated type for the errors you can use
    /// [IntoAnnotated::into_annotated].
    fn complete(
        &mut self,
        derivation_path: &mut DerivationPath,
        context: &mut CompletionContext,
    ) -> Result<(), Problem>;
}
