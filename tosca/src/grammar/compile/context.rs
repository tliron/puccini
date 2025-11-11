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
    pub store: floria::StoreRef,

    /// Errors.
    pub errors: ToscaErrorReceiverRef<'own>,
}

impl<'own> CompilationContext<'own> {
    /// Constructor.
    pub fn new(
        source_id: &'own SourceID,
        catalog: &'own Catalog,
        directory: &'own floria::Directory,
        store: floria::StoreRef,
        errors: ToscaErrorReceiverRef<'own>,
    ) -> Self {
        Self { source_id, catalog, directory, store, errors }
    }

    /// With source.
    pub fn with_source(&self, source_id: &'own SourceID) -> Self {
        Self {
            source_id,
            catalog: self.catalog,
            directory: self.directory,
            store: self.store.clone(),
            errors: self.errors.clone(),
        }
    }

    /// Get the source.
    pub fn source<AnnotatedT>(&self) -> Result<&Source, SourceNotLoadedError<AnnotatedT>>
    where
        AnnotatedT: Default,
    {
        self.catalog.source(self.source_id)
    }
}
