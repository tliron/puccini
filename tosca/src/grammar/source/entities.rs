use super::{
    super::{entity::*, errors::*, name::*, utils::*},
    source::*,
};

use {kutil::std::immutable::*, std::any::*, std::collections::*};

impl Source {
    /// Entity names.
    pub fn entity_names(&self) -> Vec<(EntityKind, Name)> {
        self.entities.keys().map(|key| (key.entity_kind, key.inner.clone())).collect()
    }

    /// Entitiy names as a sorted tree.
    pub fn entity_names_tree(&self) -> BTreeMap<EntityKind, Vec<Name>> {
        let mut entity_names_tree = BTreeMap::<EntityKind, Vec<Name>>::default();

        for key in self.entities.keys() {
            match entity_names_tree.get_mut(&key.entity_kind) {
                Some(names) => {
                    names.push(key.inner.clone());
                    names.sort();
                }

                None => {
                    let names = vec![key.inner.clone()];
                    entity_names_tree.insert(key.entity_kind, names);
                }
            }
        }

        entity_names_tree
    }

    /// Whether we have this entity.
    pub fn has_entity(&self, entity_kind: EntityKind, full_name: &FullName) -> bool {
        full_name.scope.is_empty()
            && self.entities.contains_key(&WithEntityKind::new(entity_kind, full_name.name.clone()))
    }

    /// Whether all entities are complete.
    pub fn are_entities_complete(&self) -> bool {
        self.entities.values().all(|entity| entity.is_complete())
    }

    /// Add an entity reference.
    pub fn add_entity_ref<AnnotatedT>(
        &mut self,
        entity_kind: EntityKind,
        name: Name,
        entity: EntityRef,
    ) -> Result<(), NameReusedError<AnnotatedT>>
    where
        AnnotatedT: Default,
    {
        tracing::trace!(source = self.source_id.to_string(), "adding entity: {}", name);
        match self.entities.insert(WithEntityKind::new(entity_kind, name.clone()), entity) {
            Some(_) => Err(NameReusedError::new(name.to_string())),
            None => {
                self.namespace.insert(WithEntityKind::new(entity_kind, name.into()), self.source_id.clone());
                Ok(())
            }
        }
    }

    /// Add an [Entity].
    pub fn add_entity<EntityT, AnnotatedT>(
        &mut self,
        entity_kind: EntityKind,
        name: Name,
        entity: EntityT,
        fallback: bool,
    ) -> Result<(), NameReusedError<AnnotatedT>>
    where
        EntityT: 'static + Clone + Entity,
        AnnotatedT: Default,
    {
        if fallback {
            self.add_fallback_entity_ref(entity_kind, name.clone(), entity.clone().into())?;
        }
        self.add_entity_ref(entity_kind, name.clone(), entity.into())
    }

    /// Get an entity reference.
    ///
    /// If not found (e.g. it is currently removed for its completion phase) will return the
    /// fallback if it exists.
    pub fn entity_ref<AnnotatedT>(
        &self,
        entity_kind: EntityKind,
        entity_kind_name: &ByteString,
        name: &Name,
    ) -> Result<&EntityRef, UndeclaredError<AnnotatedT>>
    where
        AnnotatedT: Default,
    {
        let key = WithEntityKind::new(entity_kind, name.clone());
        self.entities
            .get(&key)
            .or_else(|| self.fallback_entities.get(&key))
            .ok_or_else(|| UndeclaredError::new(entity_kind_name.to_string(), name.to_string()))
    }

    /// Remove an entity and return its reference if it exists.
    ///
    /// Note that this will *not* remove the entity's name from the namespace.
    pub fn remove_entity_ref<AnnotatedT>(
        &mut self,
        entity_kind: EntityKind,
        entity_kind_name: &ByteString,
        name: &Name,
    ) -> Result<EntityRef, UndeclaredError<AnnotatedT>>
    where
        AnnotatedT: Default,
    {
        tracing::trace!(
            source = self.source_id.to_string(),
            kind = entity_kind_name.to_string(),
            "removing entity: {}",
            name
        );

        self.entities
            .remove(&WithEntityKind::new(entity_kind, name.clone()))
            .ok_or_else(|| UndeclaredError::new(entity_kind_name.to_string(), name.to_string()))
    }

    /// Add a fallback entity reference.
    pub fn add_fallback_entity_ref<AnnotatedT>(
        &mut self,
        entity_kind: EntityKind,
        name: Name,
        entity: EntityRef,
    ) -> Result<(), NameReusedError<AnnotatedT>>
    where
        AnnotatedT: Default,
    {
        tracing::trace!(source = self.source_id.to_string(), "adding fallback entity: {}", name);
        match self.fallback_entities.insert(WithEntityKind::new(entity_kind, name.clone()), entity) {
            Some(_) => Err(NameReusedError::new(name.to_string())),
            None => Ok(()),
        }
    }

    /// Get a fallback entity reference *only* if we don't have the entity reference.
    pub fn fallback_entity_ref<AnnotatedT>(
        &self,
        entity_kind: EntityKind,
        entity_kind_name: &ByteString,
        name: &Name,
    ) -> Result<Option<&EntityRef>, UndeclaredError<AnnotatedT>>
    where
        AnnotatedT: Default,
    {
        let key = WithEntityKind::new(entity_kind, name.clone());
        if !self.entities.contains_key(&key) {
            self.fallback_entities
                .get(&key)
                .map(Some)
                .ok_or_else(|| UndeclaredError::new(entity_kind_name.to_string(), name.to_string()))
        } else {
            Ok(None)
        }
    }

    /// Get an [Entity].
    ///
    /// If not found (e.g. it is currently removed for its completion phase) will return the
    /// fallback if it exists.
    pub fn entity<EntityT, AnnotatedT>(
        &self,
        entity_kind: EntityKind,
        entity_kind_name: &ByteString,
        name: &Name,
    ) -> Result<&EntityT, ToscaError<AnnotatedT>>
    where
        EntityT: 'static,
        AnnotatedT: Default,
    {
        let entity = self.entity_ref(entity_kind, entity_kind_name, name)?;
        Ok(entity.downcast_ref_or_error("entity", type_name::<EntityT>())?)
    }
}
