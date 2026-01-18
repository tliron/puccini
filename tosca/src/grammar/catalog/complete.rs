use super::{
    super::{complete::*, entity::*, name::*, source::*},
    catalog::*,
};

use {kutil::std::immutable::*, problemo::*};

impl Catalog {
    /// Whether all entities are complete.
    pub fn are_entities_complete(&self) -> bool {
        self.sources.values().all(|source| source.are_entities_complete())
    }

    /// Complete all entities.
    ///
    /// Calls [complete](super::super::entity::Entity::complete) on entities for which
    /// [should_complete](super::super::entity::Entity::should_complete) is true.
    ///
    /// Note that each entity is removed from the catalog while it is being completed.
    pub fn complete_entities<ProblemReceiverT>(&mut self, problems: &mut ProblemReceiverT) -> Result<(), Problem>
    where
        ProblemReceiverT: ProblemReceiver,
    {
        let mut entity_names: Vec<_> = self
            .entity_names()
            .map(|(source_id, entity_kind, name)| (source_id.clone(), entity_kind.clone(), name.clone()))
            .collect();

        entity_names.sort();

        for (source_id, entity_kind, name) in entity_names {
            let entity_kind_name = match self.source_entity_kinds(&source_id).give_ok(problems)? {
                Some(entity_kinds) => entity_kinds.represent(entity_kind),
                None => EntityKinds::default().represent(entity_kind),
            };

            match self.sources.get_mut(&source_id) {
                Some(source) => match source.remove_entity_ref(entity_kind, &entity_kind_name, &name) {
                    Ok(mut entity) => {
                        self.complete_entity(
                            &mut entity,
                            &entity_kind_name,
                            &name,
                            &source_id,
                            &mut DerivationPath::new(source_id.clone(), name.clone()),
                            problems,
                        )?;

                        match self.sources.get_mut(&source_id) {
                            Some(source) => {
                                give_unwrap!(source.add_entity_ref(entity_kind, name, entity), problems)
                            }

                            None => panic!("source {} disappeared", source_id),
                        }
                    }

                    Err(_) => {
                        panic!("{} {} disappeared from {}", entity_kind_name, name, source_id);
                    }
                },

                None => {
                    panic!("source {} disappeared", source_id);
                }
            }
        }

        Ok(())
    }

    pub(crate) fn complete_entity<ProblemReceiverT>(
        &mut self,
        entity: &mut EntityRef,
        entity_kind_name: &ByteString,
        name: &Name,
        source_id: &SourceID,
        derivation_path: &mut DerivationPath,
        problems: &mut ProblemReceiverT,
    ) -> Result<bool, Problem>
    where
        ProblemReceiverT: ProblemReceiver,
    {
        Ok(if entity.should_complete() {
            tracing::debug!(
                source = source_id.to_string(),
                kind = entity_kind_name.to_string(),
                name = name.to_string(),
                "completing",
            );

            entity.complete(derivation_path, &mut CompletionContext::new(self, source_id, problems.as_ref()))?;

            if entity.is_complete() {
                tracing::debug!(
                    source = source_id.to_string(),
                    kind = entity_kind_name.to_string(),
                    name = name.to_string(),
                    "completed",
                );
                true
            } else {
                tracing::error!(
                    source = source_id.to_string(),
                    kind = entity_kind_name.to_string(),
                    name = name.to_string(),
                    "could not complete",
                );
                false
            }
        } else {
            tracing::trace!(
                source = source_id.to_string(),
                kind = entity_kind_name.to_string(),
                name = name.to_string(),
                "should not complete",
            );
            entity.is_complete()
        })
    }
}
