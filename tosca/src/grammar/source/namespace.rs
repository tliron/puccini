use super::{
    super::{entity::*, errors::*, name::*},
    id::*,
    source::*,
};

use {kutil::std::collections::*, problemo::*, std::collections::*};

impl Source {
    /// Namespace.
    pub fn namespace(&self) -> impl Iterator<Item = (&EntityKind, &FullName, &SourceID)> {
        self.namespace.iter().map(|(key, source_id)| (&key.entity_kind, &key.inner, source_id))
    }

    /// Merge a [Source] into a namespace.
    pub fn merge_namespace<ProblemReceiverT>(
        &mut self,
        dependency: &Self,
        namespace: &Namespace,
        problems: &mut ProblemReceiverT,
    ) -> Result<(), Problem>
    where
        ProblemReceiverT: ProblemReceiver,
    {
        tracing::debug!(
            source = self.source_id.to_string(),
            from = dependency.source_id.to_string(),
            namespace = namespace.to_string(),
            "merging namespace"
        );

        for (entity_kind, full_name, source_id) in dependency.namespace() {
            give_unwrap!(
                self.map_name(
                    entity_kind.clone(),
                    full_name.clone().into_namespace(namespace.clone()),
                    source_id.clone()
                ),
                problems,
            );
        }

        Ok(())
    }

    /// Canonical full name for an entity.
    pub fn canonical_full_name_for(
        &self,
        entity_kind: EntityKind,
        entity_kind_name: &str,
        full_name: &FullName,
    ) -> Result<&FullName, Problem> {
        let source_id = self.lookup(entity_kind, entity_kind_name, full_name)?;
        self.canonical_full_name(entity_kind, &full_name.name, source_id)
            .ok_or_else(|| UndeclaredError::as_problem(entity_kind_name, full_name))
    }

    /// Canonical full name for an entity.
    pub fn canonical_full_name(&self, entity_kind: EntityKind, name: &Name, source_id: &SourceID) -> Option<&FullName> {
        let mut full_names = self.full_names(entity_kind, name, source_id);
        full_names.sort();
        full_names.into_iter().min_by(|x, y| x.namespace.segments.len().cmp(&y.namespace.segments.len()))
    }

    /// Full names for an entity.
    pub fn full_names(&self, entity_kind: EntityKind, name: &Name, source_id: &SourceID) -> Vec<&FullName> {
        self.namespace
            .iter()
            .filter(|(key, self_source_id)| {
                (key.entity_kind == entity_kind) && (key.inner.name == *name) && (*self_source_id == source_id)
            })
            .map(|(key, _)| &key.inner)
            .collect()
    }

    /// Canonical namespace.
    pub fn canonical_namespace(&self) -> Vec<(EntityKind, FullName, SourceID)> {
        let names: FastHashSet<_> =
            self.namespace.iter().map(|(key, source_id)| (key.entity_kind, &key.inner.name, source_id)).collect();

        let mut namespace = Vec::with_capacity(names.len());

        for (entity_kind, name, source_id) in names {
            if let Some(canonical) = self.canonical_full_name(entity_kind, name, source_id) {
                namespace.push((entity_kind, canonical.clone(), source_id.clone()));
            }
        }

        namespace
    }

    /// Namespace as a sorted tree.
    pub fn namespace_tree(&self) -> BTreeMap<EntityKind, BTreeMap<FullName, SourceID>> {
        let mut namespace = BTreeMap::<EntityKind, BTreeMap<FullName, SourceID>>::default();

        for (entity_kind, full_name, source_id) in self.namespace() {
            match namespace.get_mut(entity_kind) {
                Some(names) => {
                    names.insert(full_name.clone(), source_id.clone());
                }

                None => {
                    let mut names = BTreeMap::default();
                    names.insert(full_name.clone(), source_id.clone());
                    namespace.insert(entity_kind.clone(), names);
                }
            }
        }

        namespace
    }

    /// Map a [FullName] to a [SourceID].
    pub fn map_name(
        &mut self,
        entity_kind: EntityKind,
        full_name: FullName,
        source_id: SourceID,
    ) -> Result<(), Problem> {
        tracing::trace!(source = self.source_id.to_string(), "map full name to source: {} -> {}", full_name, source_id);

        let key = WithEntityKind::new(entity_kind, full_name.clone());

        if let Some(existing) = self.namespace.get(&key)
            && (*existing != source_id)
        {
            return Err(NameReusedError::as_problem(full_name));
        }

        self.namespace.insert(key, source_id);

        Ok(())
    }

    /// Find the [SourceID] of a [FullName].
    pub fn try_lookup(&self, entity_kind: EntityKind, full_name: &FullName) -> Option<&SourceID> {
        self.namespace.get(&WithEntityKind::new(entity_kind, full_name.clone()))
    }

    /// Find the [SourceID] of a [FullName].
    pub fn lookup(
        &self,
        entity_kind: EntityKind,
        entity_kind_name: &str,
        full_name: &FullName,
    ) -> Result<&SourceID, Problem> {
        self.try_lookup(entity_kind, full_name).ok_or_else(|| UndeclaredError::as_problem(entity_kind_name, full_name))
    }
}
