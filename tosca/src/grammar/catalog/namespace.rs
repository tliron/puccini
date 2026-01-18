use super::{
    super::{entity::*, name::*, source::*},
    catalog::*,
};

use problemo::*;

impl Catalog {
    /// All entity names.
    pub fn entity_names(&self) -> impl Iterator<Item = (&SourceID, &EntityKind, &Name)> {
        self.sources.iter().flat_map(|(source_id, source)| {
            source.entity_names().map(move |(entity_kind, name)| (source_id, entity_kind, name))
        })
    }

    /// Find the source ID of a full name.
    pub fn try_lookup<'context>(
        &'context self,
        entity_kind: EntityKind,
        full_name: &FullName,
        source_id: &'context SourceID,
    ) -> Option<&'context SourceID> {
        self.sources.get(source_id)?.try_lookup(entity_kind, full_name)
    }

    /// Find the source of an entity.
    pub fn lookup(
        &self,
        entity_kind: EntityKind,
        entity_kind_name: &str,
        source_id: &SourceID,
        full_name: &FullName,
    ) -> Result<&Source, Problem> {
        let source = self.source(source_id)?;
        let entity_source_id = source.lookup(entity_kind, entity_kind_name, full_name)?;
        Ok(self.source(entity_source_id)?)
    }

    /// Find the source of an entity.
    pub fn lookup_mut(
        &mut self,
        entity_kind: EntityKind,
        entity_kind_name: &str,
        source_id: &SourceID,
        full_name: &FullName,
    ) -> Result<&mut Source, Problem> {
        let source = self.source(source_id)?;
        let entity_source_id = source.lookup(entity_kind, entity_kind_name, full_name)?.clone();
        Ok(self.source_mut(&entity_source_id)?)
    }
}
