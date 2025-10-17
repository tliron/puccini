use super::super::{catalog::*, errors::*, source::*};

//
// CompilationContext
//

/// Compilation context.
pub struct CompilationContext<'own> {
    /// Source ID.
    pub source_id: &'own SourceID,

    /// Catalog.
    pub catalog: &'own Catalog,

    /// Floria directory.
    pub directory: &'own floria::Directory,

    /// Floria store.
    pub store: floria::StoreRef<'own>,

    /// Errors.
    pub errors: ToscaErrorReceiverRef<'own>,
}

impl<'own> CompilationContext<'own> {
    /// Constructor.
    pub fn new(
        source_id: &'own SourceID,
        catalog: &'own Catalog,
        directory: &'own floria::Directory,
        store: floria::StoreRef<'own>,
        errors: ToscaErrorReceiverRef<'own>,
    ) -> Self {
        Self { source_id, catalog, directory, store, errors }
    }
}
