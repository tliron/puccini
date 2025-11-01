use super::{
    super::{dialect::*, entity::*, errors::*},
    catalog::*,
};

use std::any::*;

impl Catalog {
    /// Add a dialect reference.
    pub fn add_dialect_ref(&mut self, dialect: DialectRef) {
        self.dialects.insert(dialect.dialect_id(), dialect);
    }

    /// Get a dialect reference.
    pub fn get_dialect_ref<AnnotatedT>(
        &self,
        dialect_id: &DialectID,
    ) -> Result<&DialectRef, UnsupportedDialectError<AnnotatedT>>
    where
        AnnotatedT: Default,
    {
        self.dialects.get(dialect_id).ok_or_else(|| UnsupportedDialectError::new(dialect_id.clone()))
    }

    /// Get a [Dialect].
    pub fn get_dialect<DialectT, AnnotatedT>(&self, dialect_id: &DialectID) -> Result<&DialectT, ToscaError<AnnotatedT>>
    where
        DialectT: 'static,
        AnnotatedT: Default,
    {
        let dialect = self.get_dialect_ref(dialect_id)?;
        Ok(dialect.into_any_ref_checked("dialect", type_name::<DialectT>())?)
    }

    /// Supported entity kinds.
    pub fn dialect_entity_kinds<AnnotatedT>(
        &self,
        dialect_id: &DialectID,
    ) -> Result<&EntityKinds, UnsupportedDialectError<AnnotatedT>>
    where
        AnnotatedT: Default,
    {
        Ok(self.get_dialect_ref(dialect_id)?.entity_kinds())
    }
}
