use super::{
    super::{entity::*, errors::*, source::*},
    catalog::*,
};

use problemo::*;

impl Catalog {
    /// Add a source.
    pub fn add_source(&mut self, source: Source) {
        self.sources.insert(source.source_id.clone(), source);
    }

    /// Add sources.
    pub fn add_sources<IterableT>(&mut self, sources: IterableT)
    where
        IterableT: IntoIterator<Item = Source>,
    {
        for source in sources {
            self.add_source(source);
        }
    }

    /// Get a source.
    pub fn source(&self, source_id: &SourceID) -> Result<&Source, Problem> {
        self.sources.get(source_id).ok_or_else(|| SourceNotLoadedError::as_problem(source_id.clone()))
    }

    /// Get a source.
    pub fn source_mut(&mut self, source_id: &SourceID) -> Result<&mut Source, Problem> {
        self.sources.get_mut(source_id).ok_or_else(|| SourceNotLoadedError::as_problem(source_id.clone()))
    }

    /// Supported entity kinds.
    pub fn source_entity_kinds(&self, source_id: &SourceID) -> Result<&EntityKinds, Problem> {
        self.dialect_entity_kinds(&self.source(source_id)?.dialect_id)
    }
}
