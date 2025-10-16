use super::{
    super::{entity::*, errors::*, name::*, source::*, utils::*},
    catalog::*,
};

use {compris::annotate::*, kutil::std::error::*, std::any::*};

impl Catalog {
    /// All entity names.
    pub fn entity_names(&self) -> Vec<(SourceID, EntityKind, Name)> {
        let mut entity_names = Vec::default();
        for (source_id, source) in &self.sources {
            entity_names.extend(
                source.entity_names().into_iter().map(|(entity_kind, name)| (source_id.clone(), entity_kind, name)),
            );
        }
        entity_names
    }

    /// Add an entity reference.
    pub fn add_entity_ref<AnnotatedT>(
        &mut self,
        entity_kind: EntityKind,
        source_id: &SourceID,
        name: Name,
        entity: EntityRef,
    ) -> Result<(), ToscaError<AnnotatedT>>
    where
        AnnotatedT: Default,
    {
        Ok(self.get_source_mut(source_id)?.add_entity_ref(entity_kind, name, entity)?)
    }

    /// Get an entity reference.
    ///
    /// If not found (e.g. it is currently removed for its completion phase) will return the
    /// fallback if it exists.
    pub fn entity_ref<AnnotatedT>(
        &self,
        entity_kind: EntityKind,
        full_name: &FullName,
        source_id: &SourceID,
    ) -> Result<&EntityRef, ToscaError<AnnotatedT>>
    where
        AnnotatedT: Default,
    {
        let entity_kind_name = self.source_entity_kinds(source_id)?.represent(entity_kind);
        let source = self.lookup(entity_kind, &entity_kind_name, source_id, full_name)?;
        Ok(source.entity_ref(entity_kind, &entity_kind_name, &full_name.name)?)
    }

    /// Remove an entity and return its entity reference and source ID if it exists.
    pub fn remove_entity_ref<AnnotatedT>(
        &mut self,
        entity_kind: EntityKind,
        full_name: &FullName,
        source_id: &SourceID,
    ) -> Result<(EntityRef, SourceID), ToscaError<AnnotatedT>>
    where
        AnnotatedT: Default,
    {
        let entity_kind_name = self.source_entity_kinds(source_id)?.represent(entity_kind);
        let source = self.lookup_mut(entity_kind, &entity_kind_name, source_id, full_name)?;
        let entity = source.remove_entity_ref(entity_kind, &entity_kind_name, &full_name.name)?;
        Ok((entity, source.source_id.clone()))
    }

    /// Get an entity reference,
    /// calling [complete](Entity::complete) on it if
    /// [should_complete](Entity::should_complete) is true.
    ///
    /// Note that the entity is removed from the catalog while it is being completed.
    ///
    /// If not found (e.g. it is currently removed for its completion phase) will return the
    /// fallback if it exists.
    pub fn completed_entity_ref<AnnotatedT, ErrorRecipientT>(
        &mut self,
        entity_kind: EntityKind,
        full_name: &FullName,
        source_id: &SourceID,
        derivation_path: &mut DerivationPath,
        errors: &mut ErrorRecipientT,
    ) -> Result<Option<&EntityRef>, ToscaError<AnnotatedT>>
    where
        AnnotatedT: Annotated + Default,
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>,
    {
        let source = unwrap_or_give_and_return!(self.get_source(source_id), errors, Ok(None));

        let entity_kind_name =
            unwrap_or_give_and_return!(self.dialect_entity_kinds(&source.dialect_id), errors, Ok(None))
                .represent(entity_kind);

        let entity_source_id =
            unwrap_or_give_and_return!(source.lookup(entity_kind, &entity_kind_name, full_name), errors, Ok(None))
                .clone();

        let entity_source = unwrap_or_give_and_return!(self.get_source_mut(&entity_source_id), errors, Ok(None));

        // Fallback?

        if let Some(_) = unwrap_or_give_and_return!(
            entity_source.fallback_entity_ref(entity_kind, &entity_kind_name, &full_name.name),
            errors,
            Ok(None)
        ) {
            return self.entity_ref(entity_kind, full_name, source_id).map(Some);
        }

        // Remove entity

        let mut entity = unwrap_or_give_and_return!(
            entity_source.remove_entity_ref(entity_kind, &entity_kind_name, &full_name.name),
            errors,
            Ok(None)
        );

        // Update derivation path

        unwrap_or_give_and_return!(
            derivation_path.add(entity_source_id.clone(), full_name.name.clone()),
            errors,
            Ok(None)
        );

        // Complete

        let complete = {
            match self.complete_entity(
                &mut entity,
                &entity_kind_name,
                &full_name.name,
                &entity_source_id,
                derivation_path,
                errors,
            ) {
                Ok(complete) => complete,
                Err(error) => {
                    if self
                        .add_entity_ref::<WithoutAnnotations>(
                            entity_kind,
                            &entity_source_id,
                            full_name.name.clone(),
                            entity,
                        )
                        .is_err()
                    {
                        panic!("source {} disappeared", &entity_source_id);
                    }
                    return Err(error);
                }
            }
        };

        // Add entity back

        if self
            .add_entity_ref::<WithoutAnnotations>(entity_kind, &entity_source_id, full_name.name.clone(), entity)
            .is_err()
        {
            panic!("source {} disappeared", &entity_source_id);
        }

        Ok(if complete {
            // Get entity

            match self.entity_ref::<WithoutAnnotations>(entity_kind, full_name, source_id) {
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
    pub fn entity<EntityT, AnnotatedT>(
        &self,
        entity_kind: EntityKind,
        full_name: &FullName,
        source_id: &SourceID,
    ) -> Result<&EntityT, ToscaError<AnnotatedT>>
    where
        EntityT: 'static,
        AnnotatedT: Default,
    {
        let entity = self.entity_ref(entity_kind, full_name, source_id)?;
        Ok(entity.downcast_ref_or_error("entity", type_name::<EntityT>())?)
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
    pub fn completed_entity<EntityT, AnnotatedT, ErrorRecipientT>(
        &mut self,
        entity_kind: EntityKind,
        full_name: &FullName,
        source_id: &SourceID,
        errors: &mut ErrorRecipientT,
    ) -> Result<Option<&EntityT>, ToscaError<AnnotatedT>>
    where
        EntityT: 'static,
        AnnotatedT: Annotated + Default,
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>,
    {
        self.completed_entity_with_derivation_path(entity_kind, full_name, source_id, &mut Default::default(), errors)
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
    pub fn completed_entity_with_derivation_path<EntityT, AnnotatedT, ErrorRecipientT>(
        &mut self,
        entity_kind: EntityKind,
        full_name: &FullName,
        source_id: &SourceID,
        derivation_path: &mut DerivationPath,
        errors: &mut ErrorRecipientT,
    ) -> Result<Option<&EntityT>, ToscaError<AnnotatedT>>
    where
        EntityT: 'static,
        AnnotatedT: Annotated + Default,
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>,
    {
        Ok(match self.completed_entity_ref(entity_kind, full_name, source_id, derivation_path, errors)? {
            Some(entity) => match entity.downcast_ref_or_error("entity", type_name::<EntityT>()) {
                Ok(entity) => Some(entity),
                Err(error) => {
                    errors.give(error)?;
                    None
                }
            },
            None => None,
        })
    }
}
