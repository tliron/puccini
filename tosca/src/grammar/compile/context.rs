use super::super::{catalog::*, source::*};

use problemo::*;

//
// CompilationContext
//

/// Compilation context.
pub struct CompilationContext<'context> {
    /// Source ID.
    pub source_id: &'context SourceID,

    /// Catalog.
    pub catalog: &'context Catalog,

    /// Floria directory.
    pub directory: &'context floria::Directory,

    /// Floria store.
    pub store: floria::StoreRef,

    /// Problems.
    pub problems: ProblemReceiverRef<'context>,
}

impl<'context> CompilationContext<'context> {
    /// Constructor.
    pub fn new(
        source_id: &'context SourceID,
        catalog: &'context Catalog,
        directory: &'context floria::Directory,
        store: floria::StoreRef,
        problems: ProblemReceiverRef<'context>,
    ) -> Self {
        Self { source_id, catalog, directory, store, problems }
    }

    /// With source.
    pub fn with_source(&self, source_id: &'context SourceID) -> Self {
        Self {
            source_id,
            catalog: self.catalog,
            directory: self.directory,
            store: self.store.clone(),
            problems: self.problems.clone(),
        }
    }

    /// Get the source.
    pub fn source(&self) -> Result<&Source, Problem> {
        self.catalog.source(self.source_id)
    }
}
