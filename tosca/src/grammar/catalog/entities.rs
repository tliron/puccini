use super::{
    super::{complete::*, entity::*, errors::*, name::*, source::*},
    catalog::*,
};

use problemo::*;

impl Catalog {
    /// Find an entity by its reference.
    pub fn find(&self, entity_ref: &EntityRef) -> Option<(EntityKind, Name, &Source)> {
        for source in self.sources.values() {
            if let Some((entity_kind, name)) = source.find(entity_ref) {
                return Some((entity_kind, name, source));
            }
        }
        None
    }

    /// Add an entity reference.
    pub fn add_entity_ref(
        &mut self,
        entity_kind: EntityKind,
        source_id: &SourceID,
        name: Name,
        entity: EntityRef,
    ) -> Result<(), Problem> {
        Ok(self.source_mut(source_id)?.add_entity_ref(entity_kind, name, entity)?)
    }

    /// Get an entity reference.
    ///
    /// If not found (e.g. it is currently removed for its completion phase) will return the
    /// fallback if it exists.
    pub fn entity_ref(
        &self,
        entity_kind: EntityKind,
        full_name: &FullName,
        source_id: &SourceID,
    ) -> Result<(&EntityRef, &Source), Problem> {
        let entity_kind_name = self.source_entity_kinds(source_id)?.represent(entity_kind);
        let source = self.lookup(entity_kind, &entity_kind_name, source_id, full_name)?;
        Ok((source.entity_ref(entity_kind, &entity_kind_name, &full_name.name)?, source))
    }

    /// Remove an entity and return its entity reference and source ID if it exists.
    pub fn remove_entity_ref(
        &mut self,
        entity_kind: EntityKind,
        full_name: &FullName,
        source_id: &SourceID,
    ) -> Result<(EntityRef, &Source), Problem> {
        let entity_kind_name = self.source_entity_kinds(source_id)?.represent(entity_kind);
        let source = self.lookup_mut(entity_kind, &entity_kind_name, source_id, full_name)?;
        let entity = source.remove_entity_ref(entity_kind, &entity_kind_name, &full_name.name)?;
        Ok((entity, source))
    }

    /// Get an entity reference,
    /// calling [complete](Entity::complete) on it if
    /// [should_complete](Entity::should_complete) is true.
    ///
    /// Note that the entity is removed from the catalog while it is being completed.
    ///
    /// If not found (e.g. it is currently removed for its completion phase) will return the
    /// fallback if it exists.
    pub fn completed_entity_ref<ProblemReceiverT>(
        &mut self,
        entity_kind: EntityKind,
        full_name: &FullName,
        source_id: &SourceID,
        derivation_path: &mut DerivationPath,
        problems: &mut ProblemReceiverT,
    ) -> Result<Option<(&EntityRef, &Source)>, Problem>
    where
        ProblemReceiverT: ProblemReceiver,
    {
        if full_name.is_empty() {
            return Ok(None);
        }

        let source = give_unwrap!(self.source(source_id), problems);

        let entity_kind_name =
            give_unwrap!(self.dialect_entity_kinds(&source.dialect_id), problems).represent(entity_kind);

        let entity_source_id = give_unwrap!(source.lookup(entity_kind, &entity_kind_name, full_name), problems).clone();

        let entity_source = give_unwrap!(self.source_mut(&entity_source_id), problems);

        // Fallback?

        if let Some(_) =
            give_unwrap!(entity_source.fallback_entity_ref(entity_kind, &entity_kind_name, &full_name.name), problems)
        {
            return self.entity_ref(entity_kind, full_name, source_id).map(Some);
        }

        // Remove entity

        let mut entity =
            give_unwrap!(entity_source.remove_entity_ref(entity_kind, &entity_kind_name, &full_name.name), problems);

        // Update derivation path

        give_unwrap!(derivation_path.add(entity_source_id.clone(), full_name.name.clone()), problems);

        // Complete

        let complete = {
            match self.complete_entity(
                &mut entity,
                &entity_kind_name,
                &full_name.name,
                &entity_source_id,
                derivation_path,
                problems,
            ) {
                Ok(complete) => complete,
                Err(error) => {
                    if self.add_entity_ref(entity_kind, &entity_source_id, full_name.name.clone(), entity).is_err() {
                        panic!("source {} disappeared", &entity_source_id);
                    }
                    return Err(error);
                }
            }
        };

        // Add entity back

        if self.add_entity_ref(entity_kind, &entity_source_id, full_name.name.clone(), entity).is_err() {
            panic!("source {} disappeared", &entity_source_id);
        }

        Ok(if complete {
            // Get entity

            match self.entity_ref(entity_kind, full_name, source_id) {
                Ok(entity) => Some(entity),

                Err(_) => {
                    panic!("{} {} disappeared from {}", entity_kind_name, full_name, entity_source_id);
                }
            }
        } else {
            None
        })
    }

    /// Get an [Entity].
    ///
    /// If not found (e.g. it is currently removed for its completion phase) will return the
    /// fallback if it exists.
    pub fn entity<EntityT>(
        &self,
        entity_kind: EntityKind,
        full_name: &FullName,
        source_id: &SourceID,
    ) -> Result<(&EntityT, &Source), Problem>
    where
        EntityT: 'static,
    {
        let (entity, source) = self.entity_ref(entity_kind, full_name, source_id)?;
        Ok((entity.downcast_ref_checked()?, source))
    }

    /// Get an [Entity],
    /// calling [complete](Entity::complete) on it if
    /// [should_complete](Entity::should_complete) is true.
    ///
    /// If not found (e.g. it is currently removed for its completion phase) will return the
    /// fallback if it exists.
    ///
    /// A [DerivationPath] is created in order to detect circular dependencies.
    ///
    /// Note that the entity is removed from the catalog while it is being completed.
    pub fn completed_entity<EntityT, ProblemReceiverT>(
        &mut self,
        entity_kind: EntityKind,
        full_name: &FullName,
        source_id: &SourceID,
        problems: &mut ProblemReceiverT,
    ) -> Result<Option<(&EntityT, &Source)>, Problem>
    where
        EntityT: 'static,
        ProblemReceiverT: ProblemReceiver,
    {
        self.completed_entity_checked(entity_kind, full_name, source_id, &mut Default::default(), problems)
    }

    /// Get an [Entity],
    /// calling [complete](Entity::complete) on it if
    /// [should_complete](Entity::should_complete) is true.
    ///
    /// If not found (e.g. it is currently removed for its completion phase) will return the
    /// fallback if it exists.
    ///
    /// The call is added to the derivation_path in order to detect circular dependencies.
    ///
    /// Note that the entity is removed from the catalog while it is being completed.
    pub fn completed_entity_checked<EntityT, ProblemReceiverT>(
        &mut self,
        entity_kind: EntityKind,
        full_name: &FullName,
        source_id: &SourceID,
        derivation_path: &mut DerivationPath,
        problems: &mut ProblemReceiverT,
    ) -> Result<Option<(&EntityT, &Source)>, Problem>
    where
        EntityT: 'static,
        ProblemReceiverT: ProblemReceiver,
    {
        Ok(match self.completed_entity_ref(entity_kind, full_name, source_id, derivation_path, problems)? {
            Some((entity, source)) => Some((give_unwrap!(entity.downcast_ref_checked(), problems), source)),
            None => None,
        })
    }
}
